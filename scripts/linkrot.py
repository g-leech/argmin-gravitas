#!/usr/bin/env python3
"""
Link rot protection for the site: collect every outbound link in the source,
make sure the Wayback Machine holds a copy, and periodically check which links
have died. Dead links can then be pointed at their Wayback snapshot.

Stdlib only. State lives in <repo>/.linkrot.json (gitignored); it is a cache
of what the Wayback Machine already knows plus recent check history, so
deleting it only costs a slow re-run.

Usage:
    python scripts/linkrot.py scan                # collect outbound links
    python scripts/linkrot.py archive             # find or request a snapshot for each
    python scripts/linkrot.py archive --lookup-only --limit 50
    python scripts/linkrot.py check               # fetch every link, track dead ones
    python scripts/linkrot.py report              # summary + dead links and where they're cited
    python scripts/linkrot.py fix [--dry-run]     # point dead links at their snapshot
    python scripts/linkrot.py run                 # scan + check + notify + archive (the timer)

A link only counts as dead after 3 consecutive checks (on different days) that
return 404/410/451, a nonexistent domain, or a redirect from a deep link to a
front page. Anything else (403, 429, 5xx, timeouts, TLS errors) is transient.

Snapshots are found via the CDX API, preferring the capture closest to when
the link was cited (the post's date). Links with no capture are submitted to
Save Page Now, at most once a month each; the next run picks up the capture.

Weekly timer: ~/.config/systemd/user/linkrot.{service,timer}
    systemctl --user enable --now linkrot.timer
"""

import argparse
import html
import json
import random
import re
import socket
import subprocess
import time
import urllib.error
import urllib.parse
import urllib.request
from concurrent.futures import ThreadPoolExecutor
from datetime import date, timedelta
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
STATE = REPO / ".linkrot.json"

SOURCE_DIRS = ["_posts", "pages", "_includes", "_layouts"]
SOURCE_FILES = ["index.html", "404.html"]
EXTENSIONS = {".md", ".markdown", ".html"}

SKIP_HOSTS = {
    # mine
    "gleech.org", "g-leech.github.io",
    # already archives
    "archive.org", "archive.is", "archive.ph", "archive.today", "archive.li",
    "archive.md", "webcitation.org", "ghostarchive.org",
    # not real
    "localhost", "127.0.0.1", "example.com", "example.org",
}

UA = "Mozilla/5.0 (compatible; linkrot/1.0; +https://www.gleech.org)"
DEAD_AFTER = 3
CHECK_HISTORY = 6
RESUBMIT_AFTER = timedelta(days=30)
CDX_DELAY = 1
SAVE_DELAY = 10
WORKERS = 16

URL_RE = re.compile(r"https?://[^\s<>\"`{}|^\[\]“”]+")
HOST_RE = re.compile(r"[\w-]+(\.[\w-]+)+")
POST_DATE_RE = re.compile(r"_posts/(\d{4})-(\d\d)-(\d\d)-")
WAYBACK_RE = re.compile(r"https://web\.archive\.org/web/\d{14}/")
TRAILING = ".,;:!?*'’"


class RateLimited(Exception):
    pass


# --- links ---------------------------------------------------------------

def clean(text, m):
    url = m.group()
    # an unmatched ")" ends the URL: markdown [x](url)_, prose (see url)'s.
    # balanced ones stay, as in wikipedia's Foo_(bar)
    depth = 0
    for i, ch in enumerate(url):
        if ch == "(":
            depth += 1
        elif ch == ")":
            if not depth:
                url = url[:i]
                break
            depth -= 1
    url = url.rstrip("\\")
    # inside (...) or "..." the delimiter marks the end, so a final "." is part of the URL
    end = m.start() + len(url)
    after = text[end:end + 2].lstrip("\\")[:1]
    if (text[m.start() - 1:m.start()], after) in {("(", ")"), ('"', '"'), ("<", ">")}:
        return url
    return url.rstrip(TRAILING)


def host_of(url):
    try:
        return (urllib.parse.urlsplit(url).hostname or "").lower()
    except ValueError:
        return ""


def wanted(url):
    host = host_of(url)
    if not HOST_RE.fullmatch(host):
        return False
    return not any(host == h or host.endswith("." + h) for h in SKIP_HOSTS)


def source_files():
    for d in SOURCE_DIRS:
        for p in sorted((REPO / d).rglob("*")):
            if p.is_file() and p.suffix in EXTENSIONS:
                yield p
    for name in SOURCE_FILES:
        if (REPO / name).exists():
            yield REPO / name


def to_ascii(url):
    """What we actually request: entities decoded, no fragment, percent-encoded."""
    unescaped = re.sub(r"\\(.)", r"\1", html.unescape(url))  # markdown's \_
    parts = urllib.parse.urlsplit(unescaped)
    netloc = parts.hostname.encode("idna").decode()
    if parts.port:
        netloc += f":{parts.port}"
    path = urllib.parse.quote(parts.path, safe="/%:@!$&'()*+,;=~")
    query = urllib.parse.quote(parts.query, safe="/%:@!$&'()*+,;=~?")
    return urllib.parse.urlunsplit((parts.scheme, netloc, path, query, ""))


def get(url, timeout):
    req = urllib.request.Request(url, headers={"User-Agent": UA, "Accept": "*/*"})
    return urllib.request.urlopen(req, timeout=timeout)


def cited_on(entry):
    """YYYYMMDD of the earliest post citing this link, else when we first saw it."""
    dates = ["".join(m.groups()) for s in entry["sources"] if (m := POST_DATE_RE.match(s))]
    return min(dates) if dates else entry["first_seen"].replace("-", "")


# --- state ---------------------------------------------------------------

def load():
    return json.loads(STATE.read_text()) if STATE.exists() else {}


def save(state):
    tmp = STATE.with_suffix(".tmp")
    tmp.write_text(json.dumps(state, indent=1, sort_keys=True, ensure_ascii=False))
    tmp.replace(STATE)


# --- commands ------------------------------------------------------------

def scan(state):
    found = {}
    files = list(source_files())
    for p in files:
        rel = str(p.relative_to(REPO))
        text = p.read_text(errors="replace")
        for m in URL_RE.finditer(text):
            url = clean(text, m)
            if wanted(url):
                found.setdefault(url, set()).add(rel)

    today = date.today().isoformat()
    new = 0
    for url, cited in found.items():
        if url not in state:
            new += 1
            state[url] = {"first_seen": today, "snapshot": None, "save_requested": None,
                          "checks": [], "dead": False}
        state[url]["sources"] = sorted(cited)
    gone = [u for u in state if u not in found]
    for u in gone:
        del state[u]
    print(f"scan: {len(found)} outbound links in {len(files)} files "
          f"({new} new, {len(gone)} no longer cited)")


def cdx_lookup(url, closest):
    q = urllib.parse.urlencode({
        "url": to_ascii(url), "filter": "statuscode:200", "sort": "closest",
        "closest": closest, "limit": 1, "fl": "timestamp,original",
    })
    try:
        with get(f"https://web.archive.org/cdx/search/cdx?{q}", timeout=60) as r:
            fields = r.read().decode(errors="replace").split()
    except urllib.error.HTTPError as e:
        if e.code == 429:
            raise RateLimited("CDX") from e
        raise
    if len(fields) >= 2:
        return f"https://web.archive.org/web/{fields[0]}/{fields[1]}"
    return None


def request_save(url):
    """Ask Save Page Now for a capture. Returns the snapshot URL if it says so."""
    try:
        with get(f"https://web.archive.org/save/{to_ascii(url)}", timeout=180) as r:
            final = r.geturl()
            location = r.headers.get("Content-Location", "")
    except urllib.error.HTTPError as e:
        if e.code == 429:
            raise RateLimited("Save Page Now") from e
        raise
    for candidate in (final, "https://web.archive.org" + location):
        if WAYBACK_RE.match(candidate):
            return candidate
    return None


def with_backoff(fn, *args):
    """One retry after a minute on 429; a second 429 ends the phase."""
    try:
        return fn(*args)
    except RateLimited:
        time.sleep(60)
        return fn(*args)


def archive(state, lookup_only=False, limit=None):
    today = date.today()
    todo = [u for u, e in state.items() if not e["snapshot"]]
    if limit:
        todo = todo[:limit]
    print(f"archive: {len(todo)} links without a known snapshot")

    found = requested = failed = 0
    try:
        for i, url in enumerate(todo, 1):
            entry = state[url]
            try:
                snap = with_backoff(cdx_lookup, url, cited_on(entry))
                time.sleep(CDX_DELAY)
                if snap:
                    entry["snapshot"] = snap
                    found += 1
                    continue
                last = entry["save_requested"]
                if lookup_only or (last and today - date.fromisoformat(last) < RESUBMIT_AFTER):
                    continue
                entry["save_requested"] = today.isoformat()
                entry["snapshot"] = with_backoff(request_save, url)
                requested += 1
                time.sleep(SAVE_DELAY)
            except RateLimited as e:
                print(f"archive: rate-limited by {e} twice, stopping; next run resumes")
                break
            except Exception as e:
                failed += 1
                print(f"  ! {url}: {type(e).__name__}: {e}")
            if i % 25 == 0:
                save(state)
                print(f"  {i}/{len(todo)}")
    finally:
        save(state)
    print(f"archive: {found} snapshots found, {requested} saves requested, {failed} errors")


def probe(url):
    """(result, detail) where result is ok / dead / error / unasked."""
    try:
        with get(to_ascii(url), timeout=30) as r:
            asked, landed = urllib.parse.urlsplit(url), urllib.parse.urlsplit(r.geturl())
            deep = asked.path not in ("", "/") or asked.query
            front = landed.path in ("", "/") and not landed.query
            if deep and front:
                return "dead", f"redirects to {landed.netloc}/"
            return "ok", str(r.status)
    except urllib.error.HTTPError as e:
        return ("dead" if e.code in (404, 410, 451) else "error"), str(e.code)
    except urllib.error.URLError as e:
        nxdomain = {socket.EAI_NONAME, getattr(socket, "EAI_NODATA", None)}
        if isinstance(e.reason, socket.gaierror) and e.reason.errno in nxdomain:
            return "dead", "no such domain"
        if isinstance(e.reason, socket.gaierror) and e.reason.errno == socket.EAI_AGAIN:
            # usually our network, not theirs (e.g. the laptop suspended mid-run)
            return "unasked", str(e.reason)
        return "error", str(e.reason)[:120]
    except Exception as e:
        return "error", f"{type(e).__name__}: {e}"[:120]


def check(state):
    """Returns the links that became dead on this run."""
    result, detail = probe("https://web.archive.org/")
    if result != "ok":
        print(f"check: can't reach the internet ({detail}), not checking")
        return []

    urls = list(state)
    random.shuffle(urls)  # spread requests across hosts
    with ThreadPoolExecutor(WORKERS) as pool:
        results = dict(zip(urls, pool.map(probe, urls)))

    today = date.today().isoformat()
    newly_dead = []
    for url, (result, detail) in results.items():
        if result == "unasked":
            continue
        entry = state[url]
        checks = [c for c in entry["checks"] if c["date"] != today]
        entry["checks"] = (checks + [{"date": today, "result": result, "detail": detail}])[-CHECK_HISTORY:]
        recent = entry["checks"][-DEAD_AFTER:]
        if result == "ok":
            entry["dead"] = False
        elif len(recent) == DEAD_AFTER and all(c["result"] == "dead" for c in recent):
            if not entry["dead"]:
                newly_dead.append(url)
            entry["dead"] = True
    save(state)

    tally = {r: sum(1 for res, _ in results.values() if res == r) for r in ("ok", "dead", "error", "unasked")}
    print(f"check: {tally['ok']} ok, {tally['dead']} dead today, {tally['error']} transient errors, "
          f"{tally['unasked']} unresolvable (not recorded); {len(newly_dead)} newly confirmed dead")
    return newly_dead


def report(state):
    n = len(state)
    snapped = sum(1 for e in state.values() if e["snapshot"])
    pending = sum(1 for e in state.values() if not e["snapshot"] and e["save_requested"])
    suspect = sum(1 for e in state.values() if not e["dead"] and e["checks"]
                  and e["checks"][-1]["result"] == "dead")
    dead = {u: e for u, e in state.items() if e["dead"]}
    print(f"{n} links · {snapped} with a snapshot · {pending} saves pending · "
          f"{n - snapped - pending} unarchived")
    print(f"{len(dead)} confirmed dead · {suspect} failing but unconfirmed")
    for url, e in sorted(dead.items()):
        print(f"\n{url}\n  {e['checks'][-1]['detail']}")
        print(f"  snapshot: {e['snapshot'] or 'NONE'}")
        for s in e["sources"]:
            print(f"  cited in: {s}")


def fix(state, dry_run=False):
    edits = {}
    for url, e in state.items():
        if not (e["dead"] and e["snapshot"]):
            continue
        # whole-URL matches only: not a prefix of a longer URL, not already inside a snapshot URL
        pattern = re.compile(
            r"(?:(?<=[(\s\"'<=\[])|^)" + re.escape(url) + r"(?=[" + re.escape(TRAILING) + r"]*(?:[)\s\"'<>\]]|$))",
            re.M,
        )
        for s in e["sources"]:
            edits.setdefault(s, []).append((pattern, e["snapshot"], url))

    total = changed = 0
    for rel, subs in sorted(edits.items()):
        path = REPO / rel
        original = text = path.read_text()
        for pattern, snapshot, url in subs:
            text, n = pattern.subn(snapshot, text)
            total += n
            if n:
                print(f"{rel}: {url}\n  -> {snapshot}")
        if text != original:
            changed += 1
            if not dry_run:
                path.write_text(text)
    verb = "would replace" if dry_run else "replaced"
    print(f"fix: {verb} {total} dead links across {changed} files"
          + ("" if dry_run else "; review with git diff"))


def notify(newly_dead):
    if not newly_dead:
        return
    msg = f"{len(newly_dead)} links newly dead. python scripts/linkrot.py report"
    try:
        subprocess.run(["notify-send", "linkrot", msg], check=False)
    except FileNotFoundError:
        pass


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    sub = parser.add_subparsers(dest="cmd", required=True)
    sub.add_parser("scan")
    a = sub.add_parser("archive")
    a.add_argument("--lookup-only", action="store_true", help="find existing snapshots, request none")
    a.add_argument("--limit", type=int)
    sub.add_parser("check")
    sub.add_parser("report")
    f = sub.add_parser("fix")
    f.add_argument("--dry-run", action="store_true")
    sub.add_parser("run")
    args = parser.parse_args()

    state = load()
    if args.cmd == "scan":
        scan(state)
        save(state)
    elif args.cmd == "archive":
        archive(state, args.lookup_only, args.limit)
    elif args.cmd == "check":
        check(state)
    elif args.cmd == "report":
        report(state)
    elif args.cmd == "fix":
        fix(state, args.dry_run)
    elif args.cmd == "run":
        # check before archive: the first archive pass takes hours
        scan(state)
        save(state)
        notify(check(state))
        archive(state)


if __name__ == "__main__":
    main()

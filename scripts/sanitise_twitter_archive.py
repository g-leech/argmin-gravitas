#!/usr/bin/env python3
"""
Sanitise a Twitter/X archive for sharing: empty all sensitive data files
but keep them present (as empty arrays) so the offline viewer still loads.

Keeps intact: tweets, tweet media, community tweets, the web app itself.
Empties: DMs, likes, followers, blocks, mutes, IP logs, ad data, etc.

Usage:
    python sanitise_twitter_archive.py twitter-archive.zip [-o output.zip]
    python sanitise_twitter_archive.py twitter-archive/ [-o output/]
"""

import argparse
import json
import re
import shutil
from pathlib import Path
from zipfile import ZipFile, ZIP_DEFLATED

# JS data files whose array contents we KEEP. Everything else → [].
KEEP_DATA = {
    "tweets.js",
    "tweet.js",
    "community-tweet.js",
    "communitytweet.js",
    "profile.js",
    "account.js",
}

# Fields to strip from account.js objects.
ACCOUNT_STRIP_FIELDS = {
    "email", "phone", "phoneNumber", "createdVia",
    "timezone", "createdAt",
}
KEEP_DATA_PATTERNS = [
    re.compile(r"^tweets-part\d+\.js$", re.IGNORECASE),
    re.compile(r"^community-tweet-part\d+\.js$", re.IGNORECASE),
]

# Media directories to keep verbatim.
KEEP_MEDIA_DIRS = {
    "tweets_media",
    "tweet_media",
    "community_tweet_media",
    "community-tweet-media",
    "profile_media",
}

# Sensitive media directories to DROP entirely.
# (DM media, moments media, etc. — no point keeping empty dirs of images.)
DROP_MEDIA_DIRS = {
    "direct_messages_media",
    "direct_message_media",
    "dm_media",
    "moments_media",
    "moments_tweets_media",
}

# Fields to strip from kept tweet objects.
TWEET_STRIP_FIELDS = {"ip", "deviceToken", "carrier"}

# Sidebar labels to blank out in the viewer's JS bundle.
SIDEBAR_LABELS = [
    "Account", "Likes", "Direct Messages", "Safety",
    "Personalization", "Ads", "Lists", "Moments",
]

JS_PREFIX_RE = re.compile(r"^(window\.YTD\.\w+\.part\d+\s*=\s*)")


def patch_asset_js(raw: bytes) -> bytes:
    """Blank out sidebar label strings in the viewer's JS bundle."""
    text = raw.decode("utf-8")
    for label in SIDEBAR_LABELS:
        # Match both "Label" and 'Label' in minified JS
        text = text.replace(f'"{label}"', '""')
        text = text.replace(f"'{label}'", "''")
    return text.encode("utf-8")

def patch_html(raw: str) -> str:
    """No-op now — sidebar is handled by blanking labels in the JS bundle."""
    return raw


def parse_twitter_js(text: str) -> tuple[str, list]:
    match = JS_PREFIX_RE.match(text)
    prefix = match.group(1) if match else "window.YTD.unknown.part0 = "
    payload = text[match.end():] if match else text
    return prefix, json.loads(payload)


def serialise_twitter_js(prefix: str, data: list) -> str:
    return prefix + json.dumps(data, ensure_ascii=False, indent=2)


def scrub_tweet(tw: dict) -> dict:
    tweet = tw.get("tweet", tw)
    for f in TWEET_STRIP_FIELDS:
        tweet.pop(f, None)
    tweet.pop("coordinates", None)
    tweet.pop("geo", None)
    if "place" in tweet and tweet["place"]:
        p = tweet["place"]
        tweet["place"] = {"full_name": p.get("full_name"), "country": p.get("country")}
    return tw


def is_kept_data(filename: str) -> bool:
    lo = filename.lower()
    if lo in KEEP_DATA:
        return True
    return any(p.match(lo) for p in KEEP_DATA_PATTERNS)


def in_dir_set(rel_path: str, dir_set: set) -> bool:
    parts = rel_path.replace("\\", "/").lower().split("/")
    return any(p in dir_set for p in parts)


def process_js(raw: str, filename: str) -> str:
    try:
        prefix, data = parse_twitter_js(raw)
    except (json.JSONDecodeError, ValueError):
        # Not a data file (manifest.js, or empty/malformed) — pass through as-is.
        return raw
    if not isinstance(data, list):
        return raw
    if is_kept_data(filename):
        lo = filename.lower()
        if lo == "account.js":
            data = [scrub_account(item) for item in data]
        elif lo != "profile.js":
            data = [scrub_tweet(t) for t in data]
        return serialise_twitter_js(prefix, data)
    return serialise_twitter_js(prefix, [])


def scrub_account(wrapper: dict) -> dict:
    """Strip sensitive fields from account.js, keep display info."""
    acct = wrapper.get("account", wrapper)
    for f in ACCOUNT_STRIP_FIELDS:
        acct.pop(f, None)
    return wrapper


def normalise_rel(path: str) -> str:
    """Strip a possible top-level nesting folder."""
    normed = path.replace("\\", "/").lstrip("./")
    if normed.startswith("data/") or not "/" in normed:
        return normed
    parts = normed.split("/", 1)
    if len(parts) > 1:
        return parts[1]
    return normed


def sanitise_zip(src: Path, dst: Path):
    emptied, kept_files = [], []

    with ZipFile(src, "r") as zin, ZipFile(dst, "w", ZIP_DEFLATED) as zout:
        for info in zin.infolist():
            if info.is_dir():
                continue

            rel = normalise_rel(info.filename)

            # Drop sensitive media directories entirely
            if in_dir_set(rel, DROP_MEDIA_DIRS):
                continue

            raw = zin.read(info)

            if rel.startswith("data/") and rel.endswith(".js"):
                filename = rel.rsplit("/", 1)[-1]
                text = raw.decode("utf-8")
                processed = process_js(text, filename)
                zout.writestr(info, processed.encode("utf-8"))
                (kept_files if is_kept_data(filename) else emptied).append(filename)

            elif in_dir_set(rel, KEEP_MEDIA_DIRS):
                zout.writestr(info, raw)
                kept_files.append(rel.rsplit("/", 1)[-1])

            elif rel.endswith(".html"):
                text = raw.decode("utf-8")
                zout.writestr(info, patch_html(text).encode("utf-8"))

            elif "assets/" in rel.lower():
                # Web viewer app — patch JS bundles to blank sidebar labels
                if rel.endswith(".js"):
                    raw = patch_asset_js(raw)
                zout.writestr(info, raw)

            elif rel.count("/") <= 1:
                # Top-level files (manifest.js, etc.)
                zout.writestr(info, raw)

            # else: orphaned media for emptied data — skip

    print(f"Kept intact: {', '.join(sorted(set(f for f in kept_files if f.endswith('.js'))))}")
    print(f"Emptied to []: {', '.join(sorted(set(emptied)))}")
    print(f"→ {dst}  ({dst.stat().st_size / 1_048_576:.1f} MB)")


def sanitise_dir(src: Path, dst: Path):
    root = src
    if not (src / "data").is_dir():
        for sd in src.iterdir():
            if sd.is_dir() and (sd / "data").is_dir():
                root = sd
                break
    if not (root / "data").is_dir():
        raise FileNotFoundError(f"No 'data/' dir in {src}")

    dst.mkdir(parents=True, exist_ok=True)
    emptied = []

    for fpath in sorted(root.rglob("*")):
        if fpath.is_dir():
            continue
        rel = str(fpath.relative_to(root))

        if in_dir_set(rel, DROP_MEDIA_DIRS):
            continue

        out = dst / rel
        out.parent.mkdir(parents=True, exist_ok=True)

        if rel.startswith("data/") and rel.endswith(".js"):
            text = fpath.read_text("utf-8")
            out.write_text(process_js(text, fpath.name), "utf-8")
            if not is_kept_data(fpath.name):
                emptied.append(fpath.name)
        elif rel.endswith(".html"):
            text = fpath.read_text("utf-8")
            out.write_text(patch_html(text), "utf-8")
        elif "assets/" in rel.lower() and rel.endswith(".js"):
            raw = fpath.read_bytes()
            out.write_bytes(patch_asset_js(raw))
        elif in_dir_set(rel, KEEP_MEDIA_DIRS) or "assets/" in rel.lower() or rel.count("/") == 0:
            shutil.copy2(fpath, out)

    print(f"Emptied: {', '.join(sorted(set(emptied)))}")
    print(f"→ {dst}")


def main():
    ap = argparse.ArgumentParser(description="Sanitise a Twitter/X archive.")
    ap.add_argument("source", type=Path)
    ap.add_argument("-o", "--output", type=Path, default=None)
    args = ap.parse_args()

    src = args.source.resolve()
    if src.is_file() and src.suffix == ".zip":
        sanitise_zip(src, args.output or src.with_name(src.stem + "_sanitised.zip"))
    elif src.is_dir():
        sanitise_dir(src, args.output or src.with_name(src.name + "_sanitised"))
    else:
        ap.error(f"Need a .zip or directory: {src}")


if __name__ == "__main__":
    main()

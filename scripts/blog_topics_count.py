#!/usr/bin/env python3
"""
Analyse a Jekyll blog (gleech.org / argmin-gravitas) by tag.

Reads frontmatter from _posts/*.md and *.markdown, extracts categories,
wordcount, date, importance, pride; computes per-tag aggregates under
three weightings:

  raw_words        : sum of post wordcount (multiply-counts multi-tagged posts)
  split_words      : wordcount / n_tags  (each post's mass shared across tags)
  importance_words : split_words * importance/10  (own-curated importance)

Usage:
    python analyse_blog.py /path/to/_posts
    python analyse_blog.py /path/to/_posts --plot --top 30
    python analyse_blog.py /path/to/_posts --no-meta-filter
    python analyse_blog.py /path/to/_posts --meta-tags lists,review,bio

Zero deps for the core analysis. matplotlib only required for --plot.
"""

from __future__ import annotations

import argparse
import csv
import re
import sys
from collections import defaultdict
from datetime import datetime
from pathlib import Path

# Tags treated as format/stance, not topics. Override with --meta-tags
# or --no-meta-filter. Drawn from a quick scan of /archive/.
DEFAULT_META_TAGS = {
    "lists", "navel-gazing", "becoming", "autobio", "book", "review",
    "hypothesis-dump", "interview", "obit", "argument", "bio", "criticism",
    "long-content", "rosetta-stone", "self-representation", "warning",
    "self-help",  # debatable — remove if you disagree
}

# Markdown / Liquid / HTML stripping for wordcount.
_INCLUDE = re.compile(r"\{%\s*include(?:_relative)?\s+([^\s%}]+)(?:\s+[^%]*?)?%\}")
_LIQUID_RAW = re.compile(r"\{%\s*raw\s*%\}.*?\{%\s*endraw\s*%\}", re.DOTALL)
_LIQUID = re.compile(r"\{%.*?%\}|\{\{.*?\}\}", re.DOTALL)
_STYLE = re.compile(r"<style\b[^>]*>.*?</style>", re.DOTALL | re.IGNORECASE)
_SCRIPT = re.compile(r"<script\b[^>]*>.*?</script>", re.DOTALL | re.IGNORECASE)
_CODE_FENCE = re.compile(r"```.*?```", re.DOTALL)
_INLINE_CODE = re.compile(r"`[^`]+`")
_MD_IMG = re.compile(r"!\[[^\]]*\]\([^)]+\)")
_MD_LINK = re.compile(r"\[([^\]]+)\]\([^)]+\)")
_HTML_TAG = re.compile(r"<[^>]+>")
_WORD = re.compile(r"\b\w+\b")


def expand_includes(text: str, includes_dir: Path | None,
                    depth: int = 0, missing: set[str] | None = None) -> str:
    """Recursively expand `{% include path %}` directives.

    Includes are resolved relative to `includes_dir` (Jekyll's _includes/).
    Liquid parameters after the path are ignored. Missing files are
    silently dropped (and recorded in `missing` if provided).
    """
    if includes_dir is None or depth > 5:
        return text

    def repl(m: re.Match) -> str:
        rel = m.group(1).strip().strip("\"'")
        path = includes_dir / rel
        if not path.is_file():
            if missing is not None:
                missing.add(rel)
            return ""
        try:
            content = path.read_text(encoding="utf-8", errors="replace")
        except OSError:
            return ""
        return expand_includes(content, includes_dir, depth + 1, missing)

    return _INCLUDE.sub(repl, text)


def compute_wordcount(body: str, includes_dir: Path | None = None,
                      missing: set[str] | None = None) -> int:
    text = expand_includes(body, includes_dir, missing=missing)
    text = _LIQUID_RAW.sub(" ", text)
    text = _STYLE.sub(" ", text)
    text = _SCRIPT.sub(" ", text)
    text = _CODE_FENCE.sub(" ", text)
    text = _LIQUID.sub(" ", text)
    text = _MD_IMG.sub(" ", text)
    text = _MD_LINK.sub(r"\1", text)
    text = _INLINE_CODE.sub(" ", text)
    text = _HTML_TAG.sub(" ", text)
    return len(_WORD.findall(text))


def parse_frontmatter(text: str) -> tuple[dict, str]:
    """Minimal YAML-ish frontmatter parser.

    Handles: scalar values, flow-style lists [a, b], block-style lists,
    and comma-separated strings (which Jekyll accepts for `categories`).
    Does not handle nested maps — none in this corpus.
    """
    if not text.startswith("---"):
        return {}, text
    end = text.find("\n---", 3)
    if end == -1:
        return {}, text
    fm_text = text[3:end].strip("\n")
    body = text[end + 4:].lstrip("\n")
    meta: dict = {}
    current_key: str | None = None
    block_list: list[str] | None = None
    for line in fm_text.splitlines():
        if not line.strip():
            continue
        if block_list is not None and line.lstrip().startswith("- "):
            block_list.append(line.lstrip()[2:].strip().strip("\"'"))
            continue
        if block_list is not None:
            meta[current_key] = block_list
            block_list = None
        if ":" not in line:
            continue
        key, _, value = line.partition(":")
        key = key.strip()
        value = value.strip()
        if not value:
            current_key = key
            block_list = []
            continue
        if value.startswith("[") and value.endswith("]"):
            items = [v.strip().strip("\"'") for v in value[1:-1].split(",")]
            meta[key] = [v for v in items if v]
        else:
            meta[key] = value.strip("\"'")
    if block_list is not None:
        meta[current_key] = block_list
    return meta, body


def coerce_int(v) -> int | None:
    if v is None:
        return None
    if isinstance(v, int):
        return v
    s = str(v).strip().rstrip("%").replace(",", "")
    try:
        return int(float(s))
    except ValueError:
        return None


def coerce_tags(meta: dict) -> list[str]:
    """Merge `tags` and `categories`. Either may be a list or csv string."""
    raw: list[str] = []
    for key in ("tags", "categories"):
        v = meta.get(key)
        if v is None:
            continue
        if isinstance(v, list):
            raw.extend(v)
        else:
            # csv string. Don't split on whitespace — tag names may have
            # internal hyphens but not spaces.
            raw.extend(part.strip() for part in str(v).split(","))
    out: list[str] = []
    seen: set[str] = set()
    for t in raw:
        t = t.strip().strip("\"'").lower()
        if t and t not in seen:
            seen.add(t)
            out.append(t)
    return out


def load_posts(posts_dir: Path, includes_dir: Path | None) -> list[dict]:
    posts: list[dict] = []
    paths = sorted(
        list(posts_dir.glob("*.md")) + list(posts_dir.glob("*.markdown"))
    )
    missing_includes: set[str] = set()
    for path in paths:
        text = path.read_text(encoding="utf-8", errors="replace")
        meta, body = parse_frontmatter(text)
        tags = coerce_tags(meta)

        # date: prefer frontmatter, fall back to filename
        date: datetime | None = None
        d = meta.get("date")
        if d:
            for fmt in ("%Y-%m-%d %H:%M:%S %z", "%Y-%m-%d %H:%M:%S",
                        "%Y-%m-%d", "%Y/%m/%d"):
                try:
                    date = datetime.strptime(str(d)[:len(fmt) + 5], fmt)
                    break
                except ValueError:
                    pass
        if date is None:
            m = re.match(r"(\d{4})-(\d{2})-(\d{2})-", path.name)
            if m:
                date = datetime(int(m[1]), int(m[2]), int(m[3]))

        wc = compute_wordcount(body, includes_dir, missing_includes)

        posts.append({
            "path": path.name,
            "title": meta.get("title", path.stem),
            "tags": tags,
            "wordcount": wc,
            "year": date.year if date else None,
            "importance": coerce_int(meta.get("importance")),
            "pride": coerce_int(meta.get("pride")),
        })
    if missing_includes:
        print(f"  warning: {len(missing_includes)} include path(s) not found: "
              f"{sorted(missing_includes)[:5]}{'...' if len(missing_includes) > 5 else ''}",
              file=sys.stderr)
    return posts


def compute_stats(posts: list[dict], meta_tags: set[str]) -> dict:
    stats: dict[str, dict] = defaultdict(lambda: {
        "posts": 0,
        "raw_words": 0,
        "split_words": 0.0,
        "importance_words": 0.0,
        "years": set(),
    })
    for p in posts:
        topics = [t for t in p["tags"] if t not in meta_tags]
        if not topics:
            continue
        share = p["wordcount"] / len(topics)
        imp = (p["importance"] or 0) / 10.0  # 0 if missing => no contribution
        for t in topics:
            s = stats[t]
            s["posts"] += 1
            s["raw_words"] += p["wordcount"]
            s["split_words"] += share
            s["importance_words"] += share * imp
            if p["year"]:
                s["years"].add(p["year"])
    return stats


def yearly_split(posts: list[dict], meta_tags: set[str]) -> dict:
    yt: dict[tuple[int, str], float] = defaultdict(float)
    for p in posts:
        if not p["year"]:
            continue
        topics = [t for t in p["tags"] if t not in meta_tags]
        if not topics:
            continue
        share = p["wordcount"] / len(topics)
        for t in topics:
            yt[(p["year"], t)] += share
    return yt


def write_csv(path: Path, header: list[str], rows: list) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        w = csv.writer(f)
        w.writerow(header)
        w.writerows(rows)


def make_plots(out: Path, rows: list, yt: dict, top: int) -> None:
    try:
        import matplotlib.pyplot as plt
    except ImportError:
        print("matplotlib not installed; skipping plots", file=sys.stderr)
        return

    head = rows[:top]
    fig, ax = plt.subplots(figsize=(8, 0.32 * len(head) + 1))
    ax.barh([r[0] for r in head][::-1], [r[2] for r in head][::-1])
    ax.set_xlabel("% of total post words")
    ax.set_title(f"Top {top} tags by share of post words")
    fig.tight_layout()
    fig.savefig(out / "tags_bar.png", dpi=140)
    plt.close(fig)

    top_tags = [r[0] for r in rows[:min(top, 12)]]
    years = sorted({y for (y, _) in yt})
    if not years:
        return
    fig2, ax2 = plt.subplots(figsize=(11, 5))
    bottom = [0.0] * len(years)
    for tag in top_tags:
        h = [yt.get((y, tag), 0.0) for y in years]
        ax2.bar(years, h, bottom=bottom, label=tag)
        bottom = [b + x for b, x in zip(bottom, h)]
    ax2.set_xlabel("year")
    ax2.set_ylabel("split wordcount")
    ax2.set_title(f"Top {len(top_tags)} tags, wordcount by year")
    ax2.legend(bbox_to_anchor=(1.02, 1), loc="upper left", fontsize=8)
    fig2.tight_layout()
    fig2.savefig(out / "tags_yearly.png", dpi=140)
    plt.close(fig2)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("posts_dir", type=Path, help="Jekyll _posts/ directory")
    ap.add_argument("--includes-dir", type=Path, default=None,
                    help="Jekyll _includes/ directory for resolving "
                         "{%% include ... %%} (default: posts_dir/../_includes)")
    ap.add_argument("--out", type=Path, default=Path("blog_analysis"))
    ap.add_argument("--top", type=int, default=25)
    ap.add_argument("--meta-tags", type=str, default=None,
                    help="comma-separated tags to treat as format/stance "
                         "(overrides default list)")
    ap.add_argument("--no-meta-filter", action="store_true")
    ap.add_argument("--plot", action="store_true")
    args = ap.parse_args()

    if not args.posts_dir.is_dir():
        print(f"posts_dir not found: {args.posts_dir}", file=sys.stderr)
        return 2

    if args.no_meta_filter:
        meta_tags = set()
    elif args.meta_tags is not None:
        meta_tags = {t.strip().lower() for t in args.meta_tags.split(",") if t.strip()}
    else:
        meta_tags = DEFAULT_META_TAGS

    args.out.mkdir(parents=True, exist_ok=True)

    inc = args.includes_dir
    if inc is None:
        guess = args.posts_dir.parent / "_includes"
        inc = guess if guess.is_dir() else None
    elif not inc.is_dir():
        print(f"includes_dir not found: {inc}", file=sys.stderr)
        return 2
    if inc:
        print(f"resolving includes from: {inc}")
    else:
        print("no _includes/ found; {% include %} directives will be dropped")

    posts = load_posts(args.posts_dir, inc)
    if not posts:
        print("no posts found", file=sys.stderr)
        return 1

    n_untagged = sum(1 for p in posts if not p["tags"])
    n_no_topic = sum(1 for p in posts if p["tags"] and not [t for t in p["tags"] if t not in meta_tags])
    total = sum(p["wordcount"] for p in posts)
    print(f"loaded {len(posts)} posts, {total:,} words")
    print(f"  untagged: {n_untagged}    only-meta-tags: {n_no_topic}")
    if meta_tags:
        print(f"  filtering meta tags: {sorted(meta_tags)}")

    # per-post sanity dump
    write_csv(
        args.out / "posts.csv",
        ["path", "year", "wordcount", "importance", "pride", "n_tags", "tags"],
        [
            [p["path"], p["year"] or "", p["wordcount"],
             p["importance"] or "", p["pride"] or "",
             len(p["tags"]), ";".join(p["tags"])]
            for p in posts
        ],
    )

    stats = compute_stats(posts, meta_tags)
    total_words = sum(p["wordcount"] for p in posts)  # denominator: ALL posts
    denom = total_words or 1  # guard against empty corpus
    rows = sorted(
        (
            [
                tag,
                s["posts"],
                round(100 * s["split_words"] / denom, 2),       # split_pct
                round(100 * s["importance_words"] / denom, 2),  # importance_pct
                int(s["raw_words"]),
                round(s["split_words"], 1),
                round(s["importance_words"], 2),
                min(s["years"]) if s["years"] else "",
                max(s["years"]) if s["years"] else "",
            ]
            for tag, s in stats.items()
        ),
        key=lambda r: -r[2],
    )
    write_csv(
        args.out / "tags.csv",
        ["tag", "posts", "split_pct", "importance_pct",
         "raw_words", "split_words", "importance_words",
         "first_year", "last_year"],
        rows,
    )

    yt = yearly_split(posts, meta_tags)
    write_csv(
        args.out / "yearly_tags.csv",
        ["year", "tag", "split_words"],
        [[y, t, round(w, 1)] for (y, t), w in sorted(yt.items())],
    )

    pct_accounted = sum(r[2] for r in rows)
    print(f"\ntotal post words: {total_words:,}    "
          f"accounted for by topic tags: {pct_accounted:.1f}%    "
          f"(residual = untagged or only-meta-tagged)")

    print(f"\ntop {args.top} by split_pct:")
    print(f"  {'tag':<22} {'posts':>5} {'split%':>7} {'imp%':>6} {'raw_words':>10}")
    for r in rows[:args.top]:
        print(f"  {r[0]:<22} {r[1]:>5} {r[2]:>6.2f}% {r[3]:>5.2f}% {r[4]:>10,}")

    print(f"\ntop {args.top} by importance_pct:")
    for r in sorted(rows, key=lambda r: -r[3])[:args.top]:
        print(f"  {r[0]:<22} {r[1]:>5} {r[2]:>6.2f}% {r[3]:>5.2f}% {r[4]:>10,}")

    if args.plot:
        make_plots(args.out, rows, yt, args.top)

    print(f"\nwrote {args.out}/{{posts,tags,yearly_tags}}.csv"
          + (" + plots" if args.plot else ""))
    return 0


if __name__ == "__main__":
    sys.exit(main())

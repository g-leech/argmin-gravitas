# archiver 🐾

A personal web archiver, MIT-licensed, originally built for
[croissanthology.com](https://croissanthology.com) but designed so any
static-site author can fork it. Captures every outbound link you cite three
ways — local self-contained HTML copy via [monolith](https://github.com/Y2Z/monolith),
the Internet Archive's Wayback Machine, and archive.is — and once a month
checks them all for rot. When a link dies, the local copy gets surfaced on
your own site as a sibling `[archived]` link next to the original. Inspired
by <https://gwern.net/archiving>, $0-hosted on GitHub Pages.

If you publish anywhere you care about, you probably want this. **It will not
fix already-published Substack posts** (Substack has no write API), but it
will catch the rot, archive the original, and email you a monthly checklist
of what to patch by hand. See ["the Substack hole"](#the-substack-hole) below.

## For your own site (forking)

1. Fork or clone this repo. The tool lives in `archiver/`.
2. Copy the example config and edit it:
   ```sh
   cp archiver/config.example.toml .archiver/config.toml
   $EDITOR .archiver/config.toml   # put your own domains + email
   ```
   `.archiver/` is gitignored so your email doesn't end up public.
3. Follow the [install](#install-on-the-macbook) below for now (macOS-only —
   PRs welcome for Linux/Windows).

## Install on the MacBook

```sh
# in this repo
cd archiver
cargo build --release

# install monolith (the actual page-saver — does the single-file HTML magic)
brew install monolith

# put the binary on your PATH so `archiver` works from anywhere
# (apple silicon: brew prefix is /opt/homebrew; intel: /usr/local)
ln -s "$PWD/target/release/archiver" /opt/homebrew/bin/archiver
```

No credentials needed. The monthly notifications use macOS-native plumbing:

- **Desktop notification** via `osascript` (`display notification …`)
- **Email** via Apple Mail via `osascript` — uses whatever account Mail.app
  is already signed into, no app passwords / SMTP setup
- **Persistent checklist** in `SUBSTACK_DEAD_LINKS.md` at the repo root
- **Terminal bell** when run interactively

The first time the cron triggers Apple Mail you'll get a one-time macOS prompt
asking *"Terminal wants to control Mail.app"* — click **Allow**. From then on
it's silent. If you want to pre-grant: System Settings → Privacy & Security
→ Automation → Terminal (or your shell) → enable Mail.

## Daily use

Just type:

```sh
archiver
```

Cat appears. Paste a URL. Done.

The `https://` is optional — bare domains like `clairebookworm.com` get auto-prepended.

**Interactive suffixes** (append to the URL when prompted):
- `/paywall` — engage the bypass chain (12ft.io → archive.is render → Googlebot UA)
- `/all` — archive the post AND every outbound link on it. Use for Scott-Alexander-style posts where half the citations have rotted; you save the post and its sources in one move.

**Interactive standalone commands** (type instead of pasting a URL):
- `/domain <yourblog.com>` — register a host as "yours" and bootstrap `.archiver/config.toml` (walks you through email + site origin on first run). The fast onboarding path — fork the repo, run `archiver`, type `/domain myblog.com`, you're set.
- `/info` — print all commands (subcommands + interactive) with descriptions.

Other commands:

| command | what it does |
| --- | --- |
| `archiver` | interactive prompt with cute cat |
| `archiver add <url>` | archive a single URL |
| `archiver add --paywall <url>` | …with bypass chain |
| `archiver post <url>` | archive every outbound link on a post (post itself not archived) |
| `archiver post --all <url>` | …and also archive the post itself (same as the `/all` suffix in interactive) |
| `archiver scan` | walk every post in this repo + `subslop/`, archive every link not yet captured |
| `archiver check [--dry-run]` | run a link-rot check on everything archived |
| `archiver rehost [--dry-run]` | for dead links: rewrite source posts to add `[archived]`, queue Substack notices |
| `archiver maintain [--dry-run]` | the full monthly pass — what launchd runs |
| `archiver list` | show what's archived and its status |

Pasting a croissanthology.com or croissanthology.substack.com URL is treated
as `archiver post <url>` — it fetches the post and archives every outbound link.
Pasting any other URL with `/all` does the same plus archives the page itself.

## First-run bulk archive

After you install, run once:

```sh
archiver scan
```

This walks `_posts/`, the root `.html` posts, and `subslop/` (your Substack
mirror), pulls out every outbound link, and archives the ones not yet
captured. Expect this to take a while the first time and to be approximately
free thereafter.

## Monthly automation (launchd)

```sh
# substitute your actual path. Render into LaunchAgents/ without mutating
# the tracked template, so future clones still have the placeholder.
REPO="$(cd .. && pwd)"
TARGET=~/Library/LaunchAgents/com.croissanthology.archiver.plist
mkdir -p ~/Library/LaunchAgents
sed "s|REPLACE_ME_REPO_PATH|$REPO|g" launchd/com.croissanthology.archiver.plist > "$TARGET"
launchctl load "$TARGET"
```

This runs `archiver maintain` at 09:00 on the 1st of every month: check
every link, rehost dead ones, email about dead Substack links, commit the
result, push.

To trigger a one-off run without waiting:

```sh
launchctl start com.croissanthology.archiver
tail -f archiver/launchd/maintain.log
```

To uninstall:

```sh
launchctl unload ~/Library/LaunchAgents/com.croissanthology.archiver.plist
```

## Where things live

- `archiver/src/` — Rust source
- `archive/<slug>/index.html` — local copies (served by GitHub Pages at `croissanthology.com/archive/<slug>/`)
- `.archiver/index.json` — state: every known URL, its three archive locations, check history, dead/rehost flags
- `archiver/launchd/` — the macOS scheduler config + log

## Twitter / X

`twitter.com` and `x.com` require client-side JS to render tweets, so monolith can't capture them directly. The archiver detects these hosts and silently rewrites the URL to a nitter mirror **for the local copy only** — wayback and archive.is still receive the original twitter URL, so the state semantically tracks the original tweet. If nitter.net goes down, change `nitter_host` in `.archiver/config.toml` to another instance (xcancel.com, nitter.poast.org, etc.).

## The Substack hole

Substack has no public write API. Once a post is published there, you cannot
edit its links programmatically — only through their web editor. So this tool's
auto-rewrite (`[archived]` sibling links) only fires for posts that live in
THIS repo (the markdown and HTML files in `_posts/`, `subslop/`, etc.).

For Substack posts: the archiver still
- captures every outbound link to local + wayback + archive.is,
- detects rot monthly,
- writes a checklist to `SUBSTACK_DEAD_LINKS.md` and emails you,
- so you can patch each broken citation by hand in the Substack editor.

It's worse than the auto-rewrite flow, but it's still infinitely better than
the default ("you find out about the dead link when a reader emails you in
2031"). If you want zero-touch citation health, host your essays on a static
site you control. See the upstream essay this tool grew out of for the wider
argument.

## Design notes

- A link is considered **dead** only after 3 consecutive failures with HTTP 404/410/451 or a DNS NXDOMAIN. 5xx, timeouts, network errors, and 401/403 are treated as transient and retried next month — never trigger a rehost.
- Tracking params (`utm_*`, `fbclid`, `gclid`, …) are stripped before archiving so the same article shared by two people doesn't get archived twice.
- Rehost rewrites in markdown posts add a `[[archived]](/archive/<slug>/)` link next to the dead one — original stays visible, archived copy is a sibling. HTML posts get the same `[archived]` anchor inserted after the dead `</a>`.
- Substack posts can't be auto-edited, so dead links inside them go into a monthly email.
- Your own domains (croissanthology.com, .substack.com, .github.io) are excluded from outbound-link extraction.

## Configuration

All runtime config lives in `<repo>/.archiver/config.toml` (gitignored).
Copy `archiver/config.example.toml` and edit. Fields:

- `own_domains` — what counts as "your own site" (skipped during extraction)
- `notify_email` — where the monthly Substack-dead emails go
- `site_origin` — public URL prefix used in those emails
- `nitter_host` — twitter→nitter rewrite target for local-copy fetches

Changes take effect on next run; no rebuild needed.

## License

MIT. See [LICENSE](./LICENSE).

## Contributing

PRs welcome, especially for:
- Linux / Windows cron equivalents (this is macOS-only today via launchd)
- Generic SMTP / webhook notifier so non-Mac users get pinged
- Better HTML link rewriter (current one uses string search; `scraper` would be more correct)
- Parallelism in `scan` and `check` (currently serial)

# Archiver — handoff doc

This file exists so that **Claude running on Margot's local Mac CLI** can pick
up the project without having seen the cloud-session conversation that built
it. Margot moved from a web/iPhone Claude session into a local terminal
session, which means the conversation history doesn't travel — only the repo
does. Everything below is what the previous Claude knew. Read it top to
bottom before doing anything.

---

## 1. The project, in Margot's own words

These are her literal messages from the cloud session, pasted verbatim and in
order. Treat them as the source of truth — if anything in this doc seems to
disagree with these, the quotes win.

### Initial pitch

> Ok so wondering if we could build a system that archived posts automatically?
> So here's what I have in mind: when the tool is activated in terminal, it
> allows me to paste a link into it and then archives it by automatically
> submitting it to Archive.is and archive.org as well as creating a local copy
> of the website (typically this will be a blog post). If it's paywalled, I'll
> input the link + "/paywall" or "/paywalled" and then the archive tries a few
> tricks to remove paywall and only then download the local copy. Local copies
> go into a special folder on my MacBook air. Ok and then here's a crucial
> function: every month, if it's activated in terminal (it will be, I'll
> always keep it alive), it'll go through my list of posts, and test every
> link for 404 pages and whatever other error exists. And if these appear
> (not just a network error or something, in which case the archive postpones)
> dead, it GENERATES a version of the dead link by uploading the local copy
> of it to my github and hosting it on my website. Got it? I think this'll be
> pretty epic and useful. I'm Margot by the way. And blogs are
> croissanthology.substack.com and croissanthology.com, those are the things
> that require checking. Things like YouTube videos can be locally
> snapshotted but obviously not downloaded as MP4 lmao. Also for inspiration:
> https://gwern.net/archiving

### After clarifying language/scope

> Yes let's do Rust I like Rust it comes with being trans apparently also
> with there being increasing cybersecurity worries but whatever. Anyway yes
> a launchd agent sounds good. Also ideally summoning the archive agent
> would be as simple as typing "archiver" in terminal.
>
> Yes on a and b. I'm sorry I should have been clearer. All outbound links
> in my blog should have a local copy, be backed up to the large internet
> archives, and swoop in in case of multiple errors. Since the code can't
> directly edit Substack I suppose what would be best is an email to
> margotwarrenbancquart@gmail.com along with a notification on the terminal
> app interface. Ok
>
> Yeah let's just dive in for now. Please make the terminal interface very
> pretty. Maybe an ASCII cat and a little happy animation when you feed it
> a link. If I feed it a link from croissanthology.com or the Substack
> website, what it does is back up all the links on that post. There's also
> an option for backing up links on the entire set if that's necessary (not
> redundant given the code will have a list of links it's archived), which
> I will of course do at first to ensure my existing website is OK.
>
> Ok you're going to find this funny but it turns out the iPhone 16 is too
> smol to let me see what you wrote so I can't read the rest. Hope this
> suffices to do good work

### After the first commit (now on Mac)

> Ok! I'm on my mac now. Is there any ability to move to the CLI for work?
> I'd like you to have direct access to my computer. Anyway looks good, this
> seems great, thanks. Wait why do you need Gmail creds to just send me an
> email.

### After being told about the SMTP-vs-Apple-Mail tradeoff

> Oh sorry yeah I have claude on my CLI I was just wondering if I could
> continue this conversation on the CLI. I'm going to take it that no? I
> suppose then could you please create a package describing what the
> project is to the other claude in my words maybe just copy everything I've
> said and then explain what you've done so far + the rust code. Does that
> work? Obviously in the end I need this on my computer, so it's probably
> better to code in the CLI... Also for notifications sure, Mac's
> notifications is good, worry on the terminal app is good, and I don't
> know how long apple mail will take to set up for what you're describing
> but if it takes less than 10 minutes I'm game

(That last paragraph greenlit the Apple Mail / macOS notification refactor.)

---

## 2. Inspiration & prior art

She referenced <https://gwern.net/archiving> as the mental model. Worth
skimming before any design changes. Gwern's three habits I copied:

1. **Three layers of archive**: a local self-contained copy, plus
   Wayback Machine, plus archive.is. Redundancy wins against link rot.
2. **Auto-detect dead links and substitute archives**. Margot wants the
   substitution to happen on her own site so her citations keep working.
3. **Track everything in a small structured index** — Gwern uses a flat
   archive directory; we use `.archiver/index.json`.

The Gwern-style **"append `[archived]` link next to the original"** pattern
is what I implemented for the rehost step (less destructive than replacing
the dead href, and lets the reader see both).

---

## 3. What's currently built

### Project layout (everything new is in this commit:
`Add archiver: personal web archiver for link-rot protection` and the
follow-up `archiver: drop SMTP, use macOS-native notifications`)

```
archiver/
  Cargo.toml         # crate manifest
  README.md          # user-facing install + commands
  HANDOFF.md         # this file
  .gitignore
  launchd/
    com.croissanthology.archiver.plist   # macOS scheduler config
    maintain.sh                          # wrapper invoked by launchd
  src/
    main.rs          # module declarations, calls cli::run()
    cli.rs           # clap subcommands + interactive mode
    config.rs        # Settings struct, repo_root() walker, config.toml loader
    state.rs         # JSON state, canonicalize(), make_slug()
    archive.rs       # Wayback + archive.is submission
    local.rs         # shells out to `monolith`
    paywall.rs       # 12ft → archive.is render → Googlebot UA
    extract.rs       # parse markdown + HTML for outbound links
    check.rs         # link-rot detection (dead vs transient)
    rehost.rs        # rewrite source posts with [archived] sibling links
    notify.rs        # macOS notification + Apple Mail + checklist file
    ui.rs            # ASCII cat, nom animation, colored output
archive/             # output: served by Jekyll at /archive/<slug>/
  .gitkeep
.archiver/           # state: created on first run
  index.json
```

### Subcommands (run `archiver --help`)

| command | what it does |
| --- | --- |
| `archiver` | interactive prompt with the ASCII cat |
| `archiver add <url>` | archive one URL |
| `archiver add --paywall <url>` | with bypass chain |
| `archiver post <url>` | archive every outbound link on a post (post itself not archived — appropriate for Margot's own posts) |
| `archiver post --all <url>` | …and also archive the post itself (use for external posts like SSC, where the post is at risk of rot too) |
| `archiver scan` | walk every post in the repo + `subslop/`, archive new outbound links |
| `archiver check [--dry-run]` | link-rot check on everything; dry-run skips state save |
| `archiver rehost [--dry-run]` | rewrite source posts for confirmed-dead links + queue substack patches; dry-run skips file writes and state save |
| `archiver list` | show what's archived |
| `archiver maintain [--dry-run]` | the full monthly pass — what launchd runs; dry-run skips files/mail/notifications/state |

Pasting a `croissanthology.com` or `croissanthology.substack.com` URL is
auto-detected and treated as `archiver post <url>` (outlinks only).

**Interactive-mode niceties**:
- `https://` is auto-prepended if missing — `clairebookworm.com` works.
- Suffix the URL with `/paywall` (or `/paywalled`) to engage the bypass chain.
- Suffix the URL with `/all` to also archive the post itself + every outbound link — equivalent to `archiver post --all <url>`.

### Notification design (post-refactor)

- **macOS notification** banner — `osascript` `display notification` —
  summary like `2 rehost(s) · 1 substack patch(es)`.
- **Apple Mail** — `osascript` `tell application "Mail" to make new outgoing
  message`. Uses whatever account Mail.app is signed into. **First run will
  trigger a one-time macOS Automation permission prompt.**
- **`SUBSTACK_DEAD_LINKS.md`** at the repo root — append-only checklist with
  date headers. Margot ticks boxes as she patches the live substack posts.
- **Terminal bell** (`\x07`) so an open terminal blips when the cron runs.

`lettre` and `~/.archiver.env` are gone. No SMTP creds anywhere.

### Design invariants worth keeping

- **A link is "dead" only after 3 consecutive HTTP 404/410/451 or DNS
  NXDOMAIN.** 5xx, timeouts, 401/403 (paywall/login walls), DNS transient
  failures — all classified as `Transient` and retried next month, never
  trigger a rehost. This is the "not just a network error" rule Margot
  asked for.
- **Tracking-param stripping** (`utm_*`, `fbclid`, `gclid`, `mc_cid`, etc.)
  happens during `canonicalize()` so the same article shared two ways
  doesn't get double-archived.
- **Margot's own domains are skipped** during outbound-link extraction.
  See `config::settings().own_domains` (loaded from `.archiver/config.toml`)
  + `config::is_own_host()`. Currently her config has:
  `croissanthology.com`, `www.croissanthology.com`,
  `croissanthology.github.io`, `croissanthology.substack.com`.
- **twitter.com / x.com are rewritten to nitter** inside `local::save_local`
  only. monolith can't capture twitter directly (JS-required); nitter is the
  server-side-rendered mirror. The state file still keys off the original
  tweet URL — wayback + archive.is also still see the original. If nitter.net
  goes down (it has historically), change `nitter_host` in `.archiver/config.toml`.
- **Substack mirror posts (`subslop/<slug>/index.html`) are detected
  via path-contains-`subslop/`** and never rewritten — the substack post
  can only be edited manually, so those go into the checklist+mail.
- **Markdown rewriting is precise; HTML rewriting is heuristic.** The
  markdown path uses an exact `](url)` needle. The HTML path searches for
  `href="url"`/`href='url'` then inserts after the next `</a>`. If you
  improve the HTML rewriter, the older root-level `.html` posts
  (`gell-mann.html`, `inkhaven.html`, …) benefit.

### Things I deferred / would do next

In rough priority order if Margot wants more polish:

1. **Tests**, especially for `state::canonicalize()`, `state::make_slug()`,
   `extract::from_markdown()`, `extract::from_html()`, and the markdown
   rehost rewriter. Zero tests in the first cut.
2. **Better HTML rewriter** — current one is conservative-but-heuristic.
   Switching to `scraper` for parse + rewrite would be more correct.
3. **`--dry-run` flag on `check` and `rehost`** so Margot can preview
   what would happen before files get touched.
4. **Concurrency** — `scan` and `check` are serial. Could be parallel
   with a small worker pool, but be polite to Wayback / archive.is.
5. **YouTube enrichment** — Margot explicitly accepted that YouTube
   pages get snapshotted as HTML only, no MP4. A nice addition would be
   capturing title + transcript via the YouTube oEmbed endpoint, stored
   as a sidecar `.json`. Out of scope for v1.
6. **Optional GH Actions cron** as a redundant backup to the local
   launchd job. The previous Claude offered this and she didn't answer
   either way — worth asking.

---

## 4. Setup checklist on the Mac

```sh
# Pull the branch (this doc lives on it)
git pull
git checkout claude/cross-device-conversation-Tfs4x

# Install the actual page-saver
brew install monolith

# Build and link the binary.
# Apple-Silicon macs: brew prefix is /opt/homebrew, and /usr/local/bin
# usually doesn't exist. Use /opt/homebrew/bin (margot already owns it,
# no sudo). On Intel macs swap in /usr/local/bin.
cd archiver
cargo build --release
ln -sf "$PWD/target/release/archiver" /opt/homebrew/bin/archiver

# Smoke test
archiver --help
archiver list   # → "nothing archived yet"

# First-run bulk archive over every existing post
archiver scan
```

Then install the launchd agent:

```sh
# From inside the archiver/ directory
sed -i '' "s|REPLACE_ME_REPO_PATH|$(cd .. && pwd)|g" \
  launchd/com.croissanthology.archiver.plist
cp launchd/com.croissanthology.archiver.plist ~/Library/LaunchAgents/
launchctl load ~/Library/LaunchAgents/com.croissanthology.archiver.plist

# Trigger a one-off run to verify
launchctl start com.croissanthology.archiver
tail -f archiver/launchd/maintain.log
```

The first time the cron triggers Apple Mail, macOS pops up an Automation
permission prompt → click **Allow**.

---

## 5. Known gotchas

- The previous Claude built this in an ephemeral cloud container with
  network access to crates.io, so the code compiles cleanly with
  `cargo build --release`, but **none of the network-using paths were
  actually tested against live Wayback / archive.is / monolith**. First
  real run on the Mac is the first proper test.
- archive.is rate-limits aggressively and sometimes returns a CAPTCHA
  challenge instead of submitting. If you see consistent failures there,
  consider adding a longer timeout or an exponential-backoff retry.
- monolith on macOS occasionally crashes on JS-heavy pages. The CLI
  surfaces the error and continues to the next archive layer — no data
  loss, just a missing local copy.
- `_config.yml` excludes `archiver/` and `.archiver/` from Jekyll, but
  intentionally serves `archive/` so dead-link rehosts have live URLs.

---

## 6. Margot's tone preferences

From the conversation:

- Casual, lowercase-y, comfortable with profanity ("smol", "lmao", "fuck",
  "epic"). Don't over-formalize.
- Pretty terminal output matters to her — she asked for the ASCII cat and
  the nom animation. Don't strip them as "unprofessional"; they're features.
- Sharp on tradeoffs — she immediately noticed the "wait why do you need
  Gmail creds" question. Be upfront when you've made a design choice; she'll
  push back if it's wrong.
- Trans + cybersecurity-aware. Treats Rust as a personality trait. Roll
  with it.

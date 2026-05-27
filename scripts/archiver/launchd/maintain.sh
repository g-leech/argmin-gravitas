#!/usr/bin/env zsh
# Wrapper invoked by com.croissanthology.archiver.plist.
# Runs the nightly archiver maintenance pass:
#   1. pull latest from origin
#   2. archiver scan — pick up new outbound citations from any posts you
#      added/edited since last run (cheap; already-archived links are skipped)
#   3. archiver maintain — check for rot, rehost dead links, mail summary.
#      `archiver rehost` itself git-adds the SPECIFIC archive dirs that back
#      confirmed-dead links (archive/ is gitignored so unstaged dirs stay local)
#   4. commit + push if anything got staged
# Notifications go through Apple Mail + macOS notification center.
set -euo pipefail

# Resolve the repo root (this script lives at <repo>/archiver/launchd/).
SCRIPT_DIR="${0:A:h}"
REPO_ROOT="${SCRIPT_DIR:h:h}"
cd "$REPO_ROOT"

# Pull latest first so we don't rehost against stale state.
git fetch --quiet origin
git pull --quiet --ff-only || true

BINARY="$REPO_ROOT/archiver/target/release/archiver"
if [[ ! -x "$BINARY" ]]; then
  echo "archiver binary not found at $BINARY — building it now..."
  (cd "$REPO_ROOT/archiver" && cargo build --release)
fi

# Pick up new citations from any posts added/edited since last run.
"$BINARY" scan

# Check for rot, rehost dead links, mail summary. archiver rehost stages
# only the specific archive dirs it actually needs (archive/ is gitignored).
"$BINARY" maintain

# Commit + push whatever got staged (rewritten posts, SUBSTACK_DEAD_LINKS.md,
# archive dirs that back confirmed-dead links). Also stage edited _posts/
# files in case rehost rewrote them.
git add -- _posts/ SUBSTACK_DEAD_LINKS.md 2>/dev/null || true

if ! git diff --quiet || ! git diff --cached --quiet; then
  git commit -m "archiver: nightly maintenance pass" || true
  git push --quiet origin "$(git rev-parse --abbrev-ref HEAD)" || true
fi

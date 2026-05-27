use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

/// Per-repo configuration. Loaded from `<repo>/.archiver/config.toml` if
/// present; otherwise generic placeholder defaults are used. Fork the repo,
/// `cp archiver/config.example.toml .archiver/config.toml`, edit, you're set.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    /// Domains that count as "your own" — outbound links to these are skipped
    /// during extraction (they're your own posts, not citations to archive).
    /// Subdomains match too (`example.com` matches `www.example.com`).
    pub own_domains: Vec<String>,

    /// Recipient for the monthly "X dead substack link(s) need a manual patch"
    /// email. Apple Mail uses whatever account Mail.app is signed into, so no
    /// SMTP / app-password setup is needed.
    pub notify_email: String,

    /// Public URL prefix for your site. Used in the monthly email + the
    /// SUBSTACK_DEAD_LINKS.md checklist when surfacing local-archive URLs.
    pub site_origin: String,

    /// Nitter mirror used to fetch a renderable copy of tweets — monolith
    /// can't capture twitter.com / x.com directly (they require JS). If
    /// nitter.net is down, swap in xcancel.com or nitter.poast.org.
    pub nitter_host: String,
}

impl Default for Settings {
    fn default() -> Self {
        // Generic placeholders. Run with an empty config to see the warning
        // that points users at config.example.toml.
        Self {
            own_domains: vec!["example.com".to_string()],
            notify_email: "you@example.com".to_string(),
            site_origin: "https://example.com".to_string(),
            nitter_host: "nitter.net".to_string(),
        }
    }
}

static SETTINGS: OnceLock<Settings> = OnceLock::new();

/// Returns the loaded settings. Reads `<repo>/.archiver/config.toml` on first
/// call and caches. If the file is missing or malformed, falls back to
/// placeholder defaults (with a stderr warning) so the binary still runs.
pub fn settings() -> &'static Settings {
    SETTINGS.get_or_init(|| match try_load() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("archiver: using placeholder config — {}", e);
            eprintln!(
                "archiver: create `<repo>/.archiver/config.toml` (see `archiver/config.example.toml`)."
            );
            Settings::default()
        }
    })
}

fn try_load() -> Result<Settings> {
    let path = config_path();
    if !path.exists() {
        anyhow::bail!("no config file at {:?}", path);
    }
    let s = fs::read_to_string(&path).with_context(|| format!("read {:?}", path))?;
    toml::from_str(&s).context("parse .archiver/config.toml")
}

pub fn config_path() -> PathBuf {
    repo_root().join(".archiver").join("config.toml")
}

pub fn repo_root() -> PathBuf {
    // 1. Explicit override (set this in your shell profile for use-from-anywhere).
    if let Ok(v) = std::env::var("ARCHIVER_REPO") {
        let p = PathBuf::from(v);
        if p.join(".git").exists() {
            return p;
        }
        eprintln!(
            "archiver: ARCHIVER_REPO points to {:?} but that's not a git repo",
            p
        );
        std::process::exit(2);
    }
    // 2. Walk up from cwd looking for .git.
    if let Ok(cwd) = std::env::current_dir() {
        let mut p = cwd.clone();
        loop {
            if p.join(".git").exists() {
                return p;
            }
            if !p.pop() {
                break;
            }
        }
    }
    // 3. Last resort: derive from the installed binary's resolved location.
    //    Under the documented setup the binary is at
    //    <repo>/archiver/target/release/archiver. Canonicalize to follow the
    //    /opt/homebrew/bin/archiver symlink.
    if let Ok(exe) = std::env::current_exe() {
        if let Ok(real) = std::fs::canonicalize(&exe) {
            let mut p = real;
            for _ in 0..5 {
                if !p.pop() {
                    break;
                }
                if p.join(".git").exists() {
                    return p;
                }
            }
        }
    }
    eprintln!(
        "archiver: cannot locate the site repo. \n\
         Either cd into the repo, or set ARCHIVER_REPO=/path/to/your-site-repo \n\
         (suggested: add `export ARCHIVER_REPO=$HOME/path/to/repo` to your ~/.zshrc)."
    );
    std::process::exit(2);
}

pub fn state_path() -> PathBuf {
    repo_root().join(".archiver").join("index.json")
}

pub fn is_own_host(host: &str) -> bool {
    let h = host.to_lowercase();
    settings()
        .own_domains
        .iter()
        .any(|d| h == *d || h.ends_with(&format!(".{}", d)))
}

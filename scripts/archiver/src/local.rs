use anyhow::{anyhow, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{config, ui};

pub fn save_local(url: &str, dest: &Path, user_agent: Option<&str>) -> Result<PathBuf> {
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent).context("create archive output dir")?;
    }
    let fetch_url = rewrite_for_local(url);
    if fetch_url != url {
        ui::info(&format!("twitter → {}", config::settings().nitter_host.as_str()));
    }
    let mut cmd = Command::new("monolith");
    cmd.arg(&fetch_url)
        .arg("-o")
        .arg(dest)
        .arg("--no-audio")
        .arg("--no-video")
        .arg("--quiet");
    if let Some(ua) = user_agent {
        cmd.arg("--user-agent").arg(ua);
    }
    let status = cmd
        .status()
        .context("run `monolith` (install with `brew install monolith` on macOS)")?;
    if !status.success() {
        return Err(anyhow!("monolith exited with {}", status));
    }
    Ok(dest.to_path_buf())
}

/// twitter.com / x.com require client-side JS, so monolith can't capture them.
/// Rewrite the host to a nitter instance for the *local* fetch only. The state
/// file still keys off the original URL (this transform never touches state).
pub fn rewrite_for_local(url: &str) -> String {
    let Ok(mut parsed) = url::Url::parse(url) else {
        return url.to_string();
    };
    let host = parsed
        .host_str()
        .map(|s| s.to_lowercase())
        .unwrap_or_default();
    let is_twitter = matches!(
        host.as_str(),
        "twitter.com"
            | "www.twitter.com"
            | "mobile.twitter.com"
            | "x.com"
            | "www.x.com"
            | "mobile.x.com"
    );
    if is_twitter {
        if parsed.set_host(Some(config::settings().nitter_host.as_str())).is_ok() {
            return parsed.to_string();
        }
    }
    url.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_twitter_com() {
        assert_eq!(
            rewrite_for_local("https://twitter.com/foo/status/12345"),
            "https://nitter.net/foo/status/12345"
        );
    }

    #[test]
    fn rewrites_x_com() {
        assert_eq!(
            rewrite_for_local("https://x.com/foo/status/12345"),
            "https://nitter.net/foo/status/12345"
        );
    }

    #[test]
    fn rewrites_mobile_and_www_variants() {
        assert_eq!(
            rewrite_for_local("https://mobile.twitter.com/foo/status/1"),
            "https://nitter.net/foo/status/1"
        );
        assert_eq!(
            rewrite_for_local("https://www.x.com/foo/status/1"),
            "https://nitter.net/foo/status/1"
        );
    }

    #[test]
    fn preserves_path_query_fragment() {
        assert_eq!(
            rewrite_for_local("https://twitter.com/a/status/9?s=20#m"),
            "https://nitter.net/a/status/9?s=20#m"
        );
    }

    #[test]
    fn leaves_other_hosts_alone() {
        let original = "https://example.com/post";
        assert_eq!(rewrite_for_local(original), original);
    }

    #[test]
    fn leaves_invalid_url_alone() {
        assert_eq!(rewrite_for_local("not a url"), "not a url");
    }
}

use anyhow::{anyhow, Context, Result};
use reqwest::blocking::Client;
use std::time::Duration;

pub fn client() -> Client {
    Client::builder()
        .user_agent(
            "Mozilla/5.0 (compatible; croissanthology-archiver/0.1; +https://croissanthology.com)",
        )
        .timeout(Duration::from_secs(120))
        .redirect(reqwest::redirect::Policy::limited(20))
        .build()
        .expect("build reqwest client")
}

/// Backoff delays between retry attempts (seconds). First attempt is
/// immediate; if it fails with 429/5xx/timeout, we wait and retry.
const WAYBACK_BACKOFF: &[u64] = &[0, 3, 12];
const ARCHIVE_IS_BACKOFF: &[u64] = &[0, 5, 20];

pub fn submit_wayback(url: &str) -> Result<String> {
    let c = client();
    let mut last_status: Option<u16> = None;
    let mut last_err: Option<anyhow::Error> = None;

    for delay in WAYBACK_BACKOFF {
        if *delay > 0 {
            std::thread::sleep(Duration::from_secs(*delay));
        }
        let resp = c
            .get(format!("https://web.archive.org/save/{}", url))
            .header("Accept", "text/html")
            .send();
        match resp {
            Ok(r) => {
                let code = r.status().as_u16();
                if is_retryable_status(code) {
                    last_status = Some(code);
                    continue;
                }
                let final_url = r.url().to_string();
                if final_url.contains("web.archive.org/web/") {
                    return Ok(final_url);
                }
                if let Some(loc) = r.headers().get("content-location").and_then(|v| v.to_str().ok())
                {
                    return Ok(format!("https://web.archive.org{}", loc));
                }
                if let Some(loc) = r.headers().get("location").and_then(|v| v.to_str().ok()) {
                    if loc.starts_with("http") {
                        return Ok(loc.to_string());
                    }
                    return Ok(format!("https://web.archive.org{}", loc));
                }
                return Err(anyhow!("wayback returned no snapshot URL (status {})", code));
            }
            Err(e) => {
                if e.is_timeout() {
                    last_err = Some(anyhow::Error::from(e));
                    continue;
                }
                return Err(anyhow::Error::from(e).context("wayback save request"));
            }
        }
    }
    if let Some(code) = last_status {
        Err(anyhow!(
            "wayback rate-limited (HTTP {}) after {} retries",
            code,
            WAYBACK_BACKOFF.len() - 1
        ))
    } else {
        Err(last_err.unwrap_or_else(|| anyhow!("wayback timed out after retries")))
    }
}

pub fn submit_archive_is(url: &str) -> Result<String> {
    let c = client();
    let mut last_status: Option<u16> = None;
    let mut last_err: Option<anyhow::Error> = None;

    for delay in ARCHIVE_IS_BACKOFF {
        if *delay > 0 {
            std::thread::sleep(Duration::from_secs(*delay));
        }
        let resp = c
            .post("https://archive.ph/submit/")
            .form(&[("url", url), ("anyway", "1")])
            .send();
        match resp {
            Ok(r) => {
                let code = r.status().as_u16();
                if is_retryable_status(code) {
                    last_status = Some(code);
                    continue;
                }
                if let Some(refresh) = r.headers().get("refresh").and_then(|v| v.to_str().ok()) {
                    let lower = refresh.to_lowercase();
                    if let Some(idx) = lower.find("url=") {
                        return Ok(refresh[idx + 4..].trim().to_string());
                    }
                }
                if let Some(loc) = r.headers().get("location").and_then(|v| v.to_str().ok()) {
                    if loc.starts_with("http") {
                        return Ok(loc.to_string());
                    }
                }
                let final_url = r.url().to_string();
                if is_archive_is_snapshot(&final_url) {
                    return Ok(final_url);
                }
                // CAPTCHA / submit page — retry doesn't help, the CAPTCHA
                // won't solve itself in 20s. Surface the failure.
                return Err(anyhow!(
                    "archive.is returned no snapshot URL (status {}, landed at {})",
                    code,
                    final_url
                ));
            }
            Err(e) => {
                if e.is_timeout() {
                    last_err = Some(anyhow::Error::from(e));
                    continue;
                }
                return Err(anyhow::Error::from(e).context("archive.is submit"));
            }
        }
    }
    if let Some(code) = last_status {
        Err(anyhow!(
            "archive.is rate-limited (HTTP {}) after {} retries",
            code,
            ARCHIVE_IS_BACKOFF.len() - 1
        ))
    } else {
        Err(last_err.unwrap_or_else(|| anyhow!("archive.is timed out after retries")))
    }
}

fn is_retryable_status(code: u16) -> bool {
    code == 429 || (500..=599).contains(&code)
}

/// True iff `url` looks like an archive.ph / archive.is snapshot — host matches
/// and path is not the bare submit endpoint. archive.ph snapshot paths look
/// like `/abc12` or `/2026/01/01/...`. `/submit/` and `/` itself don't count.
fn is_archive_is_snapshot(url: &str) -> bool {
    let Ok(parsed) = url::Url::parse(url) else {
        return false;
    };
    let Some(host) = parsed.host_str() else {
        return false;
    };
    let host_matches = host.ends_with("archive.ph")
        || host.ends_with("archive.is")
        || host.ends_with("archive.today");
    if !host_matches {
        return false;
    }
    let path = parsed.path();
    path != "/" && !path.starts_with("/submit")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_url_accepted() {
        assert!(is_archive_is_snapshot("https://archive.ph/abc12"));
        assert!(is_archive_is_snapshot("https://archive.is/2026/x"));
    }

    #[test]
    fn submit_endpoint_rejected() {
        // This was the bug: the submit landing page was claimed as a snapshot.
        assert!(!is_archive_is_snapshot("https://archive.ph/submit/"));
        assert!(!is_archive_is_snapshot("https://archive.ph/submit"));
    }

    #[test]
    fn bare_host_rejected() {
        assert!(!is_archive_is_snapshot("https://archive.ph/"));
    }

    #[test]
    fn other_hosts_rejected() {
        assert!(!is_archive_is_snapshot("https://example.com/abc"));
    }
}

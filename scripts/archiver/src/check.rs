use anyhow::Result;
use chrono::Utc;
use reqwest::blocking::Client;
use std::time::Duration;

use crate::state::{Check, CheckStatus, Entry};

const DEAD_AFTER_CONSECUTIVE: usize = 3;

pub fn check_one(client: &Client, entry: &mut Entry) -> Result<CheckStatus> {
    let result = client
        .get(&entry.canonical_url)
        .timeout(Duration::from_secs(30))
        .send();

    let check = match result {
        Ok(resp) => {
            let status = resp.status();
            let code = status.as_u16();
            if status.is_success() || status.is_redirection() {
                Check {
                    at: Utc::now(),
                    status: CheckStatus::Ok,
                    http: Some(code),
                    note: None,
                }
            } else if matches!(code, 404 | 410 | 451) {
                Check {
                    at: Utc::now(),
                    status: CheckStatus::Dead,
                    http: Some(code),
                    note: None,
                }
            } else if matches!(code, 401 | 403) {
                // could be paywall / login; treat as transient — not truly dead
                Check {
                    at: Utc::now(),
                    status: CheckStatus::Transient,
                    http: Some(code),
                    note: Some("auth-gated".into()),
                }
            } else if status.is_server_error() {
                Check {
                    at: Utc::now(),
                    status: CheckStatus::Transient,
                    http: Some(code),
                    note: Some("5xx".into()),
                }
            } else {
                Check {
                    at: Utc::now(),
                    status: CheckStatus::Unknown,
                    http: Some(code),
                    note: None,
                }
            }
        }
        Err(e) => {
            let msg = e.to_string().to_lowercase();
            let dns_dead = msg.contains("dns")
                || msg.contains("failed to lookup")
                || msg.contains("name resolution")
                || msg.contains("no such host")
                || msg.contains("nodename nor servname");
            Check {
                at: Utc::now(),
                status: if dns_dead { CheckStatus::Dead } else { CheckStatus::Transient },
                http: None,
                note: Some(e.to_string()),
            }
        }
    };

    let status_now = check.status.clone();
    entry.checks.push(check);

    // Confirmed dead only after N consecutive Dead results
    let recent_dead = entry
        .checks
        .iter()
        .rev()
        .take_while(|c| c.status != CheckStatus::Ok)
        .filter(|c| c.status == CheckStatus::Dead)
        .count();
    if recent_dead >= DEAD_AFTER_CONSECUTIVE {
        entry.dead = true;
    }

    // cap history
    if entry.checks.len() > 30 {
        let drop = entry.checks.len() - 30;
        entry.checks.drain(0..drop);
    }

    Ok(status_now)
}

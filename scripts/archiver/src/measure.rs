//! Lightweight link-rot measurement: fetch a post, extract body-only outbound
//! links (skipping comments via host-aware selectors), run parallel HEAD
//! requests to classify each as alive/dead/transient/unknown. Persists to
//! `.archiver/measurements.json`; `viz::render` consumes that into a static
//! HTML visualization.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use url::Url;

use crate::{archive, config, extract, ui};

const MEASUREMENT_FILE: &str = "measurements.json";
const CONCURRENT: usize = 16;
const PER_LINK_TIMEOUT_SECS: u64 = 15;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeasurementFile {
    pub version: u32,
    pub posts: Vec<PostMeasurement>,
}

impl Default for MeasurementFile {
    fn default() -> Self {
        Self {
            version: 1,
            posts: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostMeasurement {
    pub id: String,
    pub url: String,
    pub label: String,
    pub title: Option<String>,
    pub host: String,
    pub measured_at: DateTime<Utc>,
    pub total_links: usize,
    pub alive: usize,
    pub dead: usize,
    pub transient: usize,
    pub unknown: usize,
    pub links: Vec<LinkMeasurement>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkMeasurement {
    pub url: String,
    pub host: String,
    pub status: String,
    pub http: Option<u16>,
}

pub fn run(post_url: &str, label: Option<String>) -> Result<()> {
    let client = archive::client();

    ui::info(&format!("fetching {}", post_url));
    let html = client
        .get(post_url)
        .send()
        .with_context(|| format!("fetch {}", post_url))?
        .text()?;

    let title = extract::title(&html);
    let body_links: Vec<String> = extract::from_post_body(&html, post_url)
        .into_iter()
        .collect();
    let host = Url::parse(post_url)
        .ok()
        .and_then(|u| u.host_str().map(|s| s.to_string()))
        .unwrap_or_default();
    let label = label.unwrap_or_else(|| host.clone());

    if let Some(t) = &title {
        ui::info(&format!("title: {}", t));
    }
    ui::info(&format!(
        "{} outbound link(s) in post body (comments excluded)",
        body_links.len()
    ));

    if body_links.is_empty() {
        ui::warn("no links found — host-aware selector probably didn't match. check post_body_selectors.");
        return Ok(());
    }

    let results = Arc::new(Mutex::new(Vec::<LinkMeasurement>::with_capacity(
        body_links.len(),
    )));
    let pb = ProgressBar::new(body_links.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("  {spinner:.magenta} {pos}/{len} {wide_msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    std::thread::scope(|s| {
        for chunk in body_links.chunks(CONCURRENT) {
            let handles: Vec<_> = chunk
                .iter()
                .map(|link| {
                    let client = client.clone();
                    let results = Arc::clone(&results);
                    let pb = pb.clone();
                    let link = link.clone();
                    s.spawn(move || {
                        let m = measure_one(&client, &link);
                        pb.set_message(link.clone());
                        pb.inc(1);
                        results.lock().unwrap().push(m);
                    })
                })
                .collect();
            for h in handles {
                let _ = h.join();
            }
        }
    });
    pb.finish_and_clear();

    let mut links = Arc::try_unwrap(results)
        .map_err(|_| anyhow::anyhow!("results arc still shared"))
        .unwrap()
        .into_inner()
        .unwrap();
    links.sort_by(|a, b| a.url.cmp(&b.url));

    let alive = links.iter().filter(|l| l.status == "alive").count();
    let dead = links.iter().filter(|l| l.status == "dead").count();
    let transient = links.iter().filter(|l| l.status == "transient").count();
    let unknown = links.iter().filter(|l| l.status == "unknown").count();

    let post = PostMeasurement {
        id: make_id(post_url),
        url: post_url.to_string(),
        label: label.clone(),
        title,
        host,
        measured_at: Utc::now(),
        total_links: links.len(),
        alive,
        dead,
        transient,
        unknown,
        links,
    };

    upsert(&post)?;

    ui::header(&format!(
        "{}  →  alive {} · dead {} · transient {} · unknown {}",
        label, alive, dead, transient, unknown
    ));
    ui::info(&format!(
        "saved to {}",
        measurement_path().to_string_lossy()
    ));
    Ok(())
}

fn measure_one(client: &Client, url: &str) -> LinkMeasurement {
    let host = Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(|s| s.to_string()))
        .unwrap_or_default();

    // Try HEAD first; if the server refuses (405) or doesn't speak HEAD, fall
    // back to GET so we don't misclassify a live page as transient.
    let head_result = client
        .head(url)
        .timeout(Duration::from_secs(PER_LINK_TIMEOUT_SECS))
        .send();
    let (status, http) = match head_result {
        Ok(resp) if resp.status().as_u16() == 405 => {
            let get_result = client
                .get(url)
                .timeout(Duration::from_secs(PER_LINK_TIMEOUT_SECS))
                .send();
            classify(get_result)
        }
        other => classify(other),
    };

    LinkMeasurement {
        url: url.to_string(),
        host,
        status: status.to_string(),
        http,
    }
}

fn classify(
    result: Result<reqwest::blocking::Response, reqwest::Error>,
) -> (&'static str, Option<u16>) {
    match result {
        Ok(resp) => {
            let status = resp.status();
            let code = status.as_u16();
            if status.is_success() || status.is_redirection() {
                ("alive", Some(code))
            } else if matches!(code, 404 | 410 | 451) {
                ("dead", Some(code))
            } else if matches!(code, 401 | 403) {
                ("transient", Some(code))
            } else if status.is_server_error() {
                ("transient", Some(code))
            } else {
                ("unknown", Some(code))
            }
        }
        Err(e) => {
            let msg = e.to_string().to_lowercase();
            let dns_dead = msg.contains("dns")
                || msg.contains("failed to lookup")
                || msg.contains("name resolution")
                || msg.contains("no such host")
                || msg.contains("nodename nor servname");
            if dns_dead {
                ("dead", None)
            } else {
                ("transient", None)
            }
        }
    }
}

fn make_id(url: &str) -> String {
    let parsed = Url::parse(url).ok();
    let host = parsed
        .as_ref()
        .and_then(|u| u.host_str())
        .unwrap_or("unknown")
        .replace('.', "-");
    let path: String = parsed
        .as_ref()
        .map(|u| u.path().trim_matches('/').replace('/', "-"))
        .unwrap_or_default()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .take(50)
        .collect();
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    let hash = format!("{:x}", hasher.finalize());
    let short = &hash[..6];
    if path.is_empty() {
        format!("{}-{}", host, short)
    } else {
        format!("{}-{}-{}", host, path, short)
    }
}

pub fn measurement_path() -> PathBuf {
    config::repo_root().join(".archiver").join(MEASUREMENT_FILE)
}

pub fn load() -> Result<MeasurementFile> {
    let path = measurement_path();
    if !path.exists() {
        return Ok(MeasurementFile::default());
    }
    let s = fs::read_to_string(&path).context("read measurements")?;
    if s.trim().is_empty() {
        return Ok(MeasurementFile::default());
    }
    serde_json::from_str(&s).context("parse measurements")
}

fn upsert(post: &PostMeasurement) -> Result<()> {
    let path = measurement_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut data = load().unwrap_or_default();
    data.posts.retain(|p| p.id != post.id);
    data.posts.push(post.clone());
    fs::write(&path, serde_json::to_string_pretty(&data)?)?;
    Ok(())
}

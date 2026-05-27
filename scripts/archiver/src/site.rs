//! `archiver site <domain>` / `/site <domain>` interactive command.
//!
//! Fetches `https://<domain>/sitemap.xml`, parses out every URL, filters out
//! non-post pages by heuristic, then for each post URL runs the equivalent of
//! `archiver post --all <url>` — archives the post itself and every outbound
//! link on it. Handles both flat urlsets and recursive sitemap indexes.
//!
//! Sequential by design (be polite to wayback / archive.is). For a substack
//! with ~100 posts of ~30 links each you should expect this to take hours,
//! not minutes. State persists per-link so ctrl-c at any point is safe.

use anyhow::{Context, Result};
use regex::Regex;
use reqwest::blocking::Client;
use std::collections::BTreeSet;
use std::time::Duration;

use crate::{archive, cli, ui};

/// Polite delay between post archives so we don't hammer wayback / archive.is.
const POST_DELAY: Duration = Duration::from_secs(2);

/// Max depth for sitemap-index recursion (sitemap → child sitemaps → ...).
const MAX_DEPTH: usize = 3;

pub fn run(domain: &str, limit: Option<usize>) -> Result<()> {
    run_inner(domain, limit, false)
}

/// Enumerate-and-print only (no archiving). Used for testing the sitemap
/// fetch / filter logic without firing the full archive pipeline.
pub fn dry_run(domain: &str, limit: Option<usize>) -> Result<()> {
    run_inner(domain, limit, true)
}

fn run_inner(domain: &str, limit: Option<usize>, dry_run: bool) -> Result<()> {
    let host = domain
        .trim()
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/')
        .to_string();
    if host.is_empty() || host.contains(' ') {
        ui::warn(&format!("'{}' doesn't look like a domain", domain));
        return Ok(());
    }

    let sitemap_url = format!("https://{}/sitemap.xml", host);
    ui::header(&format!("🐾  fetching {}", sitemap_url));

    let client = archive::client();
    let mut urls = fetch_sitemap_urls(&client, &sitemap_url, 0)
        .with_context(|| format!("fetch sitemap at {}", sitemap_url))?;
    urls = filter_post_urls(urls, &host);
    urls.sort();
    urls.dedup();

    if urls.is_empty() {
        ui::warn("no post URLs found in sitemap (after filtering). is the sitemap at /sitemap.xml?");
        return Ok(());
    }
    ui::info(&format!(
        "{} post URL(s) to archive (after filtering)",
        urls.len()
    ));

    if let Some(n) = limit {
        if n < urls.len() {
            urls.truncate(n);
            ui::info(&format!("--limit applied: archiving the first {}", n));
        }
    }

    // Rough time estimate. Each archive_post_outlinks does monolith + wayback +
    // archive.is for the post, then archive_single per outbound link. Wayback
    // dominates at ~10–30s per submission. Estimate ~45s per post.
    let est_minutes = (urls.len() * 45) / 60;
    ui::info(&format!(
        "rough estimate: ~{} min sequential. ctrl-c is safe (state saves per-link).",
        est_minutes.max(1)
    ));

    if dry_run {
        ui::header("dry-run: would archive these posts");
        for (i, url) in urls.iter().enumerate() {
            println!("  [{}/{}] {}", i + 1, urls.len(), url);
        }
        return Ok(());
    }

    for (i, url) in urls.iter().enumerate() {
        ui::header(&format!(
            "🐾  [{}/{}] {}",
            i + 1,
            urls.len(),
            url
        ));
        if let Err(e) = cli::archive_post_outlinks(url, true) {
            ui::fail(&format!("post failed: {}", e));
            // continue to the next one rather than aborting the whole site
        }
        std::thread::sleep(POST_DELAY);
    }
    ui::happy_cat("site done");
    Ok(())
}

/// Fetch a sitemap URL and return all `<loc>` entries inside. If the response
/// is a `<sitemapindex>` (loc entries point at other sitemap.xml files),
/// recurse into each up to MAX_DEPTH.
fn fetch_sitemap_urls(client: &Client, url: &str, depth: usize) -> Result<Vec<String>> {
    if depth > MAX_DEPTH {
        return Ok(vec![]);
    }
    let body = client
        .get(url)
        .timeout(Duration::from_secs(30))
        .send()
        .with_context(|| format!("GET {}", url))?
        .text()?;
    let locs = extract_locs(&body);
    let is_index = body.contains("<sitemapindex") || body.contains("<SitemapIndex");
    if is_index {
        let mut all = Vec::new();
        for child in locs {
            match fetch_sitemap_urls(client, &child, depth + 1) {
                Ok(more) => all.extend(more),
                Err(e) => ui::warn(&format!("child sitemap {} failed: {}", child, e)),
            }
        }
        Ok(all)
    } else {
        Ok(locs)
    }
}

/// Pull `<loc>...</loc>` URLs out of sitemap XML. Tolerant of namespaces /
/// extra attributes — we just want the URL between the tags.
fn extract_locs(xml: &str) -> Vec<String> {
    // Permissive: handle <loc>...</loc> with or without namespace prefix.
    let re = Regex::new(r"<(?:[a-zA-Z0-9]+:)?loc>\s*([^<\s][^<]*?)\s*</(?:[a-zA-Z0-9]+:)?loc>")
        .expect("valid regex");
    re.captures_iter(xml)
        .map(|c| c[1].trim().to_string())
        .collect()
}

/// Drop non-post URLs by heuristic: nothing ending in .xml (sub-sitemaps),
/// nothing under common non-post paths, nothing off-host. Caller may want
/// to broaden this if their site has unusual URL shapes.
fn filter_post_urls(urls: Vec<String>, host: &str) -> Vec<String> {
    let exclude_substrings: &[&str] = &[
        "/sitemap",
        "/robots.txt",
        "/feed",
        "/rss",
        "/atom",
        "/tag/",
        "/category/",
        "/author/",
        "/about",
        "/archive",
        "/search",
        "/page/",
        "/wp-json",
        "/wp-content",
    ];
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut out = Vec::new();
    for u in urls {
        if u.ends_with(".xml") {
            continue;
        }
        if exclude_substrings.iter().any(|x| u.contains(x)) {
            continue;
        }
        // off-host links inside the sitemap (rare but possible — image
        // sitemap entries, CDN URLs, etc.) — skip.
        if let Ok(parsed) = url::Url::parse(&u) {
            if let Some(h) = parsed.host_str() {
                if !h.eq_ignore_ascii_case(host)
                    && !h.ends_with(&format!(".{}", host))
                {
                    continue;
                }
            }
        }
        if seen.insert(u.clone()) {
            out.push(u);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_basic_urlset() {
        let xml = r#"<?xml version="1.0"?><urlset><url><loc>https://a.com/p/x</loc></url><url><loc>https://a.com/p/y</loc></url></urlset>"#;
        let locs = extract_locs(xml);
        assert_eq!(locs, vec!["https://a.com/p/x", "https://a.com/p/y"]);
    }

    #[test]
    fn extract_handles_namespaces_and_whitespace() {
        let xml = r#"<urlset xmlns:ns="x"><url><ns:loc>  https://a.com/x  </ns:loc></url></urlset>"#;
        let locs = extract_locs(xml);
        assert_eq!(locs, vec!["https://a.com/x"]);
    }

    #[test]
    fn extract_substack_real_shape() {
        // Substack puts everything on one line with lots of namespace attrs.
        let xml = r##"<?xml version="1.0"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:news="http://www.google.com/schemas/sitemap-news/0.9"><url><loc>https://x.substack.com/archive</loc><changefreq>daily</changefreq></url><url><loc>https://x.substack.com/p/post-one</loc><lastmod>2026-05-20</lastmod></url><url><loc>https://x.substack.com/p/post-two</loc></url></urlset>"##;
        let locs = extract_locs(xml);
        assert_eq!(
            locs,
            vec![
                "https://x.substack.com/archive",
                "https://x.substack.com/p/post-one",
                "https://x.substack.com/p/post-two",
            ]
        );
    }

    #[test]
    fn filter_drops_about_and_archive() {
        let urls = vec![
            "https://x.substack.com/about".to_string(),
            "https://x.substack.com/archive".to_string(),
            "https://x.substack.com/p/real-post".to_string(),
        ];
        let kept = filter_post_urls(urls, "x.substack.com");
        assert_eq!(kept, vec!["https://x.substack.com/p/real-post"]);
    }

    #[test]
    fn filter_drops_off_host() {
        let urls = vec![
            "https://x.substack.com/p/real-post".to_string(),
            "https://cdn.example.com/image.jpg".to_string(),
        ];
        let kept = filter_post_urls(urls, "x.substack.com");
        assert_eq!(kept, vec!["https://x.substack.com/p/real-post"]);
    }

    #[test]
    fn filter_allows_subdomains_of_host() {
        let urls = vec![
            "https://blog.example.com/p/one".to_string(),
            "https://example.com/p/two".to_string(),
        ];
        let kept = filter_post_urls(urls, "example.com");
        assert_eq!(
            kept,
            vec!["https://blog.example.com/p/one", "https://example.com/p/two"]
        );
    }

    #[test]
    fn filter_dedups() {
        let urls = vec![
            "https://x.com/p/a".to_string(),
            "https://x.com/p/a".to_string(),
            "https://x.com/p/b".to_string(),
        ];
        let kept = filter_post_urls(urls, "x.com");
        assert_eq!(kept, vec!["https://x.com/p/a", "https://x.com/p/b"]);
    }

    #[test]
    fn filter_drops_sub_sitemap_xml() {
        let urls = vec![
            "https://x.com/sitemap-1.xml".to_string(),
            "https://x.com/p/real".to_string(),
        ];
        let kept = filter_post_urls(urls, "x.com");
        assert_eq!(kept, vec!["https://x.com/p/real"]);
    }
}

use anyhow::Result;
use pulldown_cmark::{Event, Parser, Tag};
use scraper::{Html, Selector};
use std::collections::BTreeSet;
use std::path::Path;
use url::Url;

use crate::config;

pub fn from_markdown(md: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for event in Parser::new(md) {
        if let Event::Start(Tag::Link { dest_url, .. }) = event {
            if let Some(s) = normalize_outbound(&dest_url) {
                out.insert(s);
            }
        }
    }
    out
}

pub fn from_html(html: &str) -> BTreeSet<String> {
    let doc = Html::parse_document(html);
    let sel = Selector::parse("a[href]").unwrap();
    let mut out = BTreeSet::new();
    for a in doc.select(&sel) {
        if let Some(href) = a.value().attr("href") {
            if let Some(s) = normalize_outbound(href) {
                out.insert(s);
            }
        }
    }
    out
}

/// Pull `<title>` text from a fetched page. Returns None if missing or empty.
pub fn title(html: &str) -> Option<String> {
    let doc = Html::parse_document(html);
    let sel = Selector::parse("title").ok()?;
    let t = doc.select(&sel).next()?;
    let text: String = t.text().collect::<String>().trim().to_string();
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

/// Extract outbound links from the post body only, using host-aware selectors
/// so comments / sidebars / chrome are excluded. Relative URLs are resolved
/// against `post_url`. Returns ALL outbound links (including same-host) —
/// the caller decides whether to filter further.
pub fn from_post_body(html: &str, post_url: &str) -> BTreeSet<String> {
    let doc = Html::parse_document(html);
    let selectors = post_body_selectors(post_url);
    let link_sel = Selector::parse("a[href]").unwrap();
    let mut out = BTreeSet::new();
    for css in selectors {
        let Ok(sel) = Selector::parse(css) else {
            continue;
        };
        let mut matched_any = false;
        for region in doc.select(&sel) {
            matched_any = true;
            for a in region.select(&link_sel) {
                if let Some(href) = a.value().attr("href") {
                    if let Some(s) = normalize_with_base(href, post_url) {
                        out.insert(s);
                    }
                }
            }
        }
        if matched_any {
            return out;
        }
    }
    // No selector matched — fall back to whole-document so we don't return empty
    // on an unrecognized host. The post-body filter is best-effort.
    for a in doc.select(&link_sel) {
        if let Some(href) = a.value().attr("href") {
            if let Some(s) = normalize_with_base(href, post_url) {
                out.insert(s);
            }
        }
    }
    out
}

fn post_body_selectors(post_url: &str) -> &'static [&'static str] {
    let host = Url::parse(post_url)
        .ok()
        .and_then(|u| u.host_str().map(|s| s.to_lowercase()))
        .unwrap_or_default();
    if host.ends_with("slatestarcodex.com") {
        // WordPress theme: post body wrapped in .pjgm-postcontent; comments are
        // a separate `#comments` div below.
        &["div.pjgm-postcontent"]
    } else if host.ends_with("substack.com")
        || host.ends_with("astralcodexten.com")
    {
        // Substack: .body.markup is the post body; .available-content is the
        // wider container. Comments load via JS and are not in the static HTML.
        &["div.body.markup", "div.available-content"]
    } else if host.ends_with("gwern.net") {
        // Gwern uses semantic <article>; no comments.
        &["article", "main"]
    } else {
        &["article", "main"]
    }
}

fn normalize_outbound(href: &str) -> Option<String> {
    let trimmed = href.trim();
    if trimmed.is_empty()
        || trimmed.starts_with('#')
        || trimmed.starts_with("mailto:")
        || trimmed.starts_with("javascript:")
        || trimmed.starts_with("tel:")
        || trimmed.starts_with("data:")
    {
        return None;
    }
    let u = Url::parse(trimmed).ok()?;
    if !matches!(u.scheme(), "http" | "https") {
        return None;
    }
    let host = u.host_str()?;
    if config::is_own_host(host) {
        return None;
    }
    Some(u.to_string())
}

/// Like `normalize_outbound` but accepts relative hrefs by resolving against
/// `base`. Same own-domain filter applies.
fn normalize_with_base(href: &str, base: &str) -> Option<String> {
    let trimmed = href.trim();
    if trimmed.is_empty()
        || trimmed.starts_with('#')
        || trimmed.starts_with("mailto:")
        || trimmed.starts_with("javascript:")
        || trimmed.starts_with("tel:")
        || trimmed.starts_with("data:")
    {
        return None;
    }
    let absolute = Url::parse(trimmed)
        .or_else(|_| Url::parse(base).and_then(|b| b.join(trimmed)))
        .ok()?;
    if !matches!(absolute.scheme(), "http" | "https") {
        return None;
    }
    let host = absolute.host_str()?;
    if config::is_own_host(host) {
        return None;
    }
    Some(absolute.to_string())
}

pub fn from_file(path: &Path) -> Result<BTreeSet<String>> {
    let content = std::fs::read_to_string(path)?;
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();
    Ok(match ext.as_str() {
        "md" | "markdown" => from_markdown(&content),
        _ => from_html(&content),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_extracts_plain_links() {
        let md = "see [this](https://example.com/foo) and [that](https://other.org/bar).";
        let links = from_markdown(md);
        assert!(links.iter().any(|l| l == "https://example.com/foo"));
        assert!(links.iter().any(|l| l == "https://other.org/bar"));
        assert_eq!(links.len(), 2);
    }

    #[test]
    fn markdown_skips_own_domain_and_subdomains() {
        let md = "[home](https://croissanthology.com/about) \
                  [sub](https://www.croissanthology.com/x) \
                  [substack](https://croissanthology.substack.com/p/foo) \
                  [ext](https://example.com)";
        let links = from_markdown(md);
        assert!(!links.iter().any(|l| l.contains("croissanthology")));
        assert_eq!(links.len(), 1);
    }

    #[test]
    fn markdown_skips_anchors_mailto_relative() {
        let md = "[a](#section) [b](mailto:m@example.com) \
                  [c](/posts/x) [d](posts/y) \
                  [e](https://example.com/x)";
        let links = from_markdown(md);
        assert_eq!(links.len(), 1);
        assert!(links.iter().any(|l| l == "https://example.com/x"));
    }

    #[test]
    fn markdown_dedupes_repeats() {
        let md = "[a](https://example.com/x) and again [b](https://example.com/x)";
        let links = from_markdown(md);
        assert_eq!(links.len(), 1);
    }

    #[test]
    fn html_extracts_both_quote_styles() {
        let html = r#"<p><a href="https://example.com/x">x</a> and <a href='https://other.org/y'>y</a></p>"#;
        let links = from_html(html);
        assert!(links.iter().any(|l| l == "https://example.com/x"));
        assert!(links.iter().any(|l| l == "https://other.org/y"));
    }

    #[test]
    fn html_skips_own_subdomains() {
        let html = r#"<a href="https://www.croissanthology.com/x">x</a><a href="https://example.com/y">y</a>"#;
        let links = from_html(html);
        assert!(!links.iter().any(|l| l.contains("croissanthology")));
        assert!(links.iter().any(|l| l == "https://example.com/y"));
    }

    #[test]
    fn html_skips_non_http_schemes() {
        let html = r#"<a href="javascript:void(0)">x</a>
                      <a href="data:text/html,hi">y</a>
                      <a href="tel:+1234">z</a>
                      <a href="ftp://files.example.com/x">f</a>
                      <a href="https://example.com/z">z</a>"#;
        let links = from_html(html);
        assert_eq!(links.len(), 1);
        assert!(links.iter().any(|l| l == "https://example.com/z"));
    }

    #[test]
    fn html_handles_no_links() {
        let links = from_html("<p>nothing to see here</p>");
        assert!(links.is_empty());
    }
}

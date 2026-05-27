use anyhow::Result;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::config;
use crate::state::State;

pub struct RehostReport {
    pub rewritten: Vec<(String, String)>,    // (source_post, dead_url)
    pub substack_pending: Vec<(String, String, String)>, // (post_path, dead_url, archive_url)
    pub skipped: Vec<String>,
    pub published_archives: Vec<String>, // archive dirs git-added (for log/email)
}

pub fn rehost_dead(state: &mut State, dry_run: bool) -> Result<RehostReport> {
    let repo = config::repo_root();
    let mut report = RehostReport {
        rewritten: vec![],
        substack_pending: vec![],
        skipped: vec![],
        published_archives: vec![],
    };

    let keys: Vec<String> = state.archives.keys().cloned().collect();
    for k in keys {
        let entry = state.archives.get_mut(&k).unwrap();
        if !entry.dead {
            continue;
        }
        let Some(local_rel) = entry.local_path.clone() else {
            report
                .skipped
                .push(format!("no local copy: {}", entry.canonical_url));
            continue;
        };
        let local_abs = repo.join(&local_rel);
        if !local_abs.exists() {
            report
                .skipped
                .push(format!("local copy missing on disk: {}", local_rel));
            continue;
        }

        let archive_url = format!("/{}", local_rel.trim_start_matches('/'));
        let mut needs_publish = false;

        for src in entry.source_posts.clone() {
            let src_abs = repo.join(&src);
            if !src_abs.exists() {
                continue;
            }
            if src.contains("subslop/") || src.contains("subslop\\") {
                let post_slug = std::path::Path::new(&src)
                    .parent()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();
                let pub_url = format!(
                    "https://croissanthology.substack.com/p/{}",
                    post_slug
                );
                let archive_public =
                    format!("{}{}", config::settings().site_origin, archive_url);
                report.substack_pending.push((
                    pub_url,
                    entry.canonical_url.clone(),
                    archive_public,
                ));
                // Substack posts can't be auto-edited, but the archive URL we'll
                // paste manually still has to exist on the live site.
                needs_publish = true;
                continue;
            }
            if rewrite_post(&src_abs, &entry.canonical_url, &archive_url, dry_run)? {
                report
                    .rewritten
                    .push((src.clone(), entry.canonical_url.clone()));
                needs_publish = true;
            }
        }
        if needs_publish && !dry_run {
            if let Some(dir) = publish_archive(&repo, &local_rel) {
                report.published_archives.push(dir);
            }
        }
        if !dry_run {
            entry.rehosted = true;
        }
    }

    Ok(report)
}

/// Stage the archive directory backing `local_rel` so the `<repo>/archive/<slug>/`
/// path actually exists on the live site (the directory is gitignored by default;
/// only load-bearing archives get pushed). Returns the relative dir we staged,
/// or None on git failure (gracefully ignored — the email still tells the user).
fn publish_archive(repo: &Path, local_rel: &str) -> Option<String> {
    let archive_dir = local_rel.rsplit_once('/').map(|(d, _)| d).unwrap_or(local_rel);
    let status = Command::new("git")
        .args(["add", "-f", "--", archive_dir])
        .current_dir(repo)
        .status()
        .ok()?;
    if status.success() {
        Some(archive_dir.to_string())
    } else {
        None
    }
}

/// Append a small `[archived]` link next to every occurrence of `dead_url`
/// in the markdown/html post. Returns true if anything would change (or did
/// change when not dry-run).
fn rewrite_post(
    path: &Path,
    dead_url: &str,
    archive_url: &str,
    dry_run: bool,
) -> Result<bool> {
    let original = fs::read_to_string(path)?;
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();

    let updated = match ext.as_str() {
        "md" | "markdown" => rewrite_markdown(&original, dead_url, archive_url),
        _ => rewrite_html(&original, dead_url, archive_url),
    };

    if updated != original {
        if !dry_run {
            fs::write(path, updated)?;
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

fn rewrite_markdown(content: &str, dead_url: &str, archive_url: &str) -> String {
    let needle = format!("]({})", dead_url);
    let marker = format!("]({}) [[archived]]({})", dead_url, archive_url);
    if content.contains(&format!("[[archived]]({})", archive_url)) {
        return content.to_string();
    }
    content.replace(&needle, &marker)
}

fn rewrite_html(content: &str, dead_url: &str, archive_url: &str) -> String {
    let marker = format!(
        r#" <a class="archiver-rehost" href="{}" title="local archive of dead link">[archived]</a>"#,
        archive_url
    );
    if content.contains(&marker) {
        return content.to_string();
    }
    // Conservative replacement: only patch anchors whose href exactly matches dead_url,
    // appending the marker right after their closing </a>.
    let patterns = [
        format!(r#"href="{}""#, dead_url),
        format!(r#"href='{}'"#, dead_url),
    ];
    let mut out = content.to_string();
    for pat in &patterns {
        if !out.contains(pat.as_str()) {
            continue;
        }
        let mut rebuilt = String::with_capacity(out.len() + 256);
        let mut cursor = 0;
        while let Some(pos) = out[cursor..].find(pat.as_str()) {
            let abs_pos = cursor + pos;
            // find the </a> after the matched href
            if let Some(close_rel) = out[abs_pos..].find("</a>") {
                let close_abs = abs_pos + close_rel + "</a>".len();
                rebuilt.push_str(&out[cursor..close_abs]);
                rebuilt.push_str(&marker);
                cursor = close_abs;
            } else {
                rebuilt.push_str(&out[cursor..]);
                cursor = out.len();
                break;
            }
        }
        rebuilt.push_str(&out[cursor..]);
        out = rebuilt;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEAD: &str = "https://dead.example.com/x";
    const ARCH: &str = "/archive/dead-example-com-x-abcd1234/index.html";

    #[test]
    fn md_appends_archived_sibling() {
        let body = "see [post](https://dead.example.com/x) for more.";
        let out = rewrite_markdown(body, DEAD, ARCH);
        assert!(
            out.contains("](https://dead.example.com/x) [[archived]](/archive/dead-example-com-x-abcd1234/index.html)"),
            "got: {}", out
        );
    }

    #[test]
    fn md_idempotent_when_already_archived() {
        let original = "[post](https://dead.example.com/x) [[archived]](/archive/dead-example-com-x-abcd1234/index.html)";
        let out = rewrite_markdown(original, DEAD, ARCH);
        assert_eq!(out, original);
    }

    #[test]
    fn md_no_match_unchanged() {
        let body = "[other](https://other.com/a)";
        let out = rewrite_markdown(body, DEAD, ARCH);
        assert_eq!(out, body);
    }

    #[test]
    fn md_rewrites_every_occurrence() {
        let body = "[one](https://dead.example.com/x) and [two](https://dead.example.com/x)";
        let out = rewrite_markdown(body, DEAD, ARCH);
        let count = out.matches("[[archived]]").count();
        assert_eq!(count, 2, "got: {}", out);
    }

    #[test]
    fn html_double_quotes_rewritten() {
        let html = r#"<p><a href="https://dead.example.com/x">post</a> after</p>"#;
        let out = rewrite_html(html, DEAD, ARCH);
        assert!(out.contains("archiver-rehost"), "got: {}", out);
        // marker must be AFTER the closing </a>
        let close = out.find("</a>").unwrap();
        let marker = out.find("archiver-rehost").unwrap();
        assert!(marker > close, "marker not after </a>: {}", out);
    }

    #[test]
    fn html_single_quotes_rewritten() {
        let html = r#"<p><a href='https://dead.example.com/x'>post</a></p>"#;
        let out = rewrite_html(html, DEAD, ARCH);
        assert!(out.contains("archiver-rehost"), "got: {}", out);
    }

    #[test]
    fn html_idempotent() {
        let already = format!(
            r#"<a href="https://dead.example.com/x">post</a> <a class="archiver-rehost" href="{}" title="local archive of dead link">[archived]</a>"#,
            ARCH
        );
        let out = rewrite_html(&already, DEAD, ARCH);
        assert_eq!(out, already);
    }

    #[test]
    fn html_no_match_unchanged() {
        let html = r#"<a href="https://other.com/a">x</a>"#;
        let out = rewrite_html(html, DEAD, ARCH);
        assert_eq!(out, html);
    }

    #[test]
    fn html_multiple_occurrences_each_get_marker() {
        let html = r#"<a href="https://dead.example.com/x">one</a> and <a href="https://dead.example.com/x">two</a>"#;
        let out = rewrite_html(html, DEAD, ARCH);
        let count = out.matches("archiver-rehost").count();
        assert_eq!(count, 2, "got: {}", out);
    }

    #[test]
    fn html_does_not_match_substring_url() {
        // dead_url is a strict prefix of another URL — must NOT be patched.
        let html = r#"<a href="https://dead.example.com/x-extra">other</a>"#;
        let out = rewrite_html(html, DEAD, ARCH);
        assert_eq!(out, html, "substring URL was patched: {}", out);
    }
}

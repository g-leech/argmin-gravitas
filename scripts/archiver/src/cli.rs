use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dialoguer::Input;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;
use url::Url;
use walkdir::WalkDir;

use crate::{
    archive, check, config, extract, measure, notify, rehost, site, state, ui, viz,
};

#[derive(Parser)]
#[command(
    name = "archiver",
    about = "personal web archiver: save now, never lose a citation later 🐾",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Archive a single URL on the Wayback Machine.
    Add { url: String },
    /// Archive every outbound link on a post. Pass --all to also archive the post itself.
    Post {
        url: String,
        /// Also archive the post itself, not just its outbound links.
        #[arg(long)]
        all: bool,
    },
    /// Walk every post in the repo + subslop/ and archive every new outbound link.
    Scan,
    /// Run a link-rot check on everything archived.
    Check {
        /// Run checks but do not persist state changes.
        #[arg(long)]
        dry_run: bool,
    },
    /// Process confirmed-dead links: rewrite source posts, queue substack notifications.
    Rehost {
        /// Compute rewrites but do not touch source files or save state.
        #[arg(long)]
        dry_run: bool,
    },
    /// Show everything archived.
    List,
    /// Full monthly pass: check → rehost → email.
    Maintain {
        /// Simulate the full pass without writing files, sending mail, or saving state.
        #[arg(long)]
        dry_run: bool,
    },
    /// Measure (don't archive) link rot on a post, for the comparison viz.
    Measure {
        url: String,
        /// Display label for the post in the visualization (default: host).
        #[arg(long)]
        label: Option<String>,
    },
    /// Archive every post on a blog (via its sitemap.xml) + every outbound link.
    Site {
        /// Domain of the blog, e.g. `you.substack.com` or `yourblog.com`.
        domain: String,
        /// Stop after archiving this many posts (handy for testing).
        #[arg(long)]
        limit: Option<usize>,
        /// Enumerate posts but don't actually archive — for verifying the
        /// sitemap fetch + filter without firing the full pipeline.
        #[arg(long)]
        dry_run: bool,
    },
    /// Render the static d3 visualization from `.archiver/measurements.json`.
    Viz {
        /// Only include posts whose label is in this comma-separated list.
        /// e.g. --labels "SSC 2014,Gwern 2014"
        #[arg(long)]
        labels: Option<String>,
        /// Exclude posts whose source URL is in this comma-separated list.
        #[arg(long)]
        exclude_urls: Option<String>,
        /// Output path relative to the repo root.
        #[arg(long, default_value = "archive-health.html")]
        output: String,
    },
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        None => interactive(),
        Some(Cmd::Add { url }) => add_url(&ensure_scheme(&url)),
        Some(Cmd::Post { url, all }) => archive_post_outlinks(&ensure_scheme(&url), all),
        Some(Cmd::Scan) => scan_all(),
        Some(Cmd::Check { dry_run }) => run_check(dry_run),
        Some(Cmd::Rehost { dry_run }) => run_rehost(dry_run),
        Some(Cmd::List) => run_list(),
        Some(Cmd::Maintain { dry_run }) => maintain(dry_run),
        Some(Cmd::Measure { url, label }) => measure::run(&ensure_scheme(&url), label),
        Some(Cmd::Site {
            domain,
            limit,
            dry_run,
        }) => {
            if dry_run {
                site::dry_run(&domain, limit)
            } else {
                site::run(&domain, limit)
            }
        }
        Some(Cmd::Viz {
            labels,
            exclude_urls,
            output,
        }) => viz::run(labels, exclude_urls, output),
    }
}

fn interactive() -> Result<()> {
    ui::banner();
    let raw: String = Input::new()
        .with_prompt(" url")
        .allow_empty(false)
        .interact_text()
        .context("read url from prompt")?;
    let trimmed = raw.trim();

    // Standalone slash commands (not URL + suffix). These start the input.
    if let Some(rest) = trimmed.strip_prefix('/') {
        let mut parts = rest.splitn(2, char::is_whitespace);
        let cmd = parts.next().unwrap_or("");
        let arg = parts.next().unwrap_or("").trim();
        return match cmd {
            "domain" => handle_domain_command(arg),
            "site" => site::run(arg, None),
            "info" | "help" => {
                print_info();
                Ok(())
            }
            other => {
                ui::warn(&format!("unknown command: /{}", other));
                print_info();
                Ok(())
            }
        };
    }

    let mut url = trimmed.to_string();
    let mut archive_all = false;
    for suffix in [" /all", "/all"] {
        if url.ends_with(suffix) {
            archive_all = true;
            url.truncate(url.len() - suffix.len());
            url = url.trim().to_string();
            break;
        }
    }

    let url = ensure_scheme(&url);
    if archive_all {
        return archive_post_outlinks(&url, true);
    }
    add_url(&url)
}

/// Handle the `/domain <yourblog.com>` slash command. Registers the host as
/// "your own" (skipped during outbound-link extraction) and writes
/// `.archiver/config.toml`. If the file doesn't exist yet or has placeholder
/// values, walks you through a one-time setup prompt.
fn handle_domain_command(arg: &str) -> Result<()> {
    if arg.is_empty() {
        ui::warn("usage: /domain <yourblog.com> (or substack handle, etc.)");
        return Ok(());
    }
    // Be forgiving about pasted scheme / trailing slashes.
    let domain = arg
        .trim()
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/')
        .to_string();
    if domain.is_empty() || domain.contains(' ') {
        ui::warn(&format!("'{}' doesn't look like a valid domain", arg));
        return Ok(());
    }

    let mut settings = config::settings().clone();
    let was_placeholder = settings.own_domains == vec!["example.com".to_string()];
    if was_placeholder {
        settings.own_domains.clear();
    }
    if settings.own_domains.iter().any(|d| d == &domain) {
        ui::info(&format!("domain already registered: {}", domain));
    } else {
        settings.own_domains.push(domain.clone());
        ui::success("registered", &domain);
    }

    // First-time setup: walk through the placeholder fields.
    if settings.notify_email == "you@example.com" || settings.notify_email.is_empty() {
        let email: String = Input::new()
            .with_prompt(" email for monthly dead-link notifications")
            .interact_text()
            .context("read email")?;
        settings.notify_email = email.trim().to_string();
    }
    if settings.site_origin == "https://example.com" || settings.site_origin.is_empty() {
        let default_origin = format!("https://{}", domain);
        let origin: String = Input::new()
            .with_prompt(" public URL prefix for your archive (e.g. https://yourblog.com)")
            .default(default_origin)
            .interact_text()
            .context("read site origin")?;
        settings.site_origin = origin.trim().trim_end_matches('/').to_string();
    }

    let path = config::config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).context("create .archiver dir")?;
    }
    let toml_str = toml::to_string_pretty(&settings).context("serialize config")?;
    std::fs::write(&path, toml_str).context("write config.toml")?;

    ui::success("config saved", &path.to_string_lossy());
    ui::info(&format!(
        "you're set. own_domains: {} · notify: {} · origin: {}",
        settings.own_domains.join(", "),
        settings.notify_email,
        settings.site_origin
    ));
    if was_placeholder {
        ui::info(
            "next: build + symlink the binary + install the systemd user timer — see archiver/README.md."
        );
    }
    ui::info("(restart `archiver` for the new config to take effect in this session.)");

    Ok(())
}

fn print_info() {
    use colored::Colorize;
    let m = "archiver 🐾".bright_magenta().bold();
    println!();
    println!(" {} — wayback-only link archiver for personal blogs", m);
    println!();
    println!(" {}", "what it does".bright_cyan().bold());
    let paras = [
        " archives every outbound link in your posts to the Internet Archive's",
        " Wayback Machine. then every night via a systemd timer it HEAD-checks each one.",
        "",
        " when a link returns 404/410/451 three nights in a row, it's flagged dead.",
        " for posts in your own repo (markdown/html), archiver auto-edits them",
        " inline to add an [archived] sibling link pointing at the wayback copy.",
        " for posts hosted elsewhere (substack, medium, ghost — anywhere you don't",
        " have direct file access), it emails you the dead URL + the source post",
        " URL + the wayback URL, so you patch by hand in the platform's editor.",
        "",
        " why: link rot is silent. nobody emails you when a citation dies, you",
        " just notice years later when a reader complains. gwern.net has ~3% rot",
        " at 12 years because of mechanisms like this; scott alexander's same-era",
        " SSC has ~36%. this is how you stay closer to gwern's number.",
        "",
        " limitation: it can't auto-write to platforms you don't control. for",
        " substack/medium/etc you get the email + the location, but the actual",
        " patch is manual. (substack has no public write API; gwern's auto-rewrite",
        " works only because he hosts his own site.)",
    ];
    for line in paras {
        println!("{}", line.bright_black());
    }
    println!();
    println!(" {}", "subcommands".bright_cyan().bold());
    let rows = [
        ("archiver", "interactive prompt with the cat"),
        ("archiver add <url>", "archive one URL on the Wayback Machine"),
        ("archiver post <url>", "archive every outbound link on a post (--all also archives the page)"),
        ("archiver scan", "walk your repo + subslop/ and archive new citations"),
        ("archiver check [--dry-run]", "HEAD every archived URL; 3 consecutive 404/410/NXDOMAIN → dead"),
        ("archiver rehost [--dry-run]", "rewrite source posts for dead links + stage archive dirs"),
        ("archiver maintain", "the full nightly pass: scan + check + rehost + notify"),
        ("archiver measure <url> [--label X]", "measure (don't archive) link rot — feeds the viz"),
        ("archiver viz [--labels X --exclude-urls Y --output Z]", "render the d3 visualization"),
        ("archiver list", "show everything archived + status"),
        ("archiver site <domain> [--limit N]", "archive every post on a blog (via /sitemap.xml) + every outbound link"),
    ];
    for (cmd, desc) in rows {
        println!("    {:<54} {}", cmd.bright_white(), desc.bright_black());
    }
    println!();
    println!(" {}", "in interactive mode (the cat prompt)".bright_cyan().bold());
    let slash = [
        ("paste a URL", "archive it (https:// optional)"),
        ("<url> /all", "archive the page + every outbound link on it"),
        ("/domain <yourblog.com>", "register the host as your own + bootstrap config.toml"),
        ("/site <yourblog.com>", "archive every post + every outbound link on that domain (hours-long; ctrl-c safe)"),
        ("/info", "show this help"),
    ];
    for (cmd, desc) in slash {
        println!("    {:<54} {}", cmd.bright_white(), desc.bright_black());
    }
    println!();
    println!(" {}", "where things live".bright_cyan().bold());
    println!("    {:<54} {}", "<repo>/.archiver/config.toml".bright_white(), "your config (gitignored)".bright_black());
    println!("    {:<54} {}", "<repo>/.archiver/index.json".bright_white(), "what you've archived + check history".bright_black());
    println!("    {:<54} {}", "<repo>/archive-health.html".bright_white(), "the d3 viz".bright_black());
    println!("    {:<54} {}", "<repo>/archiver/README.md".bright_white(), "full docs".bright_black());
    println!();
    println!(" runs nightly via systemd timer · gwern.net/archiving for the design lineage");
    println!(
        " built by {} at {} with {}",
        "margot".bright_magenta(),
        "croissanthology.com".bright_cyan(),
        "claude".bright_yellow(),
    );
    println!();
}

/// If the input doesn't carry a scheme (no `://`), prepend `https://`.
/// Pure string transform; URL validation happens downstream.
pub(crate) fn ensure_scheme(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.contains("://") {
        trimmed.to_string()
    } else {
        format!("https://{}", trimmed)
    }
}

fn add_url(url: &str) -> Result<()> {
    let parsed = Url::parse(url).context("invalid URL")?;
    let host = parsed.host_str().unwrap_or("");
    if config::is_own_host(host) {
        ui::header("🐾 own post detected — archiving every outbound link on it");
        return archive_post_outlinks(url, false);
    }
    archive_single(url, &[], true)
}

fn archive_single(url: &str, source_posts: &[String], animate: bool) -> Result<()> {
    let canon = state::canonicalize(url)?;
    let mut st = state::State::load(&config::state_path())?;

    let already_done = st
        .archives
        .get(&canon)
        .map(|e| e.wayback_url.is_some())
        .unwrap_or(false);

    if already_done {
        for src in source_posts {
            let entry = st.upsert(url)?;
            if !entry.source_posts.contains(src) {
                entry.source_posts.push(src.clone());
            }
        }
        st.save(&config::state_path())?;
        ui::info(&format!("already archived: {}", canon));
        return Ok(());
    }

    if animate {
        ui::noms(url);
    } else {
        ui::info(&format!("→ {}", url));
    }

    {
        let entry = st.upsert(url)?;
        for src in source_posts {
            if !entry.source_posts.contains(src) {
                entry.source_posts.push(src.clone());
            }
        }
    }

    match archive::submit_wayback(url) {
        Ok(u) => {
            let entry = st.upsert(url)?;
            entry.wayback_url = Some(u.clone());
            ui::success("wayback", &u);
        }
        Err(e) => ui::warn(&format!("wayback failed: {}", e)),
    }

    st.save(&config::state_path())?;
    Ok(())
}

pub(crate) fn archive_post_outlinks(post_url: &str, also_archive_post: bool) -> Result<()> {
    let client = archive::client();
    ui::info(&format!("fetching {}", post_url));
    let html = client
        .get(post_url)
        .send()
        .with_context(|| format!("fetch {}", post_url))?
        .text()?;
    let links = extract::from_html(&html);
    ui::info(&format!("found {} outbound link(s) on post", links.len()));

    let source = vec![format!("[live] {}", post_url)];

    if also_archive_post {
        ui::header("🐾 archiving the post itself");
        if let Err(e) = archive_single(post_url, &[], true) {
            ui::fail(&e.to_string());
        }
        ui::header(&format!("🐾 archiving {} outbound link(s)", links.len()));
    }

    for url in &links {
        if let Err(e) = archive_single(url, &source, false) {
            ui::fail(&e.to_string());
        }
        // Polite inter-link pacing so wayback doesn't rate-limit us mid-post.
        // Retries in archive.rs catch the rest.
        std::thread::sleep(Duration::from_millis(1000));
    }
    ui::happy_cat("post done");
    Ok(())
}

fn scan_all() -> Result<()> {
    let repo = config::repo_root();
    let mut posts: Vec<std::path::PathBuf> = vec![];
    let posts_dir = repo.join("_posts");
    if posts_dir.exists() {
        for entry in WalkDir::new(&posts_dir) {
            let entry = entry?;
            if entry.file_type().is_file() {
                posts.push(entry.path().to_path_buf());
            }
        }
    }
    for entry in std::fs::read_dir(&repo)? {
        let entry = entry?;
        let p = entry.path();
        if !p.is_file() {
            continue;
        }
        let name = match p.file_name().and_then(|s| s.to_str()) {
            Some(n) => n,
            None => continue,
        };
        if name.starts_with('_') || name.starts_with('.') {
            continue;
        }
        let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("");
        if ext == "html" {
            posts.push(p);
        }
    }
    let subslop = repo.join("subslop");
    if subslop.exists() {
        for entry in WalkDir::new(&subslop) {
            let entry = entry?;
            if entry.file_name() == "index.html" {
                posts.push(entry.path().to_path_buf());
            }
        }
    }
    ui::info(&format!("scanning {} post file(s)", posts.len()));

    let mut all_links: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for p in &posts {
        let rel = p
            .strip_prefix(&repo)
            .unwrap_or(p)
            .to_string_lossy()
            .to_string();
        match extract::from_file(p) {
            Ok(links) => {
                for l in links {
                    all_links.entry(l).or_default().push(rel.clone());
                }
            }
            Err(e) => ui::warn(&format!("{}: {}", rel, e)),
        }
    }
    ui::info(&format!(
        "found {} unique outbound link(s)",
        all_links.len()
    ));

    let mut st = state::State::load(&config::state_path())?;
    let mut already: BTreeSet<String> = BTreeSet::new();
    for url in all_links.keys() {
        if let Ok(c) = state::canonicalize(url) {
            if let Some(e) = st.archives.get(&c) {
                if e.wayback_url.is_some() {
                    already.insert(url.clone());
                }
            }
        }
    }
    // record source posts even for already-archived links
    for (url, sources) in &all_links {
        let entry = st.upsert(url)?;
        for s in sources {
            if !entry.source_posts.contains(s) {
                entry.source_posts.push(s.clone());
            }
        }
    }
    st.save(&config::state_path())?;

    let to_do: Vec<(String, Vec<String>)> = all_links
        .into_iter()
        .filter(|(u, _)| !already.contains(u))
        .collect();
    ui::header(&format!("{} link(s) need archiving", to_do.len()));

    for (url, sources) in to_do {
        if let Err(e) = archive_single(&url, &sources, false) {
            ui::fail(&e.to_string());
        }
    }
    ui::happy_cat("scan done");
    Ok(())
}

fn run_check(dry_run: bool) -> Result<()> {
    let mut st = state::State::load(&config::state_path())?;
    let client = archive::client();
    let total = st.archives.len() as u64;
    if total == 0 {
        ui::info("nothing archived yet — run `archiver scan` or feed me some links");
        return Ok(());
    }
    if dry_run {
        ui::info("[dry-run] running checks but state will NOT be saved");
    }
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::with_template(" {spinner:.magenta} {pos}/{len} {wide_msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    let keys: Vec<String> = st.archives.keys().cloned().collect();
    let mut dead_now: Vec<String> = vec![];
    for k in keys {
        let entry = st.archives.get_mut(&k).unwrap();
        pb.set_message(entry.canonical_url.clone());
        let was_dead = entry.dead;
        let _ = check::check_one(&client, entry);
        if entry.dead && !was_dead {
            dead_now.push(entry.canonical_url.clone());
        }
        pb.inc(1);
    }
    pb.finish_and_clear();

    if !dry_run {
        st.save(&config::state_path())?;
    }

    if dead_now.is_empty() {
        ui::happy_cat("all links healthy");
    } else {
        ui::sad_cat(&format!("{} link(s) newly dead", dead_now.len()));
        for u in dead_now {
            ui::fail(&u);
        }
    }
    Ok(())
}

fn run_rehost(dry_run: bool) -> Result<()> {
    let mut st = state::State::load(&config::state_path())?;
    if dry_run {
        ui::info("[dry-run] no files will be written and state will NOT be saved");
    }
    let report = rehost::rehost_dead(&mut st, dry_run)?;
    let label = if dry_run { "would rehost" } else { "rehosted" };
    for (src, dead) in &report.rewritten {
        ui::success(label, &format!("{} (in {})", dead, src));
    }
    for (post, dead, archive) in &report.substack_pending {
        ui::warn(&format!(
            "substack: {} cites dead {} (archive at {})",
            post, dead, archive
        ));
    }
    for s in &report.skipped {
        ui::info(&format!("skip: {}", s));
    }
    for dir in &report.published_archives {
        ui::success("staged for push", dir);
    }
    if !dry_run {
        st.save(&config::state_path())?;
    }
    Ok(())
}

fn run_list() -> Result<()> {
    let st = state::State::load(&config::state_path())?;
    if st.archives.is_empty() {
        ui::info("nothing archived yet");
        return Ok(());
    }
    for (u, e) in &st.archives {
        let status = if e.dead {
            "DEAD".to_string()
        } else if e.wayback_url.is_some() {
            "ok".to_string()
        } else {
            "partial".to_string()
        };
        ui::info(&format!(
            "[{}] {} -> {}",
            status,
            u,
            e.wayback_url.as_deref().unwrap_or("-")
        ));
    }
    Ok(())
}

fn maintain(dry_run: bool) -> Result<()> {
    run_check(dry_run)?;

    let mut st = state::State::load(&config::state_path())?;
    if dry_run {
        ui::info("[dry-run] no files written, no mail sent, no notifications fired");
    }
    let report = rehost::rehost_dead(&mut st, dry_run)?;
    let label = if dry_run { "would rehost" } else { "rehosted" };
    for (src, dead) in &report.rewritten {
        ui::success(label, &format!("{} (in {})", dead, src));
    }
    for s in &report.skipped {
        ui::info(&format!("skip: {}", s));
    }
    for dir in &report.published_archives {
        ui::success("staged for push", dir);
    }

    // Filter substack-pending items down to those not yet notified.
    let pending: Vec<(String, String, String)> = report
        .substack_pending
        .iter()
        .filter(|(_, dead, _)| {
            if let Ok(c) = state::canonicalize(dead) {
                if let Some(e) = st.archives.get(&c) {
                    return !e.notified;
                }
            }
            true
        })
        .cloned()
        .collect();

    if !pending.is_empty() && !dry_run {
        // Persistent checklist file in the repo (committed by the wrapper).
        match notify::append_substack_todo(&pending) {
            Ok(Some(path)) => ui::success("todo file", &path.to_string_lossy()),
            Ok(None) => {}
            Err(e) => ui::warn(&format!("substack todo file failed: {}", e)),
        }

        // Send mail via the system MTA (sendmail/msmtp/etc — whatever notify::send_mail wraps).
        let body = notify::render_mail_body(&pending);
        let subject = format!(
            "[archiver] {} dead substack link(s) need a manual patch",
            pending.len()
        );
        let to = config::settings().notify_email.as_str();
        match notify::send_mail(to, &subject, &body) {
            Ok(()) => ui::success("emailed", to),
            Err(e) => ui::warn(&format!("mail send failed: {}", e)),
        }

        // Mark notified so we don't re-spam next run.
        for (_, dead, _) in &pending {
            if let Ok(c) = state::canonicalize(dead) {
                if let Some(e) = st.archives.get_mut(&c) {
                    e.notified = true;
                }
            }
        }
    } else if !pending.is_empty() {
        for (post, dead, archive) in &pending {
            ui::warn(&format!(
                "substack pending: {} cites dead {} (archive at {})",
                post, dead, archive
            ));
        }
    }

    // Single summary notification banner.
    let summary = format!(
        "{} rehost(s) · {} substack patch(es)",
        report.rewritten.len(),
        pending.len()
    );
    if !dry_run && report.rewritten.len() + pending.len() > 0 {
        if let Err(e) = notify::desktop_notification("archiver 🐾", &summary) {
            ui::warn(&format!("desktop notification failed: {}", e));
        }
    } else if dry_run {
        ui::info(&format!("[dry-run] summary: {}", summary));
    }

    if !dry_run {
        st.save(&config::state_path())?;
        notify::terminal_bell();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_scheme_preserves_https() {
        assert_eq!(ensure_scheme("https://foo.com"), "https://foo.com");
    }

    #[test]
    fn ensure_scheme_preserves_http() {
        assert_eq!(ensure_scheme("http://foo.com"), "http://foo.com");
    }

    #[test]
    fn ensure_scheme_prepends_for_bare_domain() {
        assert_eq!(ensure_scheme("foo.com"), "https://foo.com");
    }

    #[test]
    fn ensure_scheme_prepends_for_bare_subdomain_with_path() {
        assert_eq!(
            ensure_scheme("www.example.com/posts/x"),
            "https://www.example.com/posts/x"
        );
    }

    #[test]
    fn ensure_scheme_trims_whitespace() {
        assert_eq!(ensure_scheme("  foo.com  "), "https://foo.com");
    }

    #[test]
    fn ensure_scheme_leaves_non_http_schemes_alone() {
        // We don't want to silently rewrite ftp:// or gopher:// — let URL parsing reject them.
        assert_eq!(ensure_scheme("ftp://x.com"), "ftp://x.com");
    }

    #[test]
    fn ensure_scheme_handles_empty() {
        // Empty input shouldn't panic; downstream Url::parse will produce a friendly error.
        assert_eq!(ensure_scheme(""), "https://");
    }
}
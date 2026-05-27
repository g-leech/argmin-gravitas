use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

use crate::archive;
use crate::local;

const GOOGLEBOT_UA: &str =
    "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";

pub fn save_local_with_bypass(url: &str, dest: &Path) -> Result<(PathBuf, &'static str)> {
    let mut errors: Vec<String> = vec![];

    // 1. 12ft.io reader proxy
    let proxied = format!("https://12ft.io/{}", url);
    match local::save_local(&proxied, dest, None) {
        Ok(p) => return Ok((p, "12ft.io")),
        Err(e) => errors.push(format!("12ft: {}", e)),
    }

    // 2. archive.is rendered page
    match archive::submit_archive_is(url) {
        Ok(archive_url) => match local::save_local(&archive_url, dest, None) {
            Ok(p) => return Ok((p, "archive.is render")),
            Err(e) => errors.push(format!("archive.is download: {}", e)),
        },
        Err(e) => errors.push(format!("archive.is submit: {}", e)),
    }

    // 3. Googlebot user-agent
    match local::save_local(url, dest, Some(GOOGLEBOT_UA)) {
        Ok(p) => return Ok((p, "googlebot UA")),
        Err(e) => errors.push(format!("googlebot: {}", e)),
    }

    Err(anyhow!("all paywall strategies failed: {}", errors.join("; ")))
}

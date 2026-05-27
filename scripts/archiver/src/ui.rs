use colored::*;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn banner() {
    let art = r#"
       /\_/\        ┌────────────────────────────────┐
      ( o.o )       │   archiver — feed me links     │
       > ^ <        │   they live forever 🐾         │
      / >🔗<        └────────────────────────────────┘
"#;
    println!("{}", art.bright_magenta());
    println!("  {} paste a URL.", "·".bright_black());
    println!(
        "  {} suffix with {} for paywall bypass, or {} to also archive every outbound link on the page.",
        "·".bright_black(),
        "/paywall".bright_yellow(),
        "/all".bright_green(),
    );
    println!(
        "  {} or type {} to set up your own blog · {} for the full command list.\n",
        "·".bright_black(),
        "/domain <yourblog.com>".bright_cyan(),
        "/info".bright_magenta(),
    );
}

pub fn noms(url: &str) {
    // Caption text only — these run BEFORE the network calls, so don't claim
    // success here. Per-layer ✓ / ! lines come after.
    let frames = [
        ("/\\_/\\", "( o.o )", " > ^ <  ", "spotted the link…"),
        ("/\\_/\\", "( -.- )", " > * <  ", "munching…"),
        ("/\\_/\\", "( ^.^ )", " > ~ <  ", "archiving…"),
        ("/\\_/\\", "( ^_^ )", " > w <  ", "🐾"),
    ];
    for _ in 0..5 {
        println!();
    }
    for (top, mid, bot, caption) in frames {
        print!("\x1B[5A");
        println!("\x1B[2K      {}    ", top.bright_magenta());
        println!("\x1B[2K      {}    ", mid.bright_magenta());
        println!("\x1B[2K      {}    ", bot.bright_magenta());
        println!(
            "\x1B[2K  {} {}",
            caption.bright_cyan(),
            truncate(url, 70).bright_white().underline()
        );
        println!("\x1B[2K");
        io::stdout().flush().ok();
        thread::sleep(Duration::from_millis(420));
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(max - 1).collect();
        out.push('…');
        out
    }
}

pub fn success(label: &str, value: &str) {
    println!("  {} {}: {}", "✓".green().bold(), label.bold(), value);
}
pub fn warn(msg: &str) {
    println!("  {} {}", "!".yellow().bold(), msg.yellow());
}
pub fn fail(msg: &str) {
    println!("  {} {}", "✗".red().bold(), msg.red());
}
pub fn info(msg: &str) {
    println!("  {} {}", "·".bright_black(), msg.bright_black());
}
pub fn header(msg: &str) {
    println!("\n{}", msg.bright_magenta().bold());
}

pub fn sad_cat(msg: &str) {
    let art = format!(
        r#"
       /\_/\
      ( T_T )    {}
       > ! <
"#,
        msg
    );
    println!("{}", art.bright_red());
}

pub fn happy_cat(msg: &str) {
    let art = format!(
        r#"
       /\_/\
      ( ^.^ )    {}
       > w <
"#,
        msg
    );
    println!("{}", art.bright_green());
}

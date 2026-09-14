//! rfetch — StickLab OS fastfetch in Rust (std only).
//! Big block-letter STICKLAB logo, neofetch-style info block, ANSI colors on TTYs.

use std::fs;
use std::io::IsTerminal;

const BOLD_MAGENTA: &str = "\x1b[1;95m";
const CYAN: &str = "\x1b[96m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

/// Color only on real terminals (never in pipes) unless NO_COLOR is set.
fn use_color(no_color_set: bool, is_tty: bool) -> bool {
    is_tty && !no_color_set
}

fn logo_lines(coloured: bool) -> Vec<String> {
    // Box-drawing block letters: deterministic alignment in every monospace font.
    let raw = [
        r" ███████╗ ████████╗ ██╗  ██████╗ ██╗  ██╗ ██╗       █████╗  ██████╗  ",
        r" ██╔════╝ ╚══██╔══╝ ██║ ██╔════╝ ██║ ██╔╝ ██║      ██╔══██╗ ██╔══██╗ ",
        r" ███████╗    ██║    ██║ ██║      █████╔╝  ██║      ███████║ ██████╔╝ ",
        r" ╚════██║    ██║    ██║ ██║      ██╔═██╗  ██║      ██╔══██║ ██╔══██╗ ",
        r" ███████║    ██║    ██║ ╚██████╗ ██║  ██╗ ███████╗ ██║  ██║ ██████╔╝ ",
        r" ╚══════╝    ╚═╝    ╚═╝  ╚═════╝ ╚═╝  ╚═╝ ╚══════╝ ╚═╝  ╚═╝ ╚═════╝  ",
        r"   ⚡ · sticklab os · by YOGEESHWARAN C · ★  ",
    ];
    raw.iter()
        .map(|l| {
            if coloured {
                format!("{BOLD_MAGENTA}{l}{RESET}")
            } else {
                l.to_string()
            }
        })
        .collect()
}

fn read_first(path: &str) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|_| "unknown".into())
        .trim()
        .to_string()
}

fn os_pretty() -> String {
    fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|c| {
            c.lines().find(|l| l.starts_with("PRETTY_NAME=")).map(|l| {
                l.trim_start_matches("PRETTY_NAME=")
                    .trim_matches('"')
                    .to_string()
            })
        })
        .unwrap_or_else(|| "StickLab OS".into())
}

fn mem_info() -> String {
    let content = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mut total = "?".to_string();
    let mut avail = "?".to_string();
    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            total = line.split_whitespace().nth(1).unwrap_or("?").to_string() + " kB";
        }
        if line.starts_with("MemAvailable:") {
            avail = line.split_whitespace().nth(1).unwrap_or("?").to_string() + " kB";
        }
    }
    format!("{avail} avail / {total} total")
}

fn cpu_model() -> String {
    fs::read_to_string("/proc/cpuinfo")
        .unwrap_or_default()
        .lines()
        .find(|l| l.starts_with("model name"))
        .map(|l| l.split(':').nth(1).unwrap_or("unknown").trim().to_string())
        .unwrap_or_else(|| "unknown cpu".into())
}

fn uptime() -> String {
    read_first("/proc/uptime")
        .split_whitespace()
        .next()
        .unwrap_or("?")
        .to_string()
        + "s"
}

fn shell_name() -> String {
    std::env::var("SHELL").unwrap_or_else(|_| "?".into())
}

fn de_name() -> String {
    std::env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| std::env::var("WAYLAND_DISPLAY").map(|_| "wayland".to_string()))
        .unwrap_or_else(|_| "tty".into())
}

/// (label, value) rows. Pure glue over the readers above.
fn info_rows() -> Vec<(String, String)> {
    let user = std::env::var("USER").unwrap_or_else(|_| "live".into());
    let host = read_first("/etc/hostname");
    vec![
        ("user".into(), format!("{user}@{host}")),
        ("os".into(), os_pretty()),
        (
            "kernel".into(),
            read_first("/proc/version")
                .split_whitespace()
                .nth(2)
                .unwrap_or("?")
                .to_string(),
        ),
        ("cpu".into(), cpu_model()),
        ("mem".into(), mem_info()),
        ("uptime".into(), uptime()),
        ("shell".into(), shell_name()),
        ("de".into(), de_name()),
    ]
}

/// Render full output. Pure — unit tested.
fn render(coloured: bool, rows: &[(String, String)]) -> String {
    let mut out = String::new();
    for l in logo_lines(coloured) {
        out.push_str(&l);
        out.push('\n');
    }
    for (i, (k, v)) in rows.iter().enumerate() {
        if coloured {
            out.push_str(&format!("  {CYAN}{k}{RESET} {DIM}·{RESET} {v}"));
        } else {
            out.push_str(&format!("  {k} · {v}"));
        }
        if i + 1 < rows.len() {
            out.push('\n');
        }
    }
    out
}

fn main() {
    let coloured = use_color(
        std::env::var_os("NO_COLOR").is_some(),
        std::io::stdout().is_terminal(),
    );
    println!("{}", render(coloured, &info_rows()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_only_on_tty_without_no_color() {
        assert!(use_color(false, true));
        assert!(!use_color(true, true));
        assert!(!use_color(false, false));
    }

    #[test]
    fn logo_has_seven_rows_and_sticklab_shape() {
        let plain = logo_lines(false);
        assert_eq!(plain.len(), 7);
        assert!(plain[0].contains("██╗"));
        assert!(plain[5].contains("╚═╝"));
    }

    #[test]
    fn render_contains_logo_and_rows() {
        let rows = vec![("os".to_string(), "StickLab OS".to_string())];
        let out = render(false, &rows);
        assert!(out.contains("██╗"));
        assert!(out.contains("os · StickLab OS"));
        // no ANSI escapes in plain mode
        assert!(!out.contains('\x1b'));
    }

    #[test]
    fn render_coloured_has_escapes() {
        let rows = vec![("os".to_string(), "StickLab OS".to_string())];
        assert!(render(true, &rows).contains('\x1b'));
    }
}

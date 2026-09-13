//! rinit — Rust init helper for tiny-rust-os.
//! Arch uses systemd as PID 1, so rinit runs as an early oneshot service:
//! mounts API filesystems if missing, prints a banner, then optionally
//! launches the minimal GUI session. No external crates (std only).

use std::fs;
use std::path::Path;
use std::process::Command;

fn is_mounted(path: &str) -> bool {
    fs::read_to_string("/proc/mounts")
        .map(|m| m.lines().any(|l| l.split_whitespace().nth(1) == Some(path)))
        .unwrap_or(false)
}

fn ensure_mounted(target: &str, fstype: &str, source: &str) {
    if is_mounted(target) {
        return;
    }
    if !Path::new(target).exists() {
        let _ = fs::create_dir_all(target);
    }
    let st = Command::new("mount")
        .args(["-t", fstype, source, target])
        .status();
    match st {
        Ok(s) if s.success() => println!("[rinit] mounted {target}"),
        _ => eprintln!("[rinit] WARN: could not mount {target} (may already be managed by systemd)"),
    }
}

fn main() {
    println!(" tiny-rust-os 0.1.0 — Rust init helper");
    println!(" -------------------------------------");

    // 1. API filesystems (harmless under systemd; needed if ever run as PID 1 in containers).
    ensure_mounted("/proc", "proc", "proc");
    ensure_mounted("/sys", "sysfs", "sys");
    ensure_mounted("/dev", "devtmpfs", "dev");

    // 2. Banner with basic facts.
    let kernel = fs::read_to_string("/proc/version").unwrap_or_else(|_| "unknown kernel".into());
    println!("[rinit] {} wielding Rust core", kernel.trim());
    println!("[rinit] pid={} — handing control to systemd", std::process::id());

    // 3. If labwc + foot exist, note desktop readiness (autostart via live user .bash_profile).
    let labwc = Path::new("/usr/bin/labwc").exists();
    let foot = Path::new("/usr/bin/foot").exists();
    println!(
        "[rinit] desktop: labwc={} foot={} (autostart via live user .bash_profile on tty1)",
        labwc, foot
    );
    println!("[rinit] done. Try: rfetch | rsetup coding");
}

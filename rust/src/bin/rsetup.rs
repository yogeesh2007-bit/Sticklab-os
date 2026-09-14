//! rsetup — first-boot helper for StickLab OS (std only).
//! Usage: rsetup status | rsetup coding | sudo rsetup set-hostname NAME | sudo rsetup enable-gui

use std::env;
use std::fs;
use std::process::Command;

fn help() {
    println!("rsetup — StickLab OS first-boot helper");
    println!("  rsetup status              hostname, network, desktop readiness");
    println!("  rsetup coding              coding toolchain versions (gcc, python, go, node, rust, nvim)");
    println!("  sudo rsetup set-hostname NAME");
    println!("  sudo rsetup enable-gui     enable NetworkManager + seatd, show desktop hint");
}

fn status() {
    let host = fs::read_to_string("/etc/hostname").unwrap_or_else(|_| "?".into());
    println!("hostname: {}", host.trim());
    let nm = Command::new("systemctl")
        .args(["is-enabled", "NetworkManager"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".into());
    println!("NetworkManager: {nm}");
    for app in ["labwc", "waybar", "wofi", "foot", "nvim"] {
        println!(
            "  {app}: {}",
            if std::path::Path::new(&format!("/usr/bin/{app}")).exists() {
                "installed"
            } else {
                "missing"
            }
        );
    }
    println!("hint: Super+Return = terminal, Super+D = launcher, right-click = menu");
}

fn tool_version(tool: &str, args: &[&str]) -> String {
    Command::new(tool)
        .args(args)
        .output()
        .ok()
        .and_then(|o| {
            let out = format!(
                "{}{}",
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            );
            out.lines().next().map(|l| l.trim().to_string())
        })
        .unwrap_or_else(|| "not installed".into())
}

fn coding() {
    println!("StickLab OS coding toolchains:");
    println!("  gcc    {}", tool_version("gcc", &["--version"]));
    println!("  make   {}", tool_version("make", &["--version"]));
    println!("  python {}", tool_version("python", &["--version"]));
    println!("  go     {}", tool_version("go", &["version"]));
    println!("  node   {}", tool_version("node", &["--version"]));
    println!("  rustup {}", tool_version("rustup", &["--version"]));
    println!("  nvim   {}", tool_version("nvim", &["--version"]));
    println!("  git    {}", tool_version("git", &["--version"]));
    println!("first Rust toolchain: rustup toolchain install stable");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        return;
    }
    match args[1].as_str() {
        "status" => status(),
        "coding" => coding(),
        "set-hostname" => {
            if args.len() < 3 {
                eprintln!("usage: sudo rsetup set-hostname NAME");
                std::process::exit(2);
            }
            fs::write("/etc/hostname", format!("{}\n", args[2])).expect("write /etc/hostname");
            let _ = Command::new("hostname").arg(&args[2]).status();
            println!("hostname set to {}", args[2]);
        }
        "enable-gui" => {
            for svc in ["NetworkManager", "seatd"] {
                let _ = Command::new("systemctl").args(["enable", svc]).status();
                println!("enabled {svc}");
            }
            println!("Desktop autostart: log in as `live` on tty1, labwc starts automatically.");
        }
        _ => help(),
    }
}

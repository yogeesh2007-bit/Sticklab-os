//! rsetup — first-boot helper for StickLab OS (std only).
//! Usage: rsetup status | rsetup coding | rsetup customize | rsetup wm [labwc|sway|hyprland] | sudo rsetup set-hostname NAME | sudo rsetup enable-gui

use std::env;
use std::fs;
use std::process::Command;

/// Window managers shipped on the ISO, in default order.
const WMS: &[&str] = &["labwc", "sway", "hyprland"];

/// Normalize user input to a known WM id. Accepts `hypr` as shorthand. Pure — unit tested.
fn normalize_wm(input: &str) -> Option<&'static str> {
    match input.trim().to_lowercase().as_str() {
        "labwc" => Some("labwc"),
        "sway" => Some("sway"),
        "hyprland" | "hypr" => Some("hyprland"),
        _ => None,
    }
}

/// Compositor binary for a WM id. Pure — unit tested.
fn wm_binary(wm: &str) -> &'static str {
    match wm {
        "sway" => "sway",
        "hyprland" => "Hyprland",
        _ => "labwc",
    }
}

/// Config file that controls the keys for a WM id. Pure — unit tested.
fn wm_config(wm: &str) -> &'static str {
    match wm {
        "sway" => "~/.config/sway/config",
        "hyprland" => "~/.config/hypr/hyprland.conf",
        _ => "~/.config/labwc/rc.xml",
    }
}

/// Saved session choice (~/.config/sticklab-wm), defaulting to labwc. Pure over file text.
fn saved_wm(file_text: Option<&str>) -> &'static str {
    file_text.and_then(normalize_wm).unwrap_or("labwc")
}

/// Valid Linux hostname: 1-63 chars, ASCII letters/digits/hyphen, never
/// leading/trailing hyphen. Pure — unit tested. Rejects newline/space/control
/// injection into /etc/hostname (this tool runs as root).
fn valid_hostname(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 63
        && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        && !name.starts_with('-')
        && !name.ends_with('-')
}

fn wm_path() -> std::path::PathBuf {
    std::env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("/root"))
        .join(".config/sticklab-wm")
}

fn wm(args: &[String]) {
    if args.len() < 3 {
        let saved = fs::read_to_string(wm_path()).ok();
        let cur = saved_wm(saved.as_deref());
        println!("StickLab OS window managers (all Wayland, same Bolt=Alt keys, same bar):");
        for id in WMS {
            let mark = if *id == cur { "*" } else { " " };
            let state = if std::path::Path::new(&format!("/usr/bin/{}", wm_binary(id))).exists() {
                "installed"
            } else {
                "missing"
            };
            let style = match *id {
                "labwc" => "stacking (default, Openbox-style)",
                "sway" => "manual tiling",
                _ => "dynamic tiling + eye candy",
            };
            println!(
                " {mark} {id:<9} {style:<32} [{state}]  keys: {}",
                wm_config(id)
            );
        }
        println!("current: {cur} (takes effect on next tty1 login)");
        println!("switch:  rsetup wm <labwc|sway|hyprland>   or one-shot: STICKLAB_WM=sway");
        return;
    }
    match normalize_wm(&args[2]) {
        Some(id) => {
            let p = wm_path();
            if let Some(parent) = p.parent() {
                let _ = fs::create_dir_all(parent);
            }
            match fs::write(&p, format!("{id}\n")) {
                Ok(()) =>             println!("window manager → {id} (saved to {}, takes effect on next tty1 login; keys: {})", p.display(), wm_config(id)),
                Err(e) => {
                    eprintln!("could not save {p:?}: {e}");
                    std::process::exit(1);
                }
            }
        }
        None => {
            eprintln!(
                "unknown window manager '{}'. Try: rsetup wm <labwc|sway|hyprland>",
                args[2]
            );
            std::process::exit(2);
        }
    }
}

fn help() {
    println!("rsetup — StickLab OS first-boot helper");
    println!("  rsetup status              hostname, network, desktop readiness");
    println!("  rsetup coding              coding toolchain versions (gcc, python, go, node, rust, nvim)");
    println!(
        "  rsetup customize           map of every themeable file (keys, bar, terminal, editor)"
    );
    println!("  rsetup wm [labwc|sway|hyprland]   list or switch window manager (next tty1 login)");
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
    for app in [
        "labwc", "sway", "Hyprland", "waybar", "wofi", "foot", "nvim",
    ] {
        println!(
            "  {app}: {}",
            if std::path::Path::new(&format!("/usr/bin/{app}")).exists() {
                "installed"
            } else {
                "missing"
            }
        );
    }
    println!(
        "hint: Bolt+Return = terminal, Bolt+D = launcher, right-click = menu (Bolt is the Alt key)"
    );
}

fn customize() {
    println!("StickLab OS customization map (all plain text, edit + reload):");
    println!("  wm        rsetup wm                       labwc (stacking) · sway (tiling) · hyprland (dynamic)");
    println!("  keys      ~/.config/labwc/rc.xml          Bolt (= Alt) shortcuts, add your own");
    println!("            ~/.config/sway/config           same Bolt keys for the sway session");
    println!("            ~/.config/hypr/hyprland.conf    same Bolt keys for the hyprland session");
    println!("  menu      ~/.config/labwc/menu.xml        right-click menu entries");
    println!("  autostart ~/.config/labwc/autostart       wallpaper, bar, notifications, extras");
    println!(
        "  bar       ~/.config/waybar/config.jsonc + style.css   modules, position, accent #b4befe"
    );
    println!("  launcher  ~/.config/wofi/config + style.css           size, prompt, accent");
    println!("  terminal  ~/.config/foot/foot.ini         font, padding, 16 colors + accent pairs");
    println!("  editor    ~/.config/nvim/init.lua         options, keymaps, plugin pointer");
    println!("  shell     ~/.zshrc  ~/.bashrc             prompt, aliases, EDITOR");
    println!("  accent    swap #b4befe (lavender) for #a6e3a1 mint, #fab387 peach, #f38ba8 red");
    println!("  reload    labwc --reconfigure · sway reload (Bolt+Shift+R) · hyprctl reload · pkill -USR2 waybar");
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
    println!("StickLab OS hardware bench:");
    println!("  picocom {}", tool_version("picocom", &["--help"]));
    println!("  openocd {}", tool_version("openocd", &["--version"]));
    println!("  avrdude {}", tool_version("avrdude", &["--version"]));
    println!("plug a board in and run `sticklab boards` to detect it");
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
        "customize" => customize(),
        "wm" | "desktop" | "session" => wm(&args),
        "set-hostname" => {
            if args.len() < 3 {
                eprintln!("usage: sudo rsetup set-hostname NAME");
                std::process::exit(2);
            }
            if !valid_hostname(&args[2]) {
                eprintln!(
                    "invalid hostname '{}': use 1-63 letters, digits, hyphens (no leading/trailing hyphen)",
                    args[2]
                );
                std::process::exit(2);
            }
            if let Err(e) = fs::write("/etc/hostname", format!("{}\n", args[2])) {
                eprintln!("could not write /etc/hostname (need root?): {e}");
                std::process::exit(1);
            }
            let _ = Command::new("hostname").arg(&args[2]).status();
            println!("hostname set to {}", args[2]);
        }
        "enable-gui" => {
            let mut failed = false;
            for svc in ["NetworkManager", "seatd"] {
                let ok = Command::new("systemctl")
                    .args(["enable", svc])
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false);
                println!("{} {svc}", if ok { "enabled" } else { "FAILED to enable" });
                failed |= !ok;
            }
            if failed {
                std::process::exit(1);
            }
            println!("Desktop autostart: log in on tty1, your chosen WM starts automatically (`rsetup wm` to switch).");
        }
        _ => help(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wm_names_normalize_case_and_shorthand() {
        assert_eq!(normalize_wm("labwc"), Some("labwc"));
        assert_eq!(normalize_wm("Sway"), Some("sway"));
        assert_eq!(normalize_wm("  hypr  "), Some("hyprland"));
        assert_eq!(normalize_wm("Hyprland"), Some("hyprland"));
        assert_eq!(normalize_wm("i3"), None);
        assert_eq!(normalize_wm(""), None);
    }

    #[test]
    fn wm_binaries_and_configs_cover_all_sessions() {
        for id in WMS {
            assert!(!wm_binary(id).is_empty());
            assert!(wm_config(id).starts_with("~/.config/"));
        }
        assert_eq!(wm_binary("hyprland"), "Hyprland");
        assert_eq!(wm_binary("sway"), "sway");
        assert_eq!(wm_binary("labwc"), "labwc");
    }

    #[test]
    fn saved_wm_defaults_to_labwc() {
        assert_eq!(saved_wm(None), "labwc");
        assert_eq!(saved_wm(Some("garbage")), "labwc");
        assert_eq!(saved_wm(Some("sway\n")), "sway");
        assert_eq!(saved_wm(Some("hypr")), "hyprland");
    }

    #[test]
    fn hostname_allows_dns_names_rejects_injection() {
        assert!(valid_hostname("sticklab-os"));
        assert!(valid_hostname("lab1"));
        assert!(valid_hostname("a"));
        assert!(!valid_hostname(""));
        assert!(!valid_hostname("-lead"));
        assert!(!valid_hostname("trail-"));
        assert!(!valid_hostname("has space"));
        assert!(!valid_hostname("new\nline"));
        assert!(!valid_hostname("semi;colon"));
        assert!(!valid_hostname("under_score"));
        assert!(!valid_hostname(
            "way-too-long-012345678901234567890123456789012345678901234567890123456789"
        ));
    }
}

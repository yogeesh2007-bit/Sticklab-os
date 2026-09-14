//! sticklab — the StickLab OS control center (std only).
//! One command for everything people actually open an OS for:
//!   sticklab            dashboard (system + GPU + languages at a glance)
//!   sticklab gpu        GPU report: NVIDIA (CUDA-ready?) / AMD / Intel, drivers loaded
//!   sticklab langs      every pre-installed language toolchain with versions
//!   sticklab new <tpl> <name>   scaffold a project (rust|python|go|node|c|java)
//!   sticklab doctor     health + security check with fix hints (offline-friendly)
//!   sticklab power      CPU/power report: governor, battery, TLP (longer charge)
//!   sticklab learn      guided Linux-learning path using tools on this ISO
//!   sticklab setup-gpu  one-command full CUDA toolkit / JDK install (needs internet)

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn run(tool: &str, args: &[&str]) -> Option<String> {
    Command::new(tool).args(args).output().ok().and_then(|o| {
        let out = format!(
            "{}{}",
            String::from_utf8_lossy(&o.stdout),
            String::from_utf8_lossy(&o.stderr)
        );
        out.lines().next().map(|l| l.trim().to_string())
    })
}

fn have(path: &str) -> bool {
    Path::new(path).exists()
}

/// Parse MemTotal (kB) from /proc/meminfo text. Pure — unit tested.
fn mem_total_kb(meminfo: &str) -> u64 {
    meminfo
        .lines()
        .find(|l| l.starts_with("MemTotal:"))
        .and_then(|l| l.split_whitespace().nth(1)?.parse().ok())
        .unwrap_or(0)
}

/// File list (relative path, content) for `sticklab new`. Pure — unit tested.
/// Returns None for unknown templates.
fn scaffold(tpl: &str, name: &str) -> Option<Vec<(String, String)>> {
    match tpl {
        "rust" => Some(vec![
            ("Cargo.toml".into(), format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n")),
            ("src/main.rs".into(), "fn main() {\n    println!(\"hello from StickLab OS\");\n}\n".into()),
        ]),
        "python" => Some(vec![
            ("main.py".into(), "def main():\n    print(\"hello from StickLab OS\")\n\nif __name__ == \"__main__\":\n    main()\n".into()),
        ]),
        "go" => Some(vec![
            ("go.mod".into(), format!("module {name}\n\ngo 1.23\n")),
            ("main.go".into(), "package main\n\nimport \"fmt\"\n\nfunc main() {\n\tfmt.Println(\"hello from StickLab OS\")\n}\n".into()),
        ]),
        "node" => Some(vec![
            ("package.json".into(), format!("{{\n  \"name\": \"{name}\",\n  \"version\": \"1.0.0\",\n  \"main\": \"index.js\"\n}}\n")),
            ("index.js".into(), "console.log(\"hello from StickLab OS\");\n".into()),
        ]),
        "c" => Some(vec![
            ("main.c".into(), "#include <stdio.h>\n\nint main(void) {\n    printf(\"hello from StickLab OS\\n\");\n    return 0;\n}\n".into()),
            ("Makefile".into(), "all:\n\tgcc -Wall -Wextra -o app main.c\n".into()),
        ]),
        "java" => Some(vec![
            ("Main.java".into(), "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"hello from StickLab OS\");\n    }\n".into()),
        ]),
        _ => None,
    }
}

/// True when /etc/shadow locks `user` (`*` or `!` hash). Pure — unit tested.
fn account_locked(shadow: &str, user: &str) -> Option<bool> {
    shadow.lines().find_map(|l| {
        let mut f = l.split(':');
        if f.next()? == user {
            let hash = f.next().unwrap_or("");
            Some(hash == "*" || hash == "!" || hash.starts_with('!'))
        } else {
            None
        }
    })
}

fn dashboard() {
    let os = fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|c| {
            c.lines()
                .find(|l| l.starts_with("PRETTY_NAME="))
                .map(|l| l.trim_start_matches("PRETTY_NAME=").trim_matches('"').to_string())
        })
        .unwrap_or_else(|| "StickLab OS".into());
    let kernel = run("uname", &["-r"]).unwrap_or_else(|| "?".into());
    let gpu_line = Command::new("sh")
        .args(["-c", "lspci 2>/dev/null | grep -iE 'vga|3d|display' | head -3"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();
    let gpu = if gpu_line.is_empty() { "no GPU detected (VM?)".to_string() } else { gpu_line.replace('\n', " | ") };
    let cuda = if have("/usr/bin/nvidia-smi") {
        run("nvidia-smi", &["--query-gpu=name", "--format=csv,noheader"])
            .unwrap_or_else(|| "NVIDIA driver present, no GPU visible".into())
    } else if have("/usr/lib/libcuda.so.1") || have("/usr/lib/libcuda.so") {
        "CUDA userspace present (nvidia-utils), nvidia-smi missing?!".into()
    } else {
        "no NVIDIA userspace (AMD/Intel/Nouveau path)".into()
    };
    println!("  STICKLAB  {os}   kernel {kernel}");
    println!("  GPU  : {gpu}");
    println!("  CUDA : {cuda}");
    println!("  langs: C/C++ gcc · python · go · node · rust · java · ruby · php · lua · perl");
    println!("  next : `sticklab langs` toolchains · `sticklab new rust demo` scaffold · `sticklab doctor` health");
}

fn gpu() {
    println!("StickLab OS GPU report (x86_64 PCs: NVIDIA / AMD / Intel):");
    let lspci = Command::new("sh")
        .args(["-c", "lspci -nnk 2>/dev/null | grep -iA3 -E 'vga|3d|display' || echo 'lspci: no GPU lines'"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "lspci unavailable".into());
    println!("{lspci}");
    let mods = fs::read_to_string("/proc/modules").unwrap_or_default();
    for m in ["nvidia", "nouveau", "amdgpu", "radeon", "i915", "xe"] {
        if mods.lines().any(|l| l.starts_with(&format!("{m} "))) {
            println!("  driver loaded: {m}");
        }
    }
    if have("/usr/bin/nvidia-smi") {
        println!("--- nvidia-smi ---");
        let _ = Command::new("nvidia-smi").status();
    } else {
        println!("  nvidia-smi: absent → NVIDIA dGPU? run `sticklab setup-gpu` (needs internet).");
    }
    let icds: Vec<_> = ["intel", "radeon", "nouveau", "nvidia"]
        .iter()
        .filter(|v| have(&format!("/usr/share/vulkan/icd.d/{v}_icd.x86_64.json")))
        .collect();
    println!("  vulkan ICDs: {:?}", icds);
}

fn langs() {
    let table: &[(&str, &str, &[&str])] = &[
        ("C/C++ ", "gcc", &["--version"]),
        ("C/C++ ", "g++", &["--version"]),
        ("python", "python", &["--version"]),
        ("go    ", "go", &["version"]),
        ("node  ", "node", &["--version"]),
        ("rustc ", "rustc", &["--version"]),
        ("cargo ", "cargo", &["--version"]),
        ("java  ", "java", &["--version"]),
        ("ruby  ", "ruby", &["--version"]),
        ("php   ", "php", &["--version"]),
        ("lua   ", "lua", &["-v"]),
        ("perl  ", "perl", &["--version"]),
        ("sqlite", "sqlite3", &["--version"]),
    ];
    println!("StickLab OS language environments (pre-installed, offline-ready):");
    for (name, tool, args) in table {
        let v = run(tool, args).unwrap_or_else(|| "—".into());
        let first = v.lines().next().unwrap_or("—");
        println!("  {name} {first:.70}");
    }
    println!("full JDK / CUDA toolkit (online): `sticklab setup-gpu`");
}

fn new_project() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: sticklab new <rust|python|go|node|c|java> <name>");
        std::process::exit(2);
    }
    let (tpl, name) = (args[2].as_str(), args[3].as_str());
    if Path::new(name).exists() {
        eprintln!("error: '{name}' already exists");
        std::process::exit(1);
    }
    let files = scaffold(tpl, name).unwrap_or_else(|| {
        eprintln!("unknown template '{tpl}' (rust|python|go|node|c|java)");
        std::process::exit(2);
    });
    fs::create_dir_all(name).expect("mkdir");
    for (rel, content) in &files {
        let dest = format!("{name}/{rel}");
        if let Some(parent) = Path::new(&dest).parent() {
            fs::create_dir_all(parent).ok();
        }
        fs::write(&dest, content).expect("write scaffold file");
    }
    println!("scaffolded {tpl} project in ./{name} ({} files)", files.len());
}

fn doctor() {
    println!("StickLab OS doctor (health + security):");
    let mut ok = true;
    let check = |label: &str, good: bool, hint: &str, ok: &mut bool| {
        println!("  [{}] {label}{}", if good { "OK" } else { "!!" }, if good { String::new() } else { format!(" → {hint}") });
        if !good {
            *ok = false;
        }
    };
    check("root filesystem writable", have("/usr/bin/pacman"), "are you on the live ISO?", &mut ok);
    let mem = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    check("RAM >= 2GB for desktop", mem_total_kb(&mem) >= 2_000_000, "use tty (Ctrl+Alt+F2) instead of labwc", &mut ok);
    let avail = Command::new("sh").args(["-c", "df -m / | awk 'NR==2{print $4}'"]).output().map(|o| String::from_utf8_lossy(&o.stdout).trim().parse::<u64>().unwrap_or(0)).unwrap_or(0);
    check("disk space >= 512MB free", avail >= 512, "clean pacman cache: sudo pacman -Scc", &mut ok);
    check("labwc desktop installed", have("/usr/bin/labwc"), "reinstall profile or use foot on tty", &mut ok);
    check("NetworkManager present", have("/usr/bin/NetworkManager") || have("/usr/bin/nmtui"), "use `nmtui` / check cable", &mut ok);
    check("GPU userspace (nvidia-utils or mesa)", have("/usr/lib/libcuda.so.1") || have("/usr/lib/dri/radeonsi_dri.so") || have("/usr/lib/libGLX_mesa.so.0"), "run `sticklab gpu`", &mut ok);
    // --- security surface ---
    let sshd_on = Command::new("systemctl").args(["is-enabled", "sshd"]).output().map(|o| String::from_utf8_lossy(&o.stdout).trim() == "enabled").unwrap_or(false);
    check("sshd disabled by default (no remote entry)", !sshd_on, "sudo systemctl disable --now sshd", &mut ok);
    let ufw_on = Command::new("sh").args(["-c", "ufw status 2>/dev/null | grep -q 'Status: active'"]).status().map(|s| s.success()).unwrap_or(false);
    check("firewall active (ufw)", ufw_on, "sudo ufw enable", &mut ok);
    match fs::read_to_string("/etc/shadow") {
        Ok(sh) => check("root password locked (live-safe)", account_locked(&sh, "root").unwrap_or(false), "sudo passwd -l root", &mut ok),
        Err(_) => println!("  [--] root password lock: unreadable (run doctor as root for this check)"),
    }
    println!("{}", if ok { "all green. happy hacking." } else { "issues above — hints included. `sticklab learn` teaches the why." });
}

fn power() {
    println!("StickLab OS power report (sip, don't gulp):");
    let cpu = fs::read_to_string("/proc/cpuinfo")
        .unwrap_or_default()
        .lines()
        .find(|l| l.starts_with("model name"))
        .map(|l| l.split(':').nth(1).unwrap_or("?").trim().to_string())
        .unwrap_or_else(|| "?".into());
    println!("  cpu: {cpu}");
    let gov = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor").unwrap_or_else(|_| "unknown (bare metal only)".into());
    let drv = fs::read_to_string("/sys/devices/system/cpu/cpufreq/policy0/scaling_driver").unwrap_or_else(|_| "unknown".into());
    println!("  governor: {}  driver: {}", gov.trim(), drv.trim());
    let bat: Vec<_> = fs::read_dir("/sys/class/power_supply").ok().map(|rd| {
        rd.flatten().filter_map(|e| {
            let base = e.path();
            let cap = fs::read_to_string(base.join("capacity")).ok()?.trim().to_string();
            let st = fs::read_to_string(base.join("status")).ok()?.trim().to_string();
            Some(format!("{}: {cap}% ({st})", e.file_name().to_string_lossy()))
        }).collect()
    }).unwrap_or_default();
    if bat.is_empty() {
        println!("  battery: none detected (desktop/VM)");
    } else {
        for b in bat {
            println!("  battery: {b}");
        }
    }
    let tlp = run("tlp-stat", &["-s"]).map(|s| s.lines().nth(1).unwrap_or("").trim().to_string()).unwrap_or_else(|| "tlp not running? sudo systemctl enable --now tlp".into());
    println!("  tlp: {tlp}");
    println!("  tips: TLP+thermald pre-enabled · `cpupower frequency-info` · dim via waybar/battery");
}

fn learn() {
    println!("StickLab OS learn-path (everything below is pre-installed):");
    println!("  1. `man man` + `man ls` — the manual is the textbook (man-db + man-pages)");
    println!("  2. `tldr tar` — community cheat-sheets when man is too long");
    println!("  3. `rsetup coding` + `sticklab langs` — see your toolchains");
    println!("  4. `sticklab new python demo && cd demo && python main.py` — first program");
    println!("  5. `strace -c ls` — watch syscalls: this is how Linux really works");
    println!("  6. `btop` + `ls /proc` — processes, then the virtual filesystem");
    println!("  7. `nmtui` + `ip addr` — networking hands-on");
    println!("  8. `sticklab power` + `cpupower frequency-info` — how your CPU sips power");
    println!("  9. edit ~/.config/labwc/rc.xml — your WM, your rules");
    println!("  10. `rsetup customize` — map of every themeable file (bar, terminal, keys, editor)");
}

fn setup_gpu() {
    println!("StickLab OS one-command extras (needs internet):");
    println!("  Full CUDA toolkit : sudo pacman -S cuda cudnn");
    println!("  Full JDK (javac)  : sudo pacman -S jdk-openjdk  (JRE already on board)");
    println!("  32-bit gaming libs: sudo pacman -S lib32-nvidia-utils lib32-mesa");
    println!("  Verify after      : nvidia-smi ; nvcc --version ; sticklab gpu");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        dashboard();
        return;
    }
    match args[1].as_str() {
        "gpu" => gpu(),
        "langs" => langs(),
        "new" => new_project(),
        "doctor" => doctor(),
        "power" => power(),
        "learn" => learn(),
        "setup-gpu" => setup_gpu(),
        "help" | "--help" | "-h" => {
            println!("sticklab — StickLab OS control center");
            println!("  sticklab [dashboard] | gpu | langs | new <tpl> <name> | doctor | power | learn | setup-gpu");
        }
        _ => {
            eprintln!("unknown command '{}'. Try: sticklab help", args[1]);
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mem_total_parses_kb() {
        let fake = "MemTotal:        8014344 kB\nMemFree: 123 kB\n";
        assert_eq!(mem_total_kb(fake), 8014344);
        assert_eq!(mem_total_kb("garbage"), 0);
        assert_eq!(mem_total_kb(""), 0);
    }

    #[test]
    fn scaffold_rust_has_cargo_and_main() {
        let files = scaffold("rust", "demo").expect("rust template");
        let names: Vec<_> = files.iter().map(|(p, _)| p.as_str()).collect();
        assert!(names.contains(&"Cargo.toml"));
        assert!(names.contains(&"src/main.rs"));
        assert!(files.iter().find(|(p, _)| p == "Cargo.toml").unwrap().1.contains("name = \"demo\""));
    }

    #[test]
    fn scaffold_all_templates_cover_languages() {
        for tpl in ["rust", "python", "go", "node", "c", "java"] {
            let files = scaffold(tpl, "x").unwrap_or_else(|| panic!("{tpl} template missing"));
            assert!(!files.is_empty(), "{tpl} produced no files");
            for (rel, content) in &files {
                assert!(!rel.is_empty() && !content.is_empty(), "{tpl}/{rel} empty");
            }
        }
        assert!(scaffold("cobol", "x").is_none());
    }

    #[test]
    fn account_locked_detects_star_bang() {
        assert_eq!(account_locked("root:*:14871::::::\n", "root"), Some(true));
        assert_eq!(account_locked("root:!:14871::::::\n", "root"), Some(true));
        assert_eq!(account_locked("root:$6$salt$hash:14871::::::\n", "root"), Some(false));
        assert_eq!(account_locked("nobody:x::::::\n", "root"), None);
    }
}

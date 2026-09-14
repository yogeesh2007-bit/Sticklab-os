# ⚡ StickLab OS

> **A lightweight, ready-to-code Arch-based Linux that fits on a 2GB USB stick — with a Rust-native control center, every major language pre-installed, CUDA-ready NVIDIA support, and a customizable Labwc desktop.**
>
> Created and built by **YOGEESHWARAN C**.

```
 ███████╗ ████████╗ ██╗  ██████╗ ██╗  ██╗ ██╗       █████╗  ██████╗  
 ██╔════╝ ╚══██╔══╝ ██║ ██╔════╝ ██║ ██╔╝ ██║      ██╔══██╗ ██╔══██╗ 
 ███████╗    ██║    ██║ ██║      █████╔╝  ██║      ███████║ ██████╔╝ 
 ╚════██║    ██║    ██║ ██║      ██╔═██╗  ██║      ██╔══██║ ██╔══██╗ 
 ███████║    ██║    ██║ ╚██████╗ ██║  ██╗ ███████╗ ██║  ██║ ██████╔╝ 
 ╚══════╝    ╚═╝    ╚═╝  ╚═════╝ ╚═╝  ╚═╝ ╚══════╝ ╚═╝  ╚═╝ ╚═════╝  
         ⚡  ·  sticklab os  ·  ★
```

[![Built with Arch](https://img.shields.io/badge/base-Arch%20Linux-1793D1?logo=archlinux&logoColor=white)](https://archlinux.org)
[![Rust core](https://img.shields.io/badge/core-Rust-orange?logo=rust)](rust/)
[![Size](https://img.shields.io/badge/image-%3C2GB-7d63ce)](out/)
[![Desktop](https://img.shields.io/badge/desktop-Labwc%20%2B%20Waybar-b4befe)](archiso/airootfs/etc/skel/.config/)

---

## ✨ Why StickLab OS?

Most tiny distros make you choose: **small** *or* **ready**. StickLab OS refuses:

| What you want in an OS | How StickLab OS delivers |
|---|---|
| Runs on **every PC** | Linux kernel + full firmware set, Intel/AMD microcode, BIOS **and** UEFI boot, Intel/AMD/NVIDIA GPUs |
| **CUDA-ready** | NVIDIA open modules + userspace on board (`nvidia-smi` works); full toolkit one command away |
| All **languages pre-installed** | C/C++, Python, Go, Node, **Rust**, Java, Ruby, PHP, Lua, Perl — offline, verified by `sticklab langs` |
| Modern **coder dependencies** | git, cmake, ninja, curl, gh, just, man pages, tldr, btop, fzf, ripgrep… |
| **Cool + customizable**, not opinionated | Labwc stacking desktop (Openbox-style), Waybar, Wofi, Foot — every config plain text in `~/.config` |
| Learn **Linux properly** | `sticklab learn` guided path: man → tldr → syscalls → `/proc` → networking, all tools on board |
| **Longer battery** | TLP + thermald pre-enabled, `sticklab power` shows governor/battery/CPU |
| **Secure by default** | Locked root, sshd off, firewall on, no listening services — `sticklab doctor` audits it |
| One **novelty no other OS has** | ⚡ `sticklab` — a single Rust control center for system, GPU, languages, scaffolding, health |

---

## 🚀 Get StickLab OS

### Option A — Flash to USB (real hardware, BIOS or UEFI)

```bash
lsblk                      # find your stick, e.g. /dev/sdX — TRIPLE CHECK
sudo dd if=sticklab-os-usb.img of=/dev/sdX bs=4M status=progress oflag=sync
```

Or use [balenaEtcher](https://etcher.balena.io/) / [Ventoy](https://www.ventoy.net/) with the `.iso`. Reboot → boot menu → StickLab OS live session (root autologin on tty1, Labwc desktop).

### Option B — Virtual machine

```bash
./scripts/run-vm.sh bios    # or: uefi  (needs edk2-ovmf)
```

Works in QEMU/KVM, VirtualBox, VMware. Give it **≥3GB RAM** (NVIDIA userspace is heavy).

### Option C — WSL2 (Windows terminal)

```bash
./scripts/mk-wsl-rootfs.sh                       # builds out/sticklab-os-wsl.tar.gz
wsl --import StickLabOS C:\WSL\StickLabOS out/sticklab-os-wsl.tar.gz
wsl -d StickLabOS
```

Same Arch base, same languages, same `sticklab` — minus kernel/GUI (WSL2 supplies the kernel).

### Option D — Docker / terminal

```bash
docker import out/sticklab-os-wsl.tar.gz sticklab-os:wsl
docker run -it sticklab-os:wsl bash
```

---

## ⚡ The `sticklab` control center

The one thing other OSes lack: a single fast offline command for the whole system. Written in Rust, zero dependencies.

```bash
sticklab              # dashboard: OS, kernel, GPU, CUDA status, languages
sticklab gpu          # GPU report — NVIDIA/AMD/Intel, loaded drivers, Vulkan ICDs
sticklab langs        # every language toolchain with versions
sticklab new rust demo      # scaffold rust|python|go|node|c|java project
sticklab doctor       # health + SECURITY audit with fix hints
sticklab power        # CPU governor, battery, TLP — stretch that charge
sticklab learn        # guided Linux learning path
sticklab setup-gpu    # one-command full CUDA toolkit / JDK install (online)
sticklab help
```

Plus three companions in `/usr/local/bin`: `rinit` (boot helper), `rfetch` (the logo above ⚡), `rsetup` (first-boot: hostname, services).

---

## ⚖️ StickLab OS vs the others — both sides

No distro wins at everything. Here's the honest version.

### Where StickLab OS wins

| Against | The difference |
|---|---|
| Stock **Arch** | Same base, zero setup: 10 language toolchains, CUDA-ready NVIDIA userspace, TLP + thermald, firewall, and a desktop — preinstalled and verified by `sticklab doctor`. Stock Arch gives you a prompt and a wiki page. |
| **Ubuntu / Fedora** (~5GB ISOs, install-first) | Whole lab fits on a **2GB USB stick**, boots live on any BIOS/UEFI PC with nothing to install. One `sticklab` command replaces a dozen disconnected tools. |
| Dev distros (**SemiCode OS**, **GenesiOS**) | Smaller (~1.7GB vs 3–10GB), terminal-first instead of a heavy GNOME/KDE desktop, and every config is plain text you can read — it's also a *teaching* OS (`sticklab learn`), not just a tool dump. |
| Portable USB sellers / **Tails** | Tails buys amnesia at the cost of a usable dev environment. StickLab OS is the opposite trade: a full offline coder's lab (compilers, CUDA userspace, man pages, `strace`) on the same live-USB idea. |

### Where the others win (real weaknesses, no spin)

- **One maintainer vs companies/communities.** Ubuntu has Canonical, Fedora has Red Hat, Arch has thousands of packagers. StickLab OS has YOGEESHWARAN C. Security response, hardware coverage, and bug fixes will be slower. That's the price of a hand-built distro.
- **No Secure Boot.** Ubuntu/Fedora boot with Secure Boot on; StickLab OS (plain archiso) requires disabling it in firmware. A non-starter on locked-down lab/office machines.
- **Live ISO goes stale.** Ubuntu/Fedora update in place; Fedora Atomic even rolls back. StickLab OS has no updater or persistence layer — reboot the live stick and your work is gone unless you save it elsewhere. Rebuild the ISO to get fresh packages.
- **No signed checksums yet.** Ubuntu publishes signed hashes per release; StickLab OS doesn't (see Roadmap). Download-and-trust is weaker — verify the repo and build it yourself if that matters to you.
- **2GB budget means cuts.** No full IDE (VS Code), no office suite, no app store, JRE-only Java, and the full CUDA toolkit/JDK are a download away (`sticklab setup-gpu`) — not on the stick.
- **Minimal desktop, minimal hand-holding.** Labwc + Waybar is fast and light, but there's no graphical installer (only `archinstall`), no driver GUI, no settings app. GNOME/KDE users will feel the gap.
- **x86_64 PCs only.** No Raspberry Pi / ARM build yet. Old pre-Turing NVIDIA cards aren't covered by the open kernel modules either (Nouveau fallback).
- **`sticklab` knowledge doesn't transfer.** It's the fastest tool here and useless everywhere else — unlike `git`, `pacman`, or shell skills, which travel with you.

### When NOT to use StickLab OS

- Daily-driver desktop → **Fedora / Ubuntu** (updates, app ecosystem, polish).
- Gaming → **Bazzite / Nobara / Pop!_OS** (Steam, Proton, gamer drivers).
- Amnesic/anonymous sessions → **Tails** (that job needs Tor + forced amnesia, not compilers).
- Servers / fleets → **Debian / RHEL / NixOS** (unattended upgrades, Secure Boot, ARM, ten-year support).
- Locked-down school/office PCs → anything with signed Secure Boot support.

StickLab OS is for: **a coder's whole lab on a 2GB stick that boots anywhere, teaches Linux honestly, and tells you what it can't do.**

---

## 🖥️ Desktop & first boot

- **tty1** → Labwc desktop auto-starts (Waybar top bar, dark lavender theme)
- **Bolt+Return** terminal · **Bolt+D** launcher · **right-click** menu · **Bolt+Q** close
- **Bolt = the Alt key.** Super is deliberately *not* used, so a host OS (like Omarchy) keeps it when you run StickLab OS in a VM — no key fights.
- Other ttys stay plain console — perfect for learning
- Make it yours: `~/.config/labwc/rc.xml`, `~/.config/waybar/`, `~/.config/wofi/`, `~/.config/foot/foot.ini`, `~/.config/nvim/init.lua`, `~/.zshrc`

---

## 🎨 Make it yours (customization-first)

Every default config is commented, example-loaded, and safe to break — restore by deleting it (skel defaults return on a fresh boot). On the live system, run `rsetup customize` for this map anytime:

| Want to change | File | How |
|---|---|---|
| Keybindings (Bolt = Alt) | `~/.config/labwc/rc.xml` | Copy a line, change `A-x`, run `labwc --reconfigure`. Screenshot (`Bolt+S`) + lock (`Bolt+L`) included |
| Right-click menu | `~/.config/labwc/menu.xml` | Add `<item>` blocks; a Customize entry is built in |
| Wallpaper / autostart apps | `~/.config/labwc/autostart` | Swap the `swaybg` color for `-i ~/Pictures/wall.png`, uncomment extras |
| Top bar layout + theme | `~/.config/waybar/config.jsonc` + `style.css` | Reorder modules, move bar to `bottom`, swap accent `#b4befe` → mint/peach/red (reload: `pkill -USR2 waybar`) |
| App launcher | `~/.config/wofi/config` + `style.css` | Size, prompt (try `prompt=⚡`), same accent swap |
| Terminal font + colors | `~/.config/foot/foot.ini` | Font size, 16 colors, ready-made accent pairs in comments |
| Editor | `~/.config/nvim/init.lua` | Cheat sheet + example keymaps + plugin-manager pointer in comments |
| Shell prompt + aliases | `~/.zshrc`, `~/.bashrc` | Both shells work; both files say exactly which line to touch |

Rules of the game: **Bolt (Alt)** is the OS modifier — Super is left for your host OS. Accent color lives in exactly two files (`foot.ini`, `waybar`/`wofi` CSS). Nothing is compiled, nothing needs a settings daemon.

---

## 🔒 Security model

| Surface | Default | Why |
|---|---|---|
| Root password | **locked** (`*`) | No password login possible |
| Live session | root autologin, physical console only | Standard live-ISO pattern; installed systems use `archinstall` + own users |
| SSH daemon | **disabled** | No remote entry; enable explicitly: `systemctl enable --now sshd` |
| Firewall | **ufw enabled**, deny incoming | Audited by `sticklab doctor` |
| Verification | `cargo test` (12 unit tests), ISO content audit, `systemd-nspawn` boot test | See [TESTING](TESTING.md) |

---

## 🔋 Power efficiency

TLP + thermald are enabled out of the box: powersave governors, Wi-Fi power save, runtime PM for PCI/USB. `sticklab power` shows your CPU, governor (`intel_pstate`/`amd-pstate`), battery %, and TLP state; `cpupower frequency-info` digs deeper.

---

## 🛠️ Build from source (Arch/Omarchy host)

```bash
git clone https://github.com/yogeesh2007-bit/sticklab-os.git && cd sticklab-os
./scripts/install-deps.sh   # archiso, pacstrap, squashfs tools…
./build.sh                  # Rust → stage → mkarchiso → out/sticklab-os-usb.img
```

`build.sh` in 4 stages: **(1)** `cargo build --release`, **(2)** stage bins into `archiso/airootfs`, **(3)** `mkarchiso` (needs sudo, ~700MB downloads, 15–30 min), **(4)** ISO → bootable USB `.img` + 2GB budget check.

### Project layout

```
sticklab-os/
├── archiso/                 mkarchiso profile
│   ├── profiledef.sh        ISO name, BIOS+UEFI bootmodes, xz squashfs
│   ├── packages.x86_64      ~110 packages: base, toolchains, GPU, Labwc
│   ├── pacman.conf          clean core+extra repos
│   ├── syslinux/ efiboot/   boot menus (StickLab OS branded)
│   └── airootfs/            live filesystem overlay
│       ├── usr/local/bin/   staged Rust bins (rinit, rfetch, rsetup, sticklab)
│       ├── etc/systemd/…    rinit.service, autologin, enabled services
│       └── etc/skel/        desktop + shell + editor defaults for every user
├── rust/                    Cargo workspace (std-only, zero deps)
│   └── src/bin/             rinit.rs, rfetch.rs, rsetup.rs, sticklab.rs (+tests)
├── scripts/                 install-deps, mk-usb-img, run-vm, mk-wsl-rootfs
├── build.sh                 full pipeline
└── out/                     ISO + USB .img + WSL tarball (git-ignored)
```

### Tests

```bash
cargo test                   # 12 unit tests: scaffolder, mem parse, shadow lock, colors
./scripts/mk-usb-img.sh out/*.iso   # verifies 2GB budget
```

---

## 🗺️ Roadmap

- [ ] `sticklab` shell completions + `sticklab theme` switcher (Catppuccin variants)
- [ ] Signed ISO checksums published per release
- [ ] ARM64 (Raspberry Pi) profile
- [ ] Optional full-CUDA edition (4GB USB tier)

---

## 👤 Credits

**StickLab OS — created, designed, and built by YOGEESHWARAN C.**
Rust control center, desktop theme, package curation, and docs — all crafted for coders and Linux learners.

## 📜 License & credits

Build scripts + Rust userspace: **MIT** (see [LICENSE](LICENSE)). ISO contents follow their upstream Arch package licenses. Based on [Arch Linux](https://archlinux.org) + [archiso](https://wiki.archlinux.org/title/Archiso). Not affiliated with Arch.

*Your lab on a stick.* ⚡ — by YOGEESHWARAN C

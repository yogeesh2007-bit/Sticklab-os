# 🌙 Night OS

> **A lightweight, ready-to-code Arch-based Linux that fits on a 2GB USB stick — with a Rust-native control center, every major language pre-installed, CUDA-ready NVIDIA support, and a customizable Labwc desktop.**
>
> Created and built by **YOGEESHWARAN C**.

```
 ███╗   ██╗██╗ ██████╗ ██╗  ██╗████████╗
 ████╗  ██║██║██╔════╝ ██║  ██║╚══██╔══╝
 ██╔██╗ ██║██║██║  ███╗███████║   ██║
 ██║╚██╗██║██║██║   ██║██╔══██║   ██║
 ██║ ╚████║██║╚██████╔╝██║  ██║   ██║
 ╚═╝  ╚═══╝╚═╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝
        ☾  ·  night os  ·  ★
```

[![Built with Arch](https://img.shields.io/badge/base-Arch%20Linux-1793D1?logo=archlinux&logoColor=white)](https://archlinux.org)
[![Rust core](https://img.shields.io/badge/core-Rust-orange?logo=rust)](rust/)
[![Size](https://img.shields.io/badge/image-%3C2GB-7d63ce)](out/)
[![Desktop](https://img.shields.io/badge/desktop-Labwc%20%2B%20Waybar-b4befe)](archiso/airootfs/etc/skel/.config/)

---

## ✨ Why Night OS?

Most tiny distros make you choose: **small** *or* **ready**. Night OS refuses:

| What you want in an OS | How Night OS delivers |
|---|---|
| Runs on **every PC** | Linux kernel + full firmware set, Intel/AMD microcode, BIOS **and** UEFI boot, Intel/AMD/NVIDIA GPUs |
| **CUDA-ready** | NVIDIA open modules + userspace on board (`nvidia-smi` works); full toolkit one command away |
| All **languages pre-installed** | C/C++, Python, Go, Node, **Rust**, Java, Ruby, PHP, Lua, Perl — offline, verified by `night langs` |
| Modern **coder dependencies** | git, cmake, ninja, curl, gh, just, man pages, tldr, btop, fzf, ripgrep… |
| **Cool + customizable**, not opinionated | Labwc stacking desktop (Openbox-style), Waybar, Wofi, Foot — every config plain text in `~/.config` |
| Learn **Linux properly** | `night learn` guided path: man → tldr → syscalls → `/proc` → networking, all tools on board |
| **Longer battery** | TLP + thermald pre-enabled, `night power` shows governor/battery/CPU |
| **Secure by default** | Locked root, sshd off, firewall on, no listening services — `night doctor` audits it |
| One **novelty no other OS has** | 🌙 `night` — a single Rust control center for system, GPU, languages, scaffolding, health |

---

## 🚀 Get Night OS

### Option A — Flash to USB (real hardware, BIOS or UEFI)

```bash
lsblk                      # find your stick, e.g. /dev/sdX — TRIPLE CHECK
sudo dd if=night-os-usb.img of=/dev/sdX bs=4M status=progress oflag=sync
```

Or use [balenaEtcher](https://etcher.balena.io/) / [Ventoy](https://www.ventoy.net/) with the `.iso`. Reboot → boot menu → Night OS live session (root autologin on tty1, Labwc desktop).

### Option B — Virtual machine

```bash
./scripts/run-vm.sh bios    # or: uefi  (needs edk2-ovmf)
```

Works in QEMU/KVM, VirtualBox, VMware. Give it **≥3GB RAM** (NVIDIA userspace is heavy).

### Option C — WSL2 (Windows terminal)

```bash
./scripts/mk-wsl-rootfs.sh                       # builds out/night-os-wsl.tar.gz
wsl --import NightOS C:\WSL\NightOS out/night-os-wsl.tar.gz
wsl -d NightOS
```

Same Arch base, same languages, same `night` — minus kernel/GUI (WSL2 supplies the kernel).

### Option D — Docker / terminal

```bash
docker import out/night-os-wsl.tar.gz night-os:wsl
docker run -it night-os:wsl bash
```

---

## 🌙 The `night` control center

The one thing other OSes lack: a single fast offline command for the whole system. Written in Rust, zero dependencies.

```bash
night              # dashboard: OS, kernel, GPU, CUDA status, languages
night gpu          # GPU report — NVIDIA/AMD/Intel, loaded drivers, Vulkan ICDs
night langs        # every language toolchain with versions
night new rust demo      # scaffold rust|python|go|node|c|java project
night doctor       # health + SECURITY audit with fix hints
night power        # CPU governor, battery, TLP — stretch that charge
night learn        # guided Linux learning path
night setup-gpu    # one-command full CUDA toolkit / JDK install (online)
night help
```

Plus three companions in `/usr/local/bin`: `rinit` (boot helper), `rfetch` (the logo above ☾), `rsetup` (first-boot: hostname, services).

---

## 🖥️ Desktop & first boot

- **tty1** → Labwc desktop auto-starts (Waybar top bar, dark lavender theme)
- **Super+Return** terminal · **Super+D** launcher · **right-click** menu · **Super+Q** close
- Other ttys stay plain console — perfect for learning
- Make it yours: `~/.config/labwc/rc.xml`, `~/.config/waybar/`, `~/.config/wofi/`, `~/.config/foot/foot.ini`, `~/.config/nvim/init.lua`, `~/.zshrc`

---

## 🔒 Security model

| Surface | Default | Why |
|---|---|---|
| Root password | **locked** (`*`) | No password login possible |
| Live session | root autologin, physical console only | Standard live-ISO pattern; installed systems use `archinstall` + own users |
| SSH daemon | **disabled** | No remote entry; enable explicitly: `systemctl enable --now sshd` |
| Firewall | **ufw enabled**, deny incoming | Audited by `night doctor` |
| Verification | `cargo test` (12 unit tests), ISO content audit, `systemd-nspawn` boot test | See [TESTING](TESTING.md) |

---

## 🔋 Power efficiency

TLP + thermald are enabled out of the box: powersave governors, Wi-Fi power save, runtime PM for PCI/USB. `night power` shows your CPU, governor (`intel_pstate`/`amd-pstate`), battery %, and TLP state; `cpupower frequency-info` digs deeper.

---

## 🛠️ Build from source (Arch/Omarchy host)

```bash
git clone https://github.com/yogeesh2007-bit/night-os.git && cd night-os
./scripts/install-deps.sh   # archiso, pacstrap, squashfs tools…
./build.sh                  # Rust → stage → mkarchiso → out/night-os-usb.img
```

`build.sh` in 4 stages: **(1)** `cargo build --release`, **(2)** stage bins into `archiso/airootfs`, **(3)** `mkarchiso` (needs sudo, ~700MB downloads, 15–30 min), **(4)** ISO → bootable USB `.img` + 2GB budget check.

### Project layout

```
night-os/
├── archiso/                 mkarchiso profile
│   ├── profiledef.sh        ISO name, BIOS+UEFI bootmodes, xz squashfs
│   ├── packages.x86_64      ~110 packages: base, toolchains, GPU, Labwc
│   ├── pacman.conf          clean core+extra repos
│   ├── syslinux/ efiboot/   boot menus (Night OS branded)
│   └── airootfs/            live filesystem overlay
│       ├── usr/local/bin/   staged Rust bins (rinit, rfetch, rsetup, night)
│       ├── etc/systemd/…    rinit.service, autologin, enabled services
│       └── etc/skel/        desktop + shell + editor defaults for every user
├── rust/                    Cargo workspace (std-only, zero deps)
│   └── src/bin/             rinit.rs, rfetch.rs, rsetup.rs, night.rs (+tests)
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

- [ ] `night` shell completions + `night theme` switcher (Catppuccin variants)
- [ ] Signed ISO checksums published per release
- [ ] ARM64 (Raspberry Pi) profile
- [ ] Optional full-CUDA edition (4GB USB tier)

---

## 👤 Credits

**Night OS — created, designed, and built by YOGEESHWARAN C.**
Rust control center, desktop theme, package curation, and docs — all crafted for coders and Linux learners.

## 📜 License & credits

Build scripts + Rust userspace: **MIT** (see [LICENSE](LICENSE)). ISO contents follow their upstream Arch package licenses. Based on [Arch Linux](https://archlinux.org) + [archiso](https://wiki.archlinux.org/title/Archiso). Not affiliated with Arch.

*Darkness, compiled.* ☾ — by YOGEESHWARAN C

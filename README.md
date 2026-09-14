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
[![Desktop](https://img.shields.io/badge/desktops-labwc%20%7C%20sway%20%7C%20hyprland-b4befe)](archiso/airootfs/etc/skel/.config/)

---

## 💡 Why StickLab OS exists

Most newcomers meet Linux in the worst way possible: as a bare installer that hands them a black screen and a wiki link, or as a 5GB download that still can't compile "hello world" offline. Students learn *about* Linux without ever getting a lab. Hardware tinkerers — the Arduino / ESP32 / STM32 / Pico crowd — have it worse: plug a board into a fresh Linux box and you get silence. No serial console, no flasher, no debugger, and a permissions maze before the first blink sketch.

StickLab OS was created to fix exactly that: **a complete laboratory on a 2GB USB stick that boots on any PC, works fully offline, welcomes first-time Linux users instead of hazing them, and treats hardware coders as first-class citizens** — serial, flashing, and debug tools on board, with `sticklab boards` naming whatever you plug in.

If you need it, it should already be there. If it teaches you, even better.

## ✨ Why StickLab OS?

Most tiny distros make you choose: **small** *or* **ready**. StickLab OS refuses:

| What you want in an OS | How StickLab OS delivers |
|---|---|
| Runs on **every PC** | Linux kernel + full firmware set, Intel/AMD microcode, BIOS **and** UEFI boot, Intel/AMD/NVIDIA GPUs |
| **CUDA-ready** | NVIDIA open modules + userspace on board (`nvidia-smi` works); full toolkit one command away |
| All **languages pre-installed** | C/C++, Python, Go, Node, **Rust** (via rustup — `rustup toolchain install stable`), Java, Ruby, PHP, Lua, Perl — offline except the one-time Rust toolchain fetch, verified by `sticklab langs` |
| **Hardware bench included** | Serial console, AVR/ARM flashing, DFU, I2C, logic capture on board; `sticklab boards` detects Uno, Nano, ESP32, STM32, Pico, Teensy, micro:bit — heavy compilers one command away |
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

### Option E — Dual-boot alongside Windows / any other OS (keeps both)

StickLab OS dual-boots like any Arch: the live ISO ships GRUB + `os-prober` + NTFS + partitioning tools, boots BIOS **and** UEFI, and the syslinux menu has a **Boot existing OS** entry so you never strand your other system.

```bash
sticklab dualboot   # audit first: UEFI/BIOS match, ESP, Secure Boot, other OSes found
```

Safe flow:

1. **In Windows first:** disable Fast Startup, suspend BitLocker, shrink `C:` to make free space (Disk Management), and note the existing FAT32 ESP. Back up anything you can't lose.
2. **Boot the stick in the SAME mode** as the other OS — UEFI with UEFI (Windows 10/11 installs are UEFI). Use the `UEFI:` USB entry in the firmware boot menu. Still boots alongside legacy-BIOS Linux installs too.
3. **Install:** run `archinstall` → manual partitioning → **reuse** the existing ESP as `/boot` (**do NOT format it**) → install StickLab OS into the **free space only** → bootloader = **GRUB** (it chainloads Windows/other Linux; plain systemd-boot won't list them).
4. **After install:** enable OS detection so the other entry appears:
   ```bash
   # in /etc/default/grub set:
   GRUB_DISABLE_OS_PROBER=false
   sudo grub-mkconfig -o /boot/grub/grub.cfg
   ```
5. Clock wrong in Windows afterwards? On the Linux side: `timedatectl set-local-rtc 1` (Windows keeps local time, Linux defaults to UTC).
6. Still boots only one OS? Re-run `sticklab dualboot` as root (encrypted/BitLocker volumes stay hidden until unlocked) and check Secure Boot is **off** — StickLab OS has no signed shim (see weaknesses below).

---

## ⚡ The `sticklab` control center

The one thing other OSes lack: a single fast offline command for the whole system. Written in Rust, zero dependencies.

```bash
sticklab              # dashboard: OS, kernel, GPU, CUDA status, languages
sticklab gpu          # GPU report — NVIDIA/AMD/Intel, loaded drivers, Vulkan ICDs
sticklab langs        # every language toolchain with versions
sticklab boards       # USB hardware bench: detect MCU boards + serial ports + flash tools
sticklab new rust demo      # scaffold rust|python|go|node|c|java project
sticklab doctor       # health + SECURITY audit with fix hints
sticklab power        # CPU governor, battery, TLP — stretch that charge
sticklab learn        # guided Linux learning path
sticklab dualboot     # dual-boot audit: UEFI/BIOS, ESP, Secure Boot, other OSes + install recipe
sticklab setup-gpu    # one-command full CUDA toolkit / JDK install (online)
sticklab setup-hardware  # one-command MCU toolchains: ARM, AVR, ESP, Pico (online)
sticklab help
```

Plus three companions in `/usr/local/bin`: `rinit` (boot helper), `rfetch` (the logo above ⚡), `rsetup` (first-boot: hostname, services).

---

## 🔌 Hardware coders (Arduino · ESP32 · STM32 · Pico · AVR)

Software isn't the whole story — most new coders touch hardware first. StickLab OS ships an offline workbench for them:

| Step | How |
|---|---|
| **Plug the board in** | USB-serial chips (CH340, CP2102, CH9102, FTDI, PL2303), ST-Link, DFU bootloaders — kernel drivers + firmware on board |
| **Detect it** | `sticklab boards` names the board (Uno, Nano, ESP32, STM32, Pico, Teensy, micro:bit…), shows its `/dev/ttyUSB*` port and the exact tool to use |
| **Talk to it** | `picocom -b 115200 /dev/ttyUSB0` serial console; `i2cdetect -l` for I2C buses; `sigrok-cli` for logic capture |
| **Flash it** | `avrdude` (AVR), `openocd` (ARM/STM32/RP2040 debug), `dfu-util` (STM32 DFU) — preinstalled, offline |
| **Go further (online)** | `sticklab setup-hardware` — ARM + AVR GCC toolchains, Arduino-CLI, esptool via pip, picotool (AUR), Rust MCU targets |
| **No permission maze** | Live session runs as root, so serial just works. Installed systems: `sudo usermod -aG uucp $USER` + relogin |

Heavy compilers stay a download away on purpose — the 2GB stick budget goes to the tools you need *before* you have internet.

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
| Stock Debian / Pi OS for **hardware tinkering** | No serial console, flasher, or debugger out of the box — and no idea what you just plugged in. StickLab OS ships `picocom`, `avrdude`, `openocd`, `dfu-util` plus `sticklab boards` board detection. |

### Complaints users report about other systems → what StickLab does about them

Collected from recurring user complaints (Reddit, forums, reviews, 2024–2026). Each row is a real grievance — and the concrete answer shipped on the ISO.

| The complaint (elsewhere) | How StickLab OS answers |
|---|---|
| **Ubuntu: snaps are slow to cold-start, hog disk/RAM, clutter `~/snap` + loop mounts, auto-refresh without asking, and `apt install firefox` secretly installs a snap** | **No snaps at all.** Pure Arch `pacman`, native packages only. No loop-device clutter, no forced refreshes, no wrapper-switcheroos. `pacman -S` means pacman. |
| **Ubuntu/Fedora full desktops feel heavy; old PCs and HDDs struggle (GNOME)** | **3 featherweight Wayland WMs** (Labwc stacking, Sway tiling, Hyprland dynamic) instead of a heavy DE. ~1.7GB image, TLP + thermald pre-enabled, `sticklab power` proves the sip. |
| **"New Linux users get a black screen + a wiki link" (stock Arch install: manual partitioning, bootloader, drivers, desktop — all by hand)** | Same Arch base, zero hazing: `archinstall` on board, desktop + network + firewall pre-wired, `sticklab learn` teaches man → tldr → syscalls → `/proc`, `rsetup` maps every config file. |
| **Arch rolling updates break systems; AUR malware wave (1000+ malicious packages, 2025–26) + outages punish the curious** | **Official repos only on the ISO** — no AUR dependency, no AUR helper preinstalled, so the malware/DDoS surface simply isn't on the stick. `sticklab doctor` audits health before you go exploring. |
| **Fedora: NVIDIA + codecs need extra third-party repos and how-tos; Secure Boot MOK screens ambush new users** | **CUDA-ready out of the box:** NVIDIA open modules + `nvidia-utils` preinstalled, `sticklab gpu` reports status, `sticklab setup-gpu` is the one documented next step. Firmware + Intel/AMD microcode on board. Secure Boot stance is stated upfront (off required) instead of a surprise MOK screen. |
| **Mint: base/kernel months behind (new GPUs suffer), X11-only with no mature Wayland (multi-monitor refresh, scaling, HDR broken), weak for gaming** | **Wayland-first, current Arch base:** 3 wlroots compositors with per-monitor configs, `xorg-xwayland` for legacy apps, new kernels/firmware via rolling base, Vulkan ICDs for AMD/Intel/Nouveau + NVIDIA userspace. |
| **Windows 11: forced updates that break things, ads/sponsored apps in Start, telemetry you can't fully off, forced Microsoft account + internet, Copilot that reinstalls itself, default BitLocker locking out dual-booters, clock skew after dual-boot** | **No telemetry, no accounts, no ads, no bundled AI, no forced anything.** No listening services (`sshd` off, firewall on, audited by `sticklab doctor`). Dual-boot is a first-class flow: GRUB + `os-prober` + NTFS on board, `sticklab dualboot` audits ESP/mode/Secure Boot and warns about Fast Startup, BitLocker-suspend, and the Windows clock fix. |
| **First-30-minutes pain on every fresh Linux install: Wi-Fi dead (Realtek/Broadcom), wrong audio device, stuck at 800×600** | `linux-firmware` + `sof-firmware` + NetworkManager + `iwd` + PipeWire preinstalled; `rsetup status` + `sticklab doctor` triage it in one screen. |
| **"Fresh distro, still can't compile hello-world offline" — languages/IDEs missing until you find internet** | 9 language toolchains + git/cmake/ninja/man/tldr preinstalled and verified by `sticklab langs` (Rust joins via one `rustup toolchain install stable`); `sticklab new rust demo` scaffolds offline. Heavy extras stay one documented command away (`setup-gpu`, `setup-hardware`) to protect the 2GB budget. |
| **"Plug an Arduino/ESP32/STM32/Pico into fresh Linux: silence" — no serial console, flasher, debugger, permission maze** | Hardware bench on board: `picocom`, `avrdude`, `openocd`, `dfu-util`, serial drivers + firmware, and `sticklab boards` names the plugged board + port + tool. |

### Where the others win (real weaknesses, no spin)

- **One maintainer vs companies/communities.** Ubuntu has Canonical, Fedora has Red Hat, Arch has thousands of packagers. StickLab OS has YOGEESHWARAN C. Security response, hardware coverage, and bug fixes will be slower. That's the price of a hand-built distro.
- **No Secure Boot.** Ubuntu/Fedora boot with Secure Boot on; StickLab OS (plain archiso) requires disabling it in firmware. A non-starter on locked-down lab/office machines.
- **Live ISO goes stale.** Ubuntu/Fedora update in place; Fedora Atomic even rolls back. StickLab OS has no updater or persistence layer — reboot the live stick and your work is gone unless you save it elsewhere. Rebuild the ISO to get fresh packages.
- **No signed checksums yet.** Ubuntu publishes signed hashes per release; StickLab OS doesn't (see Roadmap). Download-and-trust is weaker — verify the repo and build it yourself if that matters to you.
- **2GB budget means cuts.** No full IDE (VS Code), no office suite, no app store, JRE-only Java, and the full CUDA toolkit/JDK are a download away (`sticklab setup-gpu`) — not on the stick.
- **Minimal desktop, minimal hand-holding.** 3 lightweight Wayland WMs + Waybar is fast, but there's no graphical installer (only `archinstall`), no driver GUI, no settings app, no full DE like GNOME/KDE. GNOME/KDE users will feel the gap.
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

## 🖥️ Desktop & first boot — 3 window managers, your pick

- **tty1** → your chosen desktop auto-starts (Waybar top bar, dark lavender theme)
- **Default: Labwc** (stacking, Openbox-style) · **Sway** (manual tiling) · **Hyprland** (dynamic tiling + eye candy) — all Wayland, all sharing Waybar/Wofi/Foot, same **Bolt** keys everywhere
- Switch anytime: `rsetup wm` (lists) · `rsetup wm sway` (sets, takes effect on next tty1 login) · one-shot: `STICKLAB_WM=hyprland`
- **Bolt+Return** terminal · **Bolt+D** launcher · **Bolt+Q** close · **Bolt+1..4** workspaces · **Bolt+S** screenshot · **Bolt+L** lock
- **Bolt = the Alt key.** Super is deliberately *not* used, so a host OS (like Omarchy) keeps it when you run StickLab OS in a VM — no key fights.
- Other ttys stay plain console — perfect for learning
- Make it yours: `~/.config/labwc/rc.xml`, `~/.config/sway/config`, `~/.config/hypr/hyprland.conf`, `~/.config/waybar/`, `~/.config/wofi/`, `~/.config/foot/foot.ini`, `~/.config/nvim/init.lua`, `~/.zshrc`

---

## 🎨 Make it yours (customization-first)

Every default config is commented, example-loaded, and safe to break — restore by deleting it (skel defaults return on a fresh boot). On the live system, run `rsetup customize` for this map anytime:

| Want to change | File | How |
|---|---|---|
| Window manager | `rsetup wm <labwc\|sway\|hyprland>` | Saved to `~/.config/sticklab-wm`, applies on next tty1 login |
| Keybindings (Bolt = Alt) | `~/.config/labwc/rc.xml` | Copy a line, change `A-x`, run `labwc --reconfigure`. Screenshot (`Bolt+S`) + lock (`Bolt+L`) included |
| Keybindings (sway) | `~/.config/sway/config` | Same Bolt keys, sway tiling extras; reload with `Bolt+Shift+R` |
| Keybindings (hyprland) | `~/.config/hypr/hyprland.conf` | Same Bolt keys, accent `b4befe`; reload with `Bolt+Shift+R` (`hyprctl reload`) |
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
| Verification | `cargo test` (17 unit tests), ISO content audit, `systemd-nspawn` boot test | See [TESTING](TESTING.md) |

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

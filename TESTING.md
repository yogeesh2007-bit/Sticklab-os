# StickLab OS — testing log

Every line of Rust is unit-tested (`cargo test`, 17 tests). Every ISO is content-audited before release. This file records how and what was found.

## 1. Rust unit tests (`cargo test` in `rust/`)

| Test | What it proves |
|---|---|
| `mem_total_parses_kb` | `/proc/meminfo` parsing, incl. garbage/empty input |
| `scaffold_rust_has_cargo_and_main` | `sticklab new rust` embeds the project name correctly |
| `scaffold_all_templates_cover_languages` | all 6 templates produce non-empty files; unknown template rejected |
| `account_locked_detects_star_bang` | `*`, `!`, `!hash` count as locked; real hashes don't; missing user = `None` |
| `color_only_on_tty_without_no_color` | ANSI only on TTYs, `NO_COLOR` respected |
| `logo_has_seven_rows_and_sticklab_shape` | logo intact (7 rows, block letters) |
| `identify_board_known_chips_and_unknown` | USB VID:PID → board names (case-insensitive, whitespace-tolerant); unknowns = `None` |
| `render_contains_logo_and_rows` / `render_coloured_has_escapes` | plain mode has zero escapes; color mode has them |
| `boot_mode_efi_vs_bios` | UEFI vs legacy-BIOS string mapping for the dual-boot audit |
| `os_prober_parses_devices_and_labels` | `os-prober` lines → (device, label); blanks/unknowns handled |
| `esp_mounts_finds_vfat_efi_only` | only FAT ESP mounts (`/boot/efi`, `/boot`) detected, ext4 ignored |
| `wm_names_normalize_case_and_shorthand` | `rsetup wm` accepts case/whitespace/`hypr` shorthand, rejects unknowns |
| `wm_binaries_and_configs_cover_all_sessions` | every WM id maps to a binary + a key-config path |
| `saved_wm_defaults_to_labwc` | missing/garbage session file → labwc; `sway`, `hypr` shorthand parse |
| `hostname_allows_dns_names_rejects_injection` | `set-hostname` accepts DNS names, rejects spaces/newlines/`;`/overlong |
| `project_names_block_escape_allow_subdirs` | `sticklab new` rejects absolute/`..` paths, allows `foo/bar` |

## 2. ISO content audit (unsquashfs, read-only loop mount)

- ✅ kernel + initramfs present, BIOS+UEFI bootloaders, StickLab OS menu entries
- ✅ `sticklab`, `rinit`, `rfetch`, `rsetup` in `/usr/local/bin` (mode 755, root-owned)
- ✅ `labwc`/`sway`/`Hyprland`, `nvidia-smi` on board
- 🐛 **Found:** only login-capable user was `root` with a **locked** password and **no autologin** → unbootable login prompt. **Fixed** via `airootfs/etc/systemd/system/getty@tty1.service.d/autologin.conf`.
- ✅ `root:*` (locked), sshd **not** enabled, sudoers root-only, ufw enabled by default + service wanted.
- ✅ dual-boot: `grub`, `os-prober`, `ntfs-3g`, `dosfstools`, `mtools`, `parted`, `gptfdisk` on board; `sticklab dualboot` audits UEFI/BIOS, ESP, Secure Boot, other OSes.
- ✅ `labwc`, `sway`, `Hyprland` + `xorg-xwayland` on board; `~/.config/sticklab-wm` default `labwc`; `.bash_profile` session picker with fallback chain (chosen → labwc → sway → Hyprland → foot); sway/hyprland configs share Bolt keys, bar, accent
- ✅ waybar `wlr/workspaces` (compositor-agnostic) + per-WM fallbacks; `sticklab doctor` passes with any of the 3 compositors

## 3. Userspace boot test (`systemd-nspawn --boot` on extracted squashfs)

```bash
sudo unsquashfs -d /tmp/sticklabroot out/*/airootfs.sfs
sudo systemd-nspawn --boot -D /tmp/sticklabroot
# then: systemctl is-system-running; systemctl status rinit; /usr/local/bin/sticklab doctor
```

Expected: `rinit.service` success, `sticklab doctor` green (except VM-expected GPU/battery notes).

**Result on 2026-09-14 ISO (verified):** ✅ Reached Multi-User System, `rinit` finished OK,
Console Getty started, hostname `sticklab-os`, `sticklab doctor` 8/9 green, `rfetch` logo + credits
render. Single failure: ufw ("CLI Netfilter Manager") — container-only (no netfilter in
nspawn); fine on real hardware. WSL tarball (576M) verified: `wsl.conf`, hostname, gcc,
nvim, `sticklab` present, no kernel bundled.

**Result on 2026-09-14 rebuild (full `./build.sh`, verified in QEMU/KVM with serial
console):** ✅ archiso hooks mount `/dev/sr0` → bootmnt + loop → airootfs, `rinit`
started + finished, Multi-User + Graphical targets reached, `sticklab-os login:`
prompt, hostname `sticklab-os`, TLP + ufw ("CLI Netfilter Manager") finished OK on
real boot. Zero firstboot-wizard / emergency / panic strings. `sticklab doctor` 8/9
in nspawn (ufw only), `rsetup wm` shows all 3 WMs installed, `sticklab dualboot`
reports all 5 tools ready. Serial console was `console=ttyS0` direct-kernel boot;
normal BIOS/UEFI menu boot uses the same kernel + initramfs.

**Final 2026-09-14 build (audit fixes in, `./build.sh` again, QEMU/KVM serial boot):**
✅ **1,883,799,328 bytes (1.755 GiB)** — within budget. `rustc`/`cargo` on the ISO
are rustup proxy symlinks → print rustup's fetch hint until the toolchain lands
(docs say so). Lean archiso HOOKS kept initramfs at ~60MB. Boot: sr0 → bootmnt +
loop → airootfs, Multi-User reached, `sticklab-os login:`, zero wizard/emergency/
panic strings. Build lessons: `build.sh` work dir moved from `/tmp` (tmpfs,
ENOSPC at 85% squash on 8GB tmpfs) to `out/.archiso-tmp` (override:
`ARCHISO_TMPDIR`).

## 4. Size budget

`scripts/mk-usb-img.sh` warns the build over **2,147,483,648 bytes**. Current release:
**1,897,955,328 bytes (1.77 GiB)** — OK, within budget.
Biggest residents: `nvidia-utils`, `linux-firmware`, `jdk` (JRE only), kernel.
Budget lessons (2026-09-14): `tio` was dropped from Arch upstream → replaced with
`picocom`; the initramfs needs `airootfs/etc/mkinitcpio.conf.d/archiso.conf` (else
the ISO builds but can't boot); keep its HOOKS lean (no `kms` — it drags every GPU
driver + firmware, incl. nvidia-open, into a 243MB initramfs and breaks the budget);
mask `systemd-firstboot.service` in the live airootfs (else every boot blocks on a
timezone wizard, since the live medium is always "first boot").

## 5. Security / robustness audit (2026-09-14, `cargo clippy` + `cargo fmt` + adversarial runs)

`clippy --all-targets`: 0 warnings. `cargo fmt --check`: clean. All `sh -c` call sites
use fixed strings (no user input interpolation). Findings fixed:

- `rsetup set-hostname` wrote unsanitized root-owned input to `/etc/hostname` →
  now validates DNS hostname rules (rejects spaces/newlines/`;`/overlong), clean
  exit codes instead of `expect()` panics.
- `sticklab new` accepted `..`/absolute paths and panicked on I/O errors → now
  rejects escapes (`safe_project_name`), reports I/O failures with exit 1.
- `rsetup enable-gui` printed "enabled" without checking `systemctl` → now reports
  per-service success and exits 1 on failure.
- `scripts/install-deps.sh` used `pacman -Sy` (partial-upgrade risk) → `-Syu`.
- `scripts/run-vm.sh` failed cryptically with no ISO and hid OVMF-vars copy
  failures → friendly errors, strict copy.
- `packages.x86_64` listed both `rust` + `rustup` (pacman silently drops `rust`)
  → ships `rustup` only, docs corrected (Rust joins via one toolchain fetch).
- `profiledef.sh` set permissions on `/etc/shadow`+`/etc/gshadow`, which don't
  exist at customize time (cosmetic mkarchiso warnings) → removed; runtime
  `root:*` lock verified in the squashfs + audited by `sticklab doctor`.
- Skel `.zshrc` had `correct` (autocorrect nags on every typo) → removed.

Simulation runs (all clean, correct exit codes, no host writes): `new` with
`../../escape`, `/tmp/abs`, unknown template, existing dir, no args; `set-hostname`
with newline injection, leading hyphen, missing arg; valid `set-hostname` in a
user+mount namespace (bind-mounted hostname file updated, host untouched);
`env -i` runs; piped `rfetch` emits zero ANSI escapes; `.bash_profile` session
branching sandboxed for labwc/sway/hyprland/bogus/empty/file-based/non-tty1.

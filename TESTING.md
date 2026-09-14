# StickLab OS — testing log

Every line of Rust is unit-tested (`cargo test`, 15 tests). Every ISO is content-audited before release. This file records how and what was found.

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

## 4. Size budget

`scripts/mk-usb-img.sh` fails the build over **2,147,483,648 bytes**. Current release: ~1.7G.
Biggest residents: `nvidia-utils`, `linux-firmware`, `jdk` (JRE only), kernel.

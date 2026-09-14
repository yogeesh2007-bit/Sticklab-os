# StickLab OS — testing log

Every line of Rust is unit-tested (`cargo test`, 12 tests). Every ISO is content-audited before release. This file records how and what was found.

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

## 2. ISO content audit (unsquashfs, read-only loop mount)

- ✅ kernel + initramfs present, BIOS+UEFI bootloaders, StickLab OS menu entries
- ✅ `sticklab`, `rinit`, `rfetch`, `rsetup` in `/usr/local/bin` (mode 755, root-owned)
- ✅ `labwc`, `nvidia-smi` on board
- 🐛 **Found:** only login-capable user was `root` with a **locked** password and **no autologin** → unbootable login prompt. **Fixed** via `airootfs/etc/systemd/system/getty@tty1.service.d/autologin.conf`.
- ✅ `root:*` (locked), sshd **not** enabled, sudoers root-only, ufw enabled by default + service wanted.

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

#!/usr/bin/env bash
# archiso profile for StickLab OS — Arch remaster, ~1GB, minimal GUI, USB-bootable.
# Rust core userspace (rinit, rfetch, rsetup). See: https://wiki.archlinux.org/title/Archiso
iso_name="sticklab-os"
iso_label="STICKLABOS"
iso_publisher="StickLab OS by YOGEESHWARAN C <https://github.com/yogeesh2007-bit/sticklab-os>"
iso_application="StickLab OS live/USB"
iso_version="$(date +%Y.%m.%d)"
install_dir="arch"
bootmodes=('bios.syslinux' 'uefi.systemd-boot')
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'xz' '-Xbcj' 'x86' '-b' '1M' '-Xdict-size' '1M')
file_permissions=(
  # NOTE: /etc/shadow + /etc/gshadow are NOT listed: at customize time they don't
  # exist yet (created later by package install, root locked by Arch default —
  # verified `root:*` in the squashfs and audited by `sticklab doctor` at runtime).
  ["/usr/local/bin/rinit"]="0:0:755"
  ["/usr/local/bin/rfetch"]="0:0:755"
  ["/usr/local/bin/rsetup"]="0:0:755"
  ["/usr/local/bin/sticklab"]="0:0:755"
)

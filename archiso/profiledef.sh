#!/usr/bin/env bash
# archiso profile for Night OS — Arch remaster, ~1GB, minimal GUI, USB-bootable.
# Rust core userspace (rinit, rfetch, rsetup). See: https://wiki.archlinux.org/title/Archiso
iso_name="night-os"
iso_label="NIGHTOS"
iso_publisher="Night OS by YOGEESHWARAN C <https://github.com/yogeesh2007-bit/night-os>"
iso_application="Night OS live/USB"
iso_version="$(date +%Y.%m.%d)"
install_dir="arch"
bootmodes=('bios.syslinux' 'uefi.systemd-boot')
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'xz' '-Xbcj' 'x86' '-b' '1M' '-Xdict-size' '1M')
file_permissions=(
  ["/etc/shadow"]="0:0:400"
  ["/etc/gshadow"]="0:0:400"
  ["/usr/local/bin/rinit"]="0:0:755"
  ["/usr/local/bin/rfetch"]="0:0:755"
  ["/usr/local/bin/rsetup"]="0:0:755"
  ["/usr/local/bin/night"]="0:0:755"
)

#!/usr/bin/env bash
# Convert archiso ISO -> bootable USB raw .img.
# Arch ISOs are already hybrid (dd-able), so the .img is a verified copy
# renamed for USB flashers (balenaEtcher, dd, Ventoy data partition).
set -euo pipefail
ISO="${1:?usage: mk-usb-img.sh <input.iso> [output.img]}"
OUT="${2:-out/sticklab-os-usb.img}"
mkdir -p "$(dirname "$OUT")"
cp --reflink=auto "$ISO" "$OUT"
ls -lh "$ISO" "$OUT"
echo "--- size check (StickLab OS budget 2GB = 2147483648 bytes) ---"
stat -c '%n %s bytes' "$OUT"
SIZE=$(stat -c %s "$OUT")
if [ "$SIZE" -gt 2147483648 ]; then
  echo "WARN: over 2GiB. Trim packages.x86_64 (nvidia-utils/jdk are the big ones)."
else
  echo "OK: within 2GiB budget."
fi
echo "--- flash with: sudo dd if=$OUT of=/dev/sdX bs=4M status=progress oflag=sync ---"
echo "--- (replace /dev/sdX with your USB stick; TRIPLE-CHECK with lsblk first) ---"

#!/usr/bin/env bash
# Install build deps on Arch/Omarchy host. Needs sudo.
set -euo pipefail
sudo pacman -Syu --needed --noconfirm archiso arch-install-scripts squashfs-tools dosfstools mtools qemu-img git base-devel
echo "deps ok. (mkarchiso, pacstrap, qemu-img ready)"

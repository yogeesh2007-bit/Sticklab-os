#!/usr/bin/env bash
# Boot StickLab OS in a VM: ./scripts/run-vm.sh [bios|uefi] [iso]
# Works with QEMU (KVM), VirtualBox, VMware — the ISO is hybrid BIOS+UEFI.
set -euo pipefail
MODE="${1:-bios}"
ISO="${2:-$(ls -t out/*.iso 2>/dev/null | head -n1)}"
if [ -z "${ISO:-}" ] || [ ! -f "$ISO" ]; then
  echo "no ISO found in out/. Build one first: ./build.sh"
  exit 1
fi
if ! command -v qemu-system-x86_64 >/dev/null 2>&1; then
  echo "install QEMU first: sudo pacman -S qemu-desktop  (or open \$ISO in VirtualBox/VMware)"
  exit 1
fi
KVM=()
[ -e /dev/kvm ] && KVM=(-enable-kvm -cpu host)
if [ "$MODE" = uefi ]; then
  CODE=/usr/share/edk2-ovmf/x64/OVMF_CODE.4m.fd
  [ -f "$CODE" ] || { echo "need edk2-ovmf: sudo pacman -S edk2-ovmf"; exit 1; }
  if ! cp /usr/share/edk2-ovmf/x64/OVMF_VARS.4m.fd /tmp/sticklab-os-ovmf-vars.fd; then
    echo "cannot write /tmp/sticklab-os-ovmf-vars.fd (disk full?) — aborting instead of booting without NVRAM"
    exit 1
  fi
  exec qemu-system-x86_64 "${KVM[@]}" -m 3G -smp 2 -boot d -cdrom "$ISO" \
    -drive if=pflash,format=raw,readonly=on,file="$CODE" \
    -drive if=pflash,format=raw,file=/tmp/sticklab-os-ovmf-vars.fd
else
  exec qemu-system-x86_64 "${KVM[@]}" -m 3G -smp 2 -boot d -cdrom "$ISO"
fi

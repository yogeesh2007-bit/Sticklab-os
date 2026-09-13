#!/usr/bin/env bash
# Boot Night OS in a VM: ./scripts/run-vm.sh [bios|uefi] [iso]
# Works with QEMU (KVM), VirtualBox, VMware — the ISO is hybrid BIOS+UEFI.
set -euo pipefail
MODE="${1:-bios}"
ISO="${2:-$(ls -t out/*.iso | head -n1)}"
if ! command -v qemu-system-x86_64 >/dev/null 2>&1; then
  echo "install QEMU first: sudo pacman -S qemu-desktop  (or open \$ISO in VirtualBox/VMware)"
  exit 1
fi
KVM=()
[ -e /dev/kvm ] && KVM=(-enable-kvm -cpu host)
if [ "$MODE" = uefi ]; then
  CODE=/usr/share/edk2-ovmf/x64/OVMF_CODE.4m.fd
  [ -f "$CODE" ] || { echo "need edk2-ovmf: sudo pacman -S edk2-ovmf"; exit 1; }
  cp /usr/share/edk2-ovmf/x64/OVMF_VARS.4m.fd /tmp/night-os-ovmf-vars.fd 2>/dev/null || true
  exec qemu-system-x86_64 "${KVM[@]}" -m 3G -smp 2 -boot d -cdrom "$ISO" \
    -drive if=pflash,format=raw,readonly=on,file="$CODE" \
    -drive if=pflash,format=raw,file=/tmp/night-os-ovmf-vars.fd
else
  exec qemu-system-x86_64 "${KVM[@]}" -m 3G -smp 2 -boot d -cdrom "$ISO"
fi

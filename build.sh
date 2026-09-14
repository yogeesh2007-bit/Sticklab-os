#!/usr/bin/env bash
# Full build: Rust core -> stage into airootfs -> mkarchiso -> USB .img
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"

echo "[1/4] building StickLab OS Rust core (rinit, rfetch, rsetup, sticklab)..."
cargo build --release --manifest-path "$ROOT/rust/Cargo.toml"
mkdir -p "$ROOT/archiso/airootfs/usr/local/bin"
cp "$ROOT/rust/target/release/rinit" "$ROOT/rust/target/release/rfetch" "$ROOT/rust/target/release/rsetup" "$ROOT/rust/target/release/sticklab" "$ROOT/archiso/airootfs/usr/local/bin/"
chmod 755 "$ROOT/archiso/airootfs/usr/local/bin"/rinit "$ROOT/archiso/airootfs/usr/local/bin"/rfetch "$ROOT/archiso/airootfs/usr/local/bin"/rsetup "$ROOT/archiso/airootfs/usr/local/bin"/sticklab
ls -lh "$ROOT/archiso/airootfs/usr/local/bin/"

echo "[2/4] checking mkarchiso..."
if ! command -v mkarchiso >/dev/null 2>&1; then
  echo "mkarchiso missing. Run: ./scripts/install-deps.sh"
  exit 1
fi

echo "[3/4] building ISO (needs sudo, downloads ~700MB, takes 5-20 min)..."
sudo rm -rf /tmp/archiso-tmp out
mkdir -p out
sudo mkarchiso -v -w /tmp/archiso-tmp -o out archiso/
ISO=$(ls -t out/*.iso | head -n1)
sudo chown "$(id -u):$(id -g)" "$ISO"
echo "built: $ISO"

echo "[4/4] converting to USB .img..."
./scripts/mk-usb-img.sh "$ISO" out/sticklab-os-usb.img
echo "DONE: out/sticklab-os-usb.img"

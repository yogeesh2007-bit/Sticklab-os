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

echo "[3/4] building ISO (needs sudo, downloads ~700MB, takes ~30-45 min on 4 cores)..."
# NOTE: work dir lives under out/ (real disk), NOT /tmp — /tmp is often tmpfs and
# too small (airootfs work + squash image need ~7GB; seen ENOSPC on 8GB tmpfs).
# Override with ARCHISO_TMPDIR if you prefer elsewhere.
WORKDIR="${ARCHISO_TMPDIR:-$ROOT/out/.archiso-tmp}"
sudo rm -rf "$WORKDIR" out
mkdir -p out
sudo mkarchiso -v -w "$WORKDIR" -o out archiso/
ISO=$(ls -t out/*.iso | head -n1)
sudo chown "$(id -u):$(id -g)" "$ISO"
echo "built: $ISO"

echo "[4/4] converting to USB .img..."
./scripts/mk-usb-img.sh "$ISO" out/sticklab-os-usb.img
echo "DONE: out/sticklab-os-usb.img"

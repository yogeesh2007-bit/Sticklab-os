#!/usr/bin/env bash
# Build a WSL2 rootfs tarball for Night OS (terminal-only coder environment).
# Same Arch base + languages + `night` control center, minus kernel/GUI/firmware
# (WSL2 supplies its own kernel). Needs sudo + internet. Output: out/night-os-wsl.tar.gz
# Import:  wsl --import NightOS C:\WSL\NightOS out/night-os-wsl.tar.gz
# Docker:  docker import out/night-os-wsl.tar.gz night-os:wsl
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WORK=/tmp/nightwsl
OUT="$ROOT/out/night-os-wsl.tar.gz"
cargo build --release --manifest-path "$ROOT/rust/Cargo.toml"
sudo rm -rf "$WORK"
mkdir -p "$WORK"
sudo pacstrap -c "$WORK" base bash coreutils shadow sudo pacman glibc \
  git python python-pip go rustup nodejs sqlite ruby php lua perl \
  neovim tmux zsh eza bat fzf ripgrep fd tree jq curl wget gnupg just \
  cmake ninja make gcc pkgconf patch man-db man-pages tealdeer \
  htop btop ncdu strace lsof file unzip zip iproute2 iputils openssh \
  networkmanager usbutils pciutils
sudo cp "$ROOT/rust/target/release/rinit" "$ROOT/rust/target/release/rfetch" \
  "$ROOT/rust/target/release/rsetup" "$ROOT/rust/target/release/night" "$WORK/usr/local/bin/"
sudo tee "$WORK/etc/wsl.conf" >/dev/null <<'EOF'
[boot]
systemd=true
[user]
default=root
EOF
sudo sh -c "echo night-os > '$WORK/etc/hostname' && rm -f '$WORK/etc/machine-id' && tar -czpf '$OUT' -C '$WORK' ."
sudo chown "$(id -u):$(id -g)" "$OUT"
ls -lh "$OUT"

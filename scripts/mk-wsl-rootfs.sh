#!/usr/bin/env bash
# Build a WSL2 rootfs tarball for StickLab OS (terminal-only coder environment).
# Same Arch base + languages + `sticklab` control center, minus kernel/GUI/firmware
# (WSL2 supplies its own kernel). Needs sudo + internet. Output: out/sticklab-os-wsl.tar.gz
# Import:  wsl --import StickLabOS C:\WSL\StickLabOS out/sticklab-os-wsl.tar.gz
# Docker:  docker import out/sticklab-os-wsl.tar.gz sticklab-os:wsl
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
# If the whole script runs under sudo, root's PATH lacks cargo — borrow the invoker's.
if [ "$(id -u)" = 0 ] && [ -n "${SUDO_USER:-}" ]; then
  SU_HOME=$(getent passwd "$SUDO_USER" | cut -d: -f6)
  export PATH="$SU_HOME/.cargo/bin:$PATH"
fi
WORK="$ROOT/out/.wsl-work"   # on-disk (out/ is git-ignored); /tmp tmpfs is too small
OUT="$ROOT/out/sticklab-os-wsl.tar.gz"
# Build Rust bins as the invoking user (root has no rustup toolchain); reuse if fresh.
if [ ! -x "$ROOT/rust/target/release/sticklab" ]; then
  if [ "$(id -u)" = 0 ] && [ -n "${SUDO_USER:-}" ]; then
    sudo -u "$SUDO_USER" cargo build --release --manifest-path "$ROOT/rust/Cargo.toml"
  else
    cargo build --release --manifest-path "$ROOT/rust/Cargo.toml"
  fi
fi
sudo rm -rf "$WORK"
mkdir -p "$WORK"
sudo pacstrap -c "$WORK" base bash coreutils shadow sudo pacman glibc \
  git python python-pip go rustup nodejs sqlite ruby php lua perl \
  neovim tmux zsh eza bat fzf ripgrep fd tree jq curl wget gnupg just \
  cmake ninja make gcc pkgconf patch man-db man-pages tealdeer \
  htop btop ncdu strace lsof file unzip zip iproute2 iputils openssh \
  networkmanager usbutils pciutils
sudo cp "$ROOT/rust/target/release/rinit" "$ROOT/rust/target/release/rfetch" \
  "$ROOT/rust/target/release/rsetup" "$ROOT/rust/target/release/sticklab" "$WORK/usr/local/bin/"
sudo tee "$WORK/etc/wsl.conf" >/dev/null <<'EOF'
[boot]
systemd=true
[user]
default=root
EOF
sudo sh -c "echo sticklab-os > '$WORK/etc/hostname' && rm -f '$WORK/etc/machine-id' && tar -czpf '$OUT' -C '$WORK' ."
sudo chown "$(id -u):$(id -g)" "$OUT"
ls -lh "$OUT"

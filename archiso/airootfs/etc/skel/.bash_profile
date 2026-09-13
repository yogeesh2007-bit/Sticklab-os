# ~/.bash_profile for Night OS live user — start Labwc desktop on tty1.
# Plain console on every other tty (learn-Linux friendly).
if [ -z "$WAYLAND_DISPLAY" ] && [ "$(tty)" = "/dev/tty1" ]; then
  if command -v labwc >/dev/null 2>&1; then
    exec labwc
  elif command -v foot >/dev/null 2>&1; then
    exec foot
  fi
fi
. ~/.bashrc 2>/dev/null || true

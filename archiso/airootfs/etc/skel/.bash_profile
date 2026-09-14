# ~/.bash_profile for StickLab OS live user — start the chosen Wayland desktop on tty1.
# Pick your window manager: `rsetup wm labwc|sway|hyprland` (saves ~/.config/sticklab-wm),
# or one-shot: STICKLAB_WM=sway, then log in on tty1. Default: labwc (stacking).
# Plain console on every other tty (learn-Linux friendly).
if [ -z "$WAYLAND_DISPLAY" ] && [ "$(tty)" = "/dev/tty1" ]; then
  WM="${STICKLAB_WM:-$(cat ~/.config/sticklab-wm 2>/dev/null)}"
  case "${WM:-labwc}" in
    sway)
      command -v sway >/dev/null 2>&1 && exec sway
      ;;
    hyprland|hypr|Hyprland)
      command -v Hyprland >/dev/null 2>&1 && exec Hyprland
      ;;
    labwc|*)
      command -v labwc >/dev/null 2>&1 && exec labwc
      ;;
  esac
  # Fallback chain: chosen WM missing? try the others, then a bare terminal.
  command -v labwc >/dev/null 2>&1 && exec labwc
  command -v sway >/dev/null 2>&1 && exec sway
  command -v Hyprland >/dev/null 2>&1 && exec Hyprland
  command -v foot >/dev/null 2>&1 && exec foot
fi
. ~/.bashrc 2>/dev/null || true

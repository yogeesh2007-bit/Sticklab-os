# ~/.bashrc for StickLab OS — plain bash defaults. Edit freely.
# MAKE IT YOURS: aliases below, prompt via PS1, colors via ~/.config/foot/foot.ini.
# See `rsetup customize` for a map of every customizable file.

# --- prompt: user@host dir $ (tweak colors with \[\e[..m\] codes) ---
PS1='\[\e[1;95m\]\u@\h\[\e[0m\] \[\e[96m\]\W\[\e[0m\] \$ '

# --- history: big, shared, no duplicates ---
HISTSIZE=5000
HISTFILESIZE=10000
HISTCONTROL=ignoreboth
shopt -s histappend checkwinsize

# --- friendly aliases (delete what you don't like) ---
command -v eza >/dev/null && alias ls='eza --group-directories-first' || alias ls='ls --color=auto'
command -v bat >/dev/null && alias cat='bat --paging=never'
alias ll='ls -la' grep='grep --color=auto'
command -v nvim >/dev/null && export EDITOR=nvim VISUAL=nvim

# --- your additions go below this line ---

rfetch 2>/dev/null || true

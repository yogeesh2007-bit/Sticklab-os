# StickLab OS default zshrc — pick bash or zsh, both work. Edit freely.
# MAKE IT YOURS: prompt line below (try: prompt adam2 / prompt bart / prompt redhat),
# aliases at the bottom, colors via ~/.config/foot/foot.ini. See `rsetup customize`.
HISTFILE=~/.zsh_history
HISTSIZE=5000
SAVEHIST=5000
setopt appendhistory autocd
# NOTE: zsh `correct` (autocorrect prompts) is OFF on purpose — it interrupts
# newcomers on every typo. You get autosuggestions + syntax highlighting instead.
autoload -Uz compinit promptinit && compinit && promptinit
prompt adam1

[ -r /usr/share/zsh/plugins/zsh-autosuggestions/zsh-autosuggestions.zsh ] && \
  source /usr/share/zsh/plugins/zsh-autosuggestions/zsh-autosuggestions.zsh
[ -r /usr/share/zsh/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh ] && \
  source /usr/share/zsh/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh

command -v eza >/dev/null && alias ls='eza --group-directories-first' || alias ls='ls --color=auto'
command -v bat >/dev/null && alias cat='bat --paging=never'
alias ll='ls -la' grep='grep --color=auto'
command -v nvim >/dev/null && export EDITOR=nvim VISUAL=nvim

rfetch 2>/dev/null || true

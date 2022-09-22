
# tmux helpers
alias stx='tmux source-file ~/.tmux.conf'
alias ta='tmux attach -t'
alias ts='tmux new -s'
alias tw='tmux new-window -n'
alias twk='tmux kill-window -t '
alias tks='tmux kill-server'
alias kill-session='tmux kill-session -t'
alias pwru='/root/pwru/pwru'
alias rebash='source /root/.bashrc'

# network stuff
nn () {
    # enter a network namespace
    nsenter --net=/var/run/netns/$1
}

# v2 of nn
ns() {
  ip netns exec $1  /bin/bash --rcfile <(echo "PS1=\"$1> \"")
}
#!/bin/sh
in_vim=$(ps -o state= -o comm= -t $PANE_TTY | grep 'S[+]\s*[n]*vim')
in_vim_pretty=$([ ! -z "$in_vim" ] && echo "true" || echo "false")
# 8 - ^H - Ctrl h
# 10 - ^J - Ctrl j
# 11 - ^K - Ctrl k
# 12 - ^L - Ctrl l
zellij action write 12

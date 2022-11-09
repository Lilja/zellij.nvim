#!/bin/sh
if [ -z $PANE_TTY ]; then
    echo "\$PANE_TTY unset. Please set it in your shell and reload it."
    exit 1
fi
direction=$1

in_vim=$(ps -o state= -o comm= -t $PANE_TTY | grep 'S[+]\s*[n]*vim')
in_vim_pretty=$([ ! -z "$in_vim" ] && echo "true" || echo "false")
echo "in_vim: $in_vim_pretty"

if [ ! -z $in_vim ]; then
    echo "b"
    direction_num=-1
    if [ "$direction" = "h" ]; then
        direction_num=8
    elif [ "$direction" = "j" ]; then
        direction_num=10
    elif [ "$direction" = "k" ]; then
        direction_num=11
    elif [ "$direction" = "l" ]; then
        direction_num=12
    elif [ "$direction" = "\\" ]; then
        direction_num=28
    else
        echo "Unknown direction '$direction'. Allowed: h|j|k|l\\"
        exit 1
    fi
    echo "$direction_num"

    # 8 - ^H - Ctrl h
    # 10 - ^J - Ctrl j
    # 11 - ^K - Ctrl k
    # 12 - ^L - Ctrl l
    # zellij action write $direction_num
else
    echo "a"
    direction_z=-1
    if [ "$direction" = "h" ]; then
        direction_z="left"
    elif [ "$direction" = "j" ]; then
        direction_z="down"
    elif [ "$direction" = "k" ]; then
        direction_z="up"
    elif [ "$direction" = "l" ]; then
        direction_z="right"
    elif [ "$direction" = "\\" ]; then
        zellij action focus-next-pane
    else
        echo "Unknown direction '$direction'. Allowed: h|j|k|l\\"
        exit 1
    fi
    zellij action move-focus "$direction_z"
    exit $?
fi

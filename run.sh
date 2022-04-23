#!/bin/bash
VERBOSE=false
FOLLOW=false

pkill -x Xephyr -9
if [ $? -eq 0 ]; then
    exit 0
fi

while getopts "vf" options; do
    case "${options}" in
        v) VERBOSE=true;;
        f) FOLLOW=true;;
        *) exit 1 ;;
    esac
done

if [ $FOLLOW = true ]; then
    rm -f ./log; touch ./log
fi

if [ $VERBOSE = true ]; then
    echo "exec cargo run" | startx /dev/stdin -- /usr/bin/Xephyr :1 -resizeable
else
    nohup startx ./xinitrc -- /usr/bin/Xephyr :1 -resizeable > /dev/null &
fi

if [ $FOLLOW = true ]; then
    tail -f ./log
fi

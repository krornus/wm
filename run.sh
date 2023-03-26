#!/bin/bash
pkill -x Xephyr -15

if [ $? -eq 0 ]; then
    exit 0
fi

echo "RUST_BACKTRACE=1 exec cargo run --release" | startx /dev/stdin -- /usr/bin/Xephyr :1 -resizeable

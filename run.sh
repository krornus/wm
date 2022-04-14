#!/bin/bash
nohup startx ./xinitrc -- /usr/bin/Xephyr :1 -resizeable > /dev/null

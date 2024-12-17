#!/bin/sh

# Restart Waybar
pkill -x waybar || true
nohup waybar >/dev/null 2>&1 &

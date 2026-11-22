#!/bin/sh

# swaymsg output HEADLESS-1 resolution 1920x1080 scale 1

source $(dirname $0)/ips.sh

for BANANA_IP in "${BANANA_IPS[@]}"
do
    ssh pi@$BANANA_IP "XDG_RUNTIME_DIR=/run/user/$(id -u) WAYLAND_DISPLAY=wayland-1 SWAYSOCK=$(ls $XDG_RUNTIME_DIR/sway-ipc.*.sock | head -n1) swaymsg output HEADLESS-1 resolution 1920x1080 scale 1 & XDG_RUNTIME_DIR=/run/user/$(id -u) WAYLAND_DISPLAY=wayland-1 SWAYSOCK=$(ls $XDG_RUNTIME_DIR/sway-ipc.*.sock | head -n1) wayvnc 0.0.0.0 5900" &
done

sleep 5

for BANANA_IP in "${BANANA_IPS[@]}"
do
    krdc vnc://$BANANA_IP:5900 &
done

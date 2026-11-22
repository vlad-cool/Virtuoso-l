#!/bin/bash
set -e

source $(dirname $0)/ips.sh

cross build --target armv7-unknown-linux-gnueabihf --release --no-default-features --features=embeded_device,sdl_frontend,legacy_backend_full,gpio_frontend,repeater,cyrano_server

reboot_requested=false

for arg in "$@"; do
    case "$arg" in
        --reboot)
            reboot_requested=true
            ;;
    esac
done

for BANANA_IP in "${BANANA_IPS[@]}"
do
    rsync --info=progress2 target/armv7-unknown-linux-gnueabihf/release/Virtuoso pi@$BANANA_IP:Virtuoso/app
    if [ "$reboot_requested" = true ]; then
        ssh pi@$BANANA_IP sudo reboot
    fi
done

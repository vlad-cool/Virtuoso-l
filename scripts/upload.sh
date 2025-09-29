#!/bin/bash
set -e

BANANA_IPS=("192.168.1.41")

# cross build --target armv7-unknown-linux-gnueabihf --release --no-default-features --features=sdl_frontend,legacy_backend_full,gpio_frontend,repeater,cyrano_server
# cross build --target armv7-unknown-linux-gnueabihf --release --no-default-features --features=sdl_frontend,legacy_backend_full,gpio_frontend,repeater
cross build --target armv7-unknown-linux-gnueabihf --no-default-features --features=embeded_device,sdl_frontend,legacy_backend_full,gpio_frontend,repeater

for BANANA_IP in "${BANANA_IPS[@]}"
do
    # rsync --info=progress2 target/armv7-unknown-linux-gnueabihf/release/Virtuoso pi@$BANANA_IP:Virtuoso/app
    rsync --info=progress2 target/armv7-unknown-linux-gnueabihf/debug/Virtuoso pi@$BANANA_IP:Virtuoso/app
    # ssh pi@$BANANA_IP ./kill.sh
    ssh pi@$BANANA_IP sudo reboot
done
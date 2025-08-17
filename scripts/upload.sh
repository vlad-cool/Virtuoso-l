#!/bin/bash
set -e
# BANANA_IPS=("192.168.1.33" "192.168.1.35")
BANANA_IPS=("192.168.1.64")
# BANANA_IPS=("192.168.3.122" "192.168.3.110")
# BANANA_IPS=("192.168.3.35")

# cross build --target armv7-unknown-linux-gnueabihf --release --no-default-features --features=slint_frontend,legacy_backend,cyrano_server,gpio_frontend
# cross build --target armv7-unknown-linux-gnueabihf --release --no-default-features --features=sdl_frontend,legacy_backend,gpio_frontend,repeater,cyrano_server
cross build --target armv7-unknown-linux-gnueabihf --release --no-default-features --features=sdl_frontend,legacy_backend,gpio_frontend,repeater,cyrano_server
for BANANA_IP in "${BANANA_IPS[@]}"
do
    rsync --info=progress2 target/armv7-unknown-linux-gnueabihf/release/Virtuoso pi@$BANANA_IP:Virtuoso/app
    # ssh pi@$BANANA_IP ./kill.sh
    ssh pi@$BANANA_IP sudo reboot
done
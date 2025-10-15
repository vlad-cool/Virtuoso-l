#!/bin/bash
set -e

BANANA_IPS=("192.168.1.38")

cross build --target armv7-unknown-linux-gnueabihf --release --no-default-features --features=embeded_device,sdl_frontend,legacy_backend_full,gpio_frontend,repeater,cyrano_server

for BANANA_IP in "${BANANA_IPS[@]}"
do
    rsync --info=progress2 target/armv7-unknown-linux-gnueabihf/release/Virtuoso pi@$BANANA_IP:Virtuoso/app
done

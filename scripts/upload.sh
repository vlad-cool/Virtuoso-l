#!/bin/bash
set -e
BANANA_IPS=("192.168.1.33" "192.168.1.42")

# cross build --target armv7-unknown-linux-gnueabihf --release --no-default-features --features=slint_frontend,legacy_backend,cyrano_server,gpio_frontend
cross build --target armv7-unknown-linux-gnueabihf --release --no-default-features --features=slint_frontend,legacy_backend,gpio_frontend,repeater
for BANANA_IP in "${BANANA_IPS[@]}"
do
    rsync --info=progress2 target/armv7-unknown-linux-gnueabihf/release/Virtuoso pi@$BANANA_IP:Virtuoso/app
    ssh pi@$BANANA_IP ./kill.sh
done
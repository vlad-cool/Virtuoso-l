#!/bin/sh
BANANA_IP=192.168.4.34

cross build --target armv7-unknown-linux-gnueabihf --release --no-default-features --features=slint_frontend,legacy_backend,cyrano_server,gpio_frontend
rsync --info=progress2 target/armv7-unknown-linux-gnueabihf/release/Virtuoso pi@$BANANA_IP:Virtuoso/app
ssh pi@$BANANA_IP ./kill.sh

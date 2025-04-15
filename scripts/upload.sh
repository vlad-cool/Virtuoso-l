#!/bin/sh
BANANA_IP=192.168.1.35

cross build --target armv7-unknown-linux-gnueabihf --release
rsync target/armv7-unknown-linux-gnueabihf/release/Virtuoso-l src/run.sh pi@$BANANA_IP:Virtuoso/app
ssh pi@$BANANA_IP ./kill.sh


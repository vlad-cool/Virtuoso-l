#!/bin/bash
sudo cp -r ../linux_assets/* mnt/home/pi
chmod +x mnt/home/pi/setup.sh

sudo umount mnt

sudo mount /dev/loop0p2 mnt
sudo mkdir mnt/app
# cp ../src/scripts/run.sh mnt/app/run.sh
# chmod +x mnt/app/run.sh
# cp ../target

cross build --target armv7-unknown-linux-gnueabihf --release --no-default-features --features=sdl_frontend,legacy_backend,gpio_frontend,repeater
sudo cp ../target/armv7-unknown-linux-gnueabihf/release/Virtuoso mnt/app
sudo umount mnt

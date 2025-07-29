#!/bin/bash

echo DANGEROUS SCRIPT. MAKE SURE IMAGE WILL BE MOUNTED TO /dev/loop0
sleep 10
sudo dd if=/dev/zero bs=1M count=2049 >> $1
sudo parted --script $1 mkpart primary fat32 4096MiB 6144MiB
sudo losetup -Pf $1
sudo mkfs.vfat /dev/loop0p2

cd "$(dirname "$(readlink -f "$0")")"
mkdir -p mnt

sudo mount /dev/loop0p1 mnt
UUID=$(sudo blkid -s UUID -o value /dev/loop0p2)
sudo tee -a mnt/etc/fstab <<< "UUID=$UUID  /home/pi/Virtuoso  vfat  defaults,nofail,uid=1000,gid=1000,umask=022  0  2"
# mkdir -p mnt/home
sudo mkdir -p mnt/home/pi
sudo cp -r ../linux_assets/* mnt/home/pi
sudo umount mnt

sudo mount /dev/loop0p2 mnt
sudo mkdir mnt/app
# cp ../src/scripts/run.sh mnt/app/run.sh
# chmod +x mnt/app/run.sh
# cp ../target

cross build --target armv7-unknown-linux-gnueabihf --release --no-default-features --features=sdl_frontend,legacy_backend,gpio_frontend,repeater
sudo cp ../target/armv7-unknown-linux-gnueabihf/release/Virtuoso mnt/app
sudo umount mnt

sudo losetup -d /dev/loop0

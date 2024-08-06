#!/bin/sh

mkdir -p bin
umount mnt
mount -o loop,rw,sync,offset=4194304 image_build/build_image.img mnt
mkdir -p mnt/home/pi/gpio/
cp src/gpio/* mnt/home/pi/gpio
umount mnt

sudo image_exec -i image_build/build_image.img -u "cd home/pi/gpio && make"

mount -o loop,rw,sync,offset=4194304 image_build/build_image.img mnt
cp mnt/home/pi/gpio/get_time mnt/home/pi/gpio/get_pin mnt/home/pi/gpio/get_rc5 mnt/home/pi/gpio/send_pin mnt/home/pi/gpio/send_rc5 bin
umount mnt

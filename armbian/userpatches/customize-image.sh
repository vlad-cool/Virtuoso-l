#!/bin/bash

# arguments: $RELEASE $LINUXFAMILY $BOARD $BUILD_DESKTOP
#
# This is the image customization script

# NOTE: It is copied to /tmp directory inside the image
# and executed there inside chroot environment
# so don't reference any files that are not already installed

# NOTE: If you want to transfer files between chroot and host
# userpatches/overlay directory on host is bind-mounted to /tmp/overlay in chroot
# The sd card's root path is accessible via $SDCARD variable.

RELEASE=$1
LINUXFAMILY=$2
BOARD=$3
BUILD_DESKTOP=$4

chpasswd <<< "root:VirtuosoRoot"

adduser --quiet --disabled-password --gecos "" pi
chpasswd <<< "pi:Virtuoso"

rm /root/.not_logged_in_yet
touch /root/.no_rootfs_resize

mkdir -p /etc/systemd/system/getty@tty1.service.d

cat >  /etc/systemd/system/getty@tty1.service.d/override.conf <<< "[Service]"
cat >> /etc/systemd/system/getty@tty1.service.d/override.conf <<< "ExecStart="
cat >> /etc/systemd/system/getty@tty1.service.d/override.conf <<< "ExecStart=-/sbin/mingetty --autologin pi --noclear tty1"
systemctl enable getty@tty1.service

cat > /etc/sudoers <<< "pi ALL=(ALL) NOPASSWD: /home/pi/setup.sh"

dpkg --set-selections <<< "armbian-bsp-cli-bananapim2zero      hold"
dpkg --set-selections <<< "armbian-firmware                    hold"
dpkg --set-selections <<< "armbian-jammy-desktop-xfce          hold"
dpkg --set-selections <<< "linux-dtb-current-sunxi             hold"
dpkg --set-selections <<< "linux-image-current-sunxi           hold"
dpkg --set-selections <<< "linux-u-boot-bananapim2zero-current hold"

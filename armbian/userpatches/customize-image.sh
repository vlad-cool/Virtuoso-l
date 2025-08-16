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

apt update
apt install -y sway mingetty overlayroot libsdl2-2.0-0 libsdl2-gfx-1.0-0 libsdl2-image-2.0-0 libsdl2-mixer-2.0-0 libsdl2-net-2.0-0 libsdl2-ttf-2.0-0

mkdir -p /etc/systemd/system/getty@tty1.service.d

cat > /etc/systemd/system/getty@tty1.service.d/override.conf <<< '[Service]
ExecStart=
ExecStart=-/sbin/mingetty --autologin pi --noclear tty1'

systemctl enable getty@tty1.service

cat > /etc/udev/rules.d/97-gpio.rules <<< 'SUBSYSTEM=="gpio", KERNEL=="gpiochip[0-4]", GROUP="gpio", MODE="0660"'

for group in tty disk dialout sudo audio video plugdev games users systemd-journal input render netdev gpio; do groupadd $group 2>/dev/null; usermod -aG $group pi; done

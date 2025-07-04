#!/bin/sh
cd /home/$SUDO_USER/device_configurations/$1
cp run.sh /home/$SUDO_USER/Virtuoso/app/
cp armbianEnv.txt /boot/

chown pi /home/$SUDO_USER/Virtuoso/app/run.sh
chgrp pi /home/$SUDO_USER/Virtuoso/app/run.sh

mkdir -p /usr/share/plymouth/themes/Virtuoso/
rm -f /usr/share/plymouth/themes/Virtuoso/*
cp splash/* /usr/share/plymouth/themes/Virtuoso/ 

update-initramfs -u -v

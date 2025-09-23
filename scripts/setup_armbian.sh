#!/bin/bash
shopt -s dotglob

echo DANGEROUS SCRIPT. MAKE SURE IMAGE WILL BE MOUNTED TO /dev/loop0

sudo image_exec -i $1 -p 1 "chpasswd <<< "root:VirtuosoRoot"

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

for group in tty disk dialout sudo audio video plugdev games users systemd-journal input render netdev gpio; do groupadd $group; usermod -aG $group pi; done
"

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
chmod +x mnt/home/pi/setup.sh

sudo tee mnt/etc/sudorers <<< 'Defaults        env_reset
Defaults        mail_badpass
Defaults        secure_path="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/snap/bin"
Defaults        !requiretty
Defaults        use_pty
root    ALL=(ALL:ALL) ALL
%admin  ALL=(ALL) ALL
%sudo   ALL=(ALL:ALL) ALL
@includedir /etc/sudoers.d
pi ALL=(ALL) NOPASSWD: /usr/bin/plymouth
pi ALL=(ALL) NOPASSWD: /home/pi/setup.sh'

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

echo sudo image_exec -i $1 -p 1 -u bash <<< "chown -R pi:pi /home/pi"
sudo image_exec -i $1 -p 1 -u bash <<< "chown -R pi:pi /home/pi
chmod +x /home/pi/setup.sh"

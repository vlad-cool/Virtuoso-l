#!/bin/sh

echo "armbian-bsp-cli-bananapim2zero      hold" | sudo dpkg --set-selections
echo "armbian-firmware                    hold" | sudo dpkg --set-selections
echo "armbian-jammy-desktop-xfce          hold" | sudo dpkg --set-selections
echo "linux-dtb-current-sunxi             hold" | sudo dpkg --set-selections
echo "linux-image-current-sunxi           hold" | sudo dpkg --set-selections
echo "linux-u-boot-bananapim2zero-current hold" | sudo dpkg --set-selections

sudo systemctl set-default multi-user.target

sudo apt update
sudo apt upgrade

# Kivy requirements
    sudo apt -y install build-essential git make autoconf automake libtool \
        pkg-config cmake ninja-build libasound2-dev libpulse-dev libaudio-dev \
        libjack-dev libsndio-dev libsamplerate0-dev libx11-dev libxext-dev \
        libxrandr-dev libxcursor-dev libxfixes-dev libxi-dev libxss-dev libwayland-dev \
        libxkbcommon-dev libdrm-dev libgbm-dev libgl1-mesa-dev libgles2-mesa-dev \
        libegl1-mesa-dev libdbus-1-dev libibus-1.0-dev libudev-dev fcitx-libs-dev 
# FFpyplayer requirements
sudo apt install -y python3-dev python3-venv libportmidi-dev libswscale-dev libfreetype6-dev \
    libavformat-dev libavcodec-dev libjpeg-dev libtiff-dev libx11-6 libx11-dev \
    libavfilter-dev libavfilter-extra

cd

wget https://libsdl.org/release/SDL2-2.0.10.tar.gz
tar -zxvf SDL2-2.0.10.tar.gz
cd SDL2-2.0.10
./configure --enable-video-kmsdrm --disable-video-opengl --disable-video-x11 --disable-video-rpi
make -j$(nproc)
sudo make install
rm SDL2-2.0.10.tar.gz
rm -r SDL2-2.0.10.tar.gz
cd

wget https://libsdl.org/projects/SDL_image/release/SDL2_image-2.0.5.tar.gz
tar -zxvf SDL2_image-2.0.5.tar.gz
cd SDL2_image-2.0.5
./configure
make -j$(nproc)
sudo make install
rm SDL2_image-2.0.5.tar.gz
rm -r SDL2_image-2.0.5
cd

wget https://libsdl.org/projects/SDL_mixer/release/SDL2_mixer-2.0.4.tar.gz
tar -zxvf SDL2_mixer-2.0.4.tar.gz
cd SDL2_mixer-2.0.4
./configure
make -j$(nproc)
sudo make install
rm SDL2_mixer-2.0.4.tar.gz
rm -r SDL2_mixer-2.0.4
cd

wget https://libsdl.org/projects/SDL_ttf/release/SDL2_ttf-2.0.15.tar.gz
tar -zxvf SDL2_ttf-2.0.15.tar.gz
cd SDL2_ttf-2.0.15
./configure
make -j$(nproc)
sudo make install
rm SDL2_ttf-2.0.15.tar.gz
rm -r SDL2_ttf-2.0.15
cd

sudo apt -y install gpiod xorg

sudo groupadd gpio
sudo usermod -a -G gpio pi
echo "SUBSYSTEM==\"gpio\", KERNEL==\"gpiochip[0-4]\", GROUP=\"gpio\", MODE=\"0660\"" | sudo tee -a  /etc/udev/rules.d/97-gpio.rules
sudo udevadm control --reload-rules
sudo udevadm trigger

sudo apt -y install xinput xorg
sudo apt -y install mencoder



sudo cp sun8i-h2-plus-bananapi-m2-zero.dtb /boot/dtb/

echo /dev/mmcblk0p2 /mnt/V24m vfat defaults,rw,users,uid=1000,gid=1000  0    0 | sudo tee -a /etc/fstab


sudo apt -y install python3-setuptools python3-pip python3-dev python3-venv
sudo apt -y install mencoder
sudo apt -y install xinput xorg
sudo apt -y install gpiod


sudo cp cedar_ve.ko /lib/modules/5.15.80-sunxi/
sudo depmod
echo cedar_ve | sudo tee -a /etc/modules
echo SUBSYSTEM==\"cedar_dev\", KERNEL==\"cedar_dev\", GROUP=\"video\", MODE=\"0660\" | sudo tee /etc/udev/rules.d/60-cedar.rules

tar xvf FFmpeg.tar.gz
cd FFmpeg
bash install_ffmpeg.sh
cd ..

sudo mkdir -p /var/lib/bananapi
sudo touch /var/lib/bananapi/board.sh
echo "BOARD=bpi-m2z"      | sudo tee    /var/lib/bananapi/board.sh
echo "BOARD_AUTO=bpi-m2z" | sudo tee -a /var/lib/bananapi/board.sh
echo "BOARD_OLD=bpi-m64"  | sudo tee -a /var/lib/bananapi/board.sh

tar xvf BPI-WiringPi2.tar.gz
cd BPI-WiringPi2
sudo ./build
cd ..

tar xvf SDL2.tar.gz

cd SDL2-2.0.10
sudo make install
cd ..

cd SDL2_image-2.0.5
sudo make install
cd ..

cd SDL2_mixer-2.0.4
sudo make install
cd ..

cd SDL2_ttf-2.0.15
sudo make install
cd ..


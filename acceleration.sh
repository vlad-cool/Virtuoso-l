#run this script with 'sudo -E bash acceleration.sh'

apt -y update
apt -y upgrade
apt -y install build-essential
apt -y install ninja-build
apt -y install python3-pip
apt -y install python3-dev
apt -y install libglfw3-dev libgl1-mesa-dev libglu1-mesa-dev
apt -y install libsdl2-dev
apt -y install libxml2-dev
apt -y install ffmpeg
apt -y install libavcodec-dev
apt -y install libavfilter-dev
apt -y install libavdevice-dev
apt -y install clang lldb lld
apt -y install libxcb-shm0-dev

pip3 install meson
pip3 install setuptools
pip3 install Mako
. ~/.profile

#sudo apt install gcc-mingw-w64 #May work without this line

wget https://dri.freedesktop.org/libdrm/libdrm-2.4.112.tar.xz
tar xf libdrm-2.4.112.tar.xz
rm libdrm-2.4.112.tar.xz
cd libdrm-2.4.112
##
meson build/
ninja -C build/ install
##
cd

git clone https://gitlab.freedesktop.org/wayland/wayland
cd waylad
meson build/ -Ddocumentation=false
sudo -E ninja -C build/ install
cd

https://github.com/wayland-project/wayland-protocols.git
cd wayland-protocols
meson build/
cd

#git clone https://github.com/wayland-project/wayland-protocols.git
#cd wayland-protocols

wget https://github.com/Kitware/CMake/releases/download/v3.23.2/cmake-3.23.2.tar.gz
tar xf cmake-3.23.2.tar.gz
rm cmake-3.23.2.tar.gz
cd cmake-3.23.2
./bootstrap
make
make install
cd
wget https://archive.mesa3d.org//mesa-22.1.3.tar.xz
tar xf mesa-22.1.3.tar.xz
rm mesa-22.1.3.tar.xz
cd mesa-22.1.3

meson build/
ninja -C build/
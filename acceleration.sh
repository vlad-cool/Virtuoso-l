#run this script with 'bash acceleration.sh'
cd

sudo apt -y update
sudo apt -y upgrade

sudo apt -y install g++ zlib1g-dev libexpat1-dev flex bison libx11-dev libxext-dev libxdamage-dev \
libxcb-glx0-dev libx11-xcb-dev libxcb-dri2-0-dev libxcb-dri3-dev libxcb-present-dev libxshmfence-dev \
libxxf86vm-dev libxrandr-dev x11proto-gl-dev x11proto-dri2-dev gettext pkg-config build-essential \
ninja-build python3-pip python3-dev libglfw3-dev libgl1-mesa-dev libglu1-mesa-dev libsdl2-dev \
libxml2-dev ffmpeg libavcodec-dev libavfilter-dev libavdevice-dev clang lldb lld libxcb-shm0-dev \
xinit xorg mesa-utils llvm-dev

pip3 install setuptools
pip3 install meson
pip3 install Mako
. ~/.profile

wget https://github.com/Kitware/CMake/releases/download/v3.23.2/cmake-3.23.2.tar.gz
tar xf cmake-3.23.2.tar.gz
rm cmake-3.23.2.tar.gz
cd cmake-3.23.2
#cmake .
#make
#sudo make install
./bootstrap
make
sudo make install
cd

wget https://dri.freedesktop.org/libdrm/libdrm-2.4.112.tar.xz
tar xf libdrm-2.4.112.tar.xz
rm libdrm-2.4.112.tar.xz
cd libdrm-2.4.112
meson build/
sudo -E ninja -C build/ install
cd

wget https://gitlab.freedesktop.org/mesa/mesa/-/archive/mesa-22.1.6/mesa-mesa-22.1.6.tar.gz
tar xf mesa-mesa-22.1.6.tar.gz
cd mesa-mesa-22.1.6

meson build/ --optimization s --buildtype release --prefix=/usr/local --libdir=lib/arm-linux-gnueabihf \
-Dgallium-drivers=lima -Dplatforms=x11 -Dvulkan-drivers= -Ddri-drivers= \
-Dllvm=false

ninja -C build/

sudo -E ninja -C build/ install

#meson build/ --optimization s --buildtype release --prefix=/usr/local --libdir=lib/arm-linux-gnueabihf \
#-Dgallium-drivers=lima -Dplatforms=x11 -Dvulkan-drivers= -Ddri-drivers=sun4i_drm \
#-Dllvm=false
#
#ninja -C build/
#
#sudo -E ninja -C build/ install

#meson build/
#ninja -C build/
#sudo ninja -C build/ install

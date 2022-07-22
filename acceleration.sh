#run this script with 'sudo -E bash acceleration.sh'

g++ zlib1g-dev libexpat1-dev libdrm-dev flex bison libx11-dev libxext-dev libxdamage-dev \
libxcb-glx0-dev libx11-xcb-dev libxcb-dri2-0-dev libxcb-dri3-dev libxcb-present-dev libxshmfence-dev \
libxxf86vm-dev libxrandr-dev x11proto-gl-dev x11proto-dri2-dev gettext pkg-config

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
apt -y install xinit xorg
apt -y install mesa-utils

pip3 install setuptools
pip3 install meson
pip3 install Mako
. ~/.profile

wget https://dri.freedesktop.org/libdrm/libdrm-2.4.112.tar.xz
tar xf libdrm-2.4.112.tar.xz
rm libdrm-2.4.112.tar.xz
cd libdrm-2.4.112
###
meson build/
ninja -C build/ install
###
cd

wget https://github.com/Kitware/CMake/releases/download/v3.23.2/cmake-3.23.2.tar.gz
tar xf cmake-3.23.2.tar.gz
rm cmake-3.23.2.tar.gz
cd cmake-3.23.2
./bootstrap
make
make install
cd

git clone https://github.com/llvm/llvm-project.git
cd llvm-project
cmake -S llvm -B build -G Ninja
cmake --build build
cd

git clone https://gitlab.freedesktop.org/mesa/mesa.git
cd mesa/

meson build/ --optimization s --buildtype release --prefix=/usr/local --libdir=lib/arm-linux-gnueabihf \
-Dgallium-drivers=lima,panfrost,kmsro,swrast -Dplatforms=x11 -Dvulkan-drivers= -Ddri-drivers= \
-Dllvm=false

ninja -C build/

sudo ninja -C build/ install
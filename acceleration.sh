#download latest Mesa from https://archive.mesa3d.org/
#unpack with tar xf mesa-Y.N.P.tar.xz(kz) and cd to mesa-Y.N.P
#run this script

sudo apt install build-essential
sudo apt install ninja-build

pip3 install meson
pip3 install Mako
. ~/.profile

#sudo apt install gcc-mingw-w64 #May work without this line

https://github.com/Kitware/CMake/releases/download/v3.23.2/cmake-3.23.2.tar.gz
tar xf cmake-3.23.2.tar.gz
./bootstrap
make
sudo make install
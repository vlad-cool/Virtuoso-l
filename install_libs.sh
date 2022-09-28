sudo apt -y install gcc-8
sudo update-alternatives --remove-all gcc 
sudo update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-8 20

sudo mkdir -p /var/lib/bananapi
sudo touch /var/lib/bananapi/board.sh
echo "BOARD=bpi-m2z"      | sudo tee    /var/lib/bananapi/board.sh
echo "BOARD_AUTO=bpi-m2z" | sudo tee -a /var/lib/bananapi/board.sh
echo "BOARD_OLD=bpi-m64"  | sudo tee -a /var/lib/bananapi/board.sh
git clone https://github.com/BPI-SINOVOIP/BPI-WiringPi2.git
cd BPI-WiringPi2
sudo ./build
cd

sudo adduser "$USER" render
sudo apt -y update
sudo apt -y upgrade
sudo apt -y install libglfw3-dev libglu1-mesa-dev
sudo apt -y install build-essential
sudo apt -y install ffmpeg
sudo apt -y install libavcodec-dev
sudo apt -y install libavfilter-dev
sudo apt -y install libavdevice-dev
sudo apt -y install libsdl2-dev libsdl2-image-dev libsdl2-mixer-dev libsdl2-ttf-dev
sudo apt -y install mesa-utils
sudo apt -y install python3-setuptools python3-pip python3-dev git-core
sudo apt -y install pkg-config libgstreamer1.0-dev gstreamer1.0-plugins-{bad,base,good,ugly} gstreamer1.0-{omx,alsa} libmtdev-dev xclip xsel
sudo apt -y install libfreetype6-dev libgl1-mesa-dev libgles2-mesa-dev libdrm-dev libgbm-dev libudev-dev libasound2-dev liblzma-dev libjpeg-dev libtiff-dev libwebp-dev git build-essential
sudo apt -y install gir1.2-ibus-1.0 libdbus-1-dev libegl1-mesa-dev libibus-1.0-5 libibus-1.0-dev libice-dev libsm-dev libsndio-dev libwayland-bin libwayland-dev libxi-dev libxinerama-dev libxkbcommon-dev libxrandr-dev libxss-dev libxt-dev libxv-dev x11proto-randr-dev x11proto-scrnsaver-dev x11proto-video-dev x11proto-xinerama-dev

wget https://libsdl.org/release/SDL2-2.0.10.tar.gz
tar -zxvf SDL2-2.0.10.tar.gz
pushd SDL2-2.0.10
./configure --enable-video-kmsdrm --disable-video-opengl --disable-video-x11 --disable-video-rpi
make -j$(nproc)
sudo make install
popd

wget https://libsdl.org/projects/SDL_image/release/SDL2_image-2.0.5.tar.gz
tar -zxvf SDL2_image-2.0.5.tar.gz
cd SDL2_image-2.0.5
./configure
make -j$(nproc)
sudo make install
cd

wget https://libsdl.org/projects/SDL_mixer/release/SDL2_mixer-2.0.4.tar.gz
tar -zxvf SDL2_mixer-2.0.4.tar.gz
cd SDL2_mixer-2.0.4
./configure
make -j$(nproc)
sudo make install
cd

wget https://libsdl.org/projects/SDL_ttf/release/SDL2_ttf-2.0.15.tar.gz
tar -zxvf SDL2_ttf-2.0.15.tar.gz
cd SDL2_ttf-2.0.15
./configure
make -j$(nproc)
sudo make install
cd

python3 -m pip install --upgrade pip setuptools virtualenv
python3 -m pip install Pillow
python3 -m pip install "kivy[base]"
python3 -m pip install pyserial
python3 -m pip install setuptools
python3 -m pip install cython
python3 -m pip install clang
python3 -m pip install ffpyplayer

sudo apt autoremove

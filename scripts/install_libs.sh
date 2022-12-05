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
sudo apt -y install python3-setuptools python3-pip python3-dev python3-venv git-core python3-opencv
sudo apt -y install pkg-config libgstreamer1.0-dev gstreamer1.0-plugins-{bad,base,good,ugly} gstreamer1.0-{omx,alsa} libmtdev-dev xclip xsel
sudo apt -y install libfreetype6-dev libgl1-mesa-dev libgles2-mesa-dev libdrm-dev libgbm-dev libudev-dev libasound2-dev liblzma-dev libjpeg-dev libtiff-dev libwebp-dev git build-essential
sudo apt -y install gir1.2-ibus-1.0 libdbus-1-dev libegl1-mesa-dev libibus-1.0-5 libibus-1.0-dev libice-dev libsm-dev libsndio-dev libwayland-bin libwayland-dev libxi-dev libxinerama-dev libxkbcommon-dev libxrandr-dev libxss-dev libxt-dev libxv-dev x11proto-randr-dev x11proto-scrnsaver-dev x11proto-video-dev x11proto-xinerama-dev

wget https://libsdl.org/release/SDL2-2.0.10.tar.gz
tar -zxvf SDL2-2.0.10.tar.gz
cd SDL2-2.0.10
./configure --enable-video-kmsdrm --disable-video-opengl --disable-video-x11 --disable-video-rpi
make -j$(nproc)
sudo make install
cd

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

mkdir -p V24m
cd V24m
sudo mount -o remount,size=4G /tmp/
python3 -m pip install --upgrade pip setuptools virtualenv
python3 -m venv venv
venv/bin/python3 -m pip install --upgrade pip setuptools virtualenv
venv/bin/python3 -m pip install Pillow
venv/bin/python3 -m pip install setuptools
venv/bin/python3 -m pip install cython
venv/bin/python3 -m pip install clang
venv/bin/python3 -m pip install "kivy[full]"
venv/bin/python3 -m pip install pyserial
#venv/bin/python3 -m pip install opencv-python
sudo chmod 4775 venv/bin/python3
cd

./kivy_config.py

./configure --disable-stripping --pkg-config=/work/buildroot/output/host/bin/pkg-config --disable-static --enable-shared --prefix=/usr --enable-avfilter --disable-version3 --enable-logging --enable-optimizations --disable-extra-warnings --enable-avdevice --enable-avcodec --enable-avformat --enable-network --disable-gray --enable-swscale-alpha --disable-small --enable-dct --enable-fft --enable-mdct --enable-rdft --disable-crystalhd --disable-dxva2 --enable-runtime-cpudetect --disable-hardcoded-tables --disable-mipsdsp --disable-mipsdspr2 --disable-msa --enable-hwaccels --disable-cuda --disable-cuvid --disable-nvenc --disable-avisynth --disable-frei0r --disable-libopencore-amrnb --disable-libopencore-amrwb --disable-libdc1394 --disable-libgsm --disable-libilbc --disable-libvo-amrwbenc --disable-symver --disable-doc --enable-gpl --enable-nonfree --enable-ffmpeg --enable-ffplay --enable-libv4l2 --enable-swresample --enable-ffprobe --disable-libxcb --enable-postproc --enable-swscale --enable-indevs --enable-alsa --enable-outdevs --enable-pthreads --enable-zlib --disable-bzlib --disable-libfdk-aac --disable-libcdio --disable-openssl --enable-libopenh264 --enable-vaapi --enable-vdpau --disable-mmal --disable-omx --disable-omx-rpi --disable-libopencv --disable-libopus --enable-libvpx --disable-libass --disable-libbluray --disable-libmfx --disable-librtmp --disable-libmp3lame --disable-libmodplug --enable-libspeex --disable-libtheora --disable-iconv --enable-libfreetype --enable-fontconfig --enable-libopenjpeg --enable-libx264 --disable-libx265 --disable-libdav1d --disable-x86asm --disable-mmx --disable-sse --disable-sse2 --disable-sse3 --disable-ssse3 --disable-sse4 --disable-sse42 --disable-avx --disable-avx2 --enable-armv6 --enable-vfp --enable-neon --disable-altivec --extra-libs=-latomic --enable-pic --cpu=cortex-a7 --enable-encoder=cedrus264

./configure --extra-cflags=-mfloat-abi=soft --prefix=/usr --enable-nonfree --enable-gpl --enable-version3 --enable-vdpau --enable-libx264 --enable-libmp3lame --enable-libpulse --enable-libv4l2


--enable-cross-compile --cross-prefix=/work/buildroot/output/host/bin/arm-buildroot-linux-gnueabihf- --sysroot=/work/buildroot/output/host/arm-buildroot-linux-gnueabihf/sysroot --host-cc=/usr/bin/gcc --arch=arm --target-os=linux --disable-stripping --pkg-config=/work/buildroot/output/host/bin/pkg-config --disable-static --enable-shared --prefix=/usr --enable-avfilter --disable-version3 --enable-logging --enable-optimizations --disable-extra-warnings --enable-avdevice --enable-avcodec --enable-avformat --enable-network --disable-gray --enable-swscale-alpha --disable-small --enable-dct --enable-fft --enable-mdct --enable-rdft --disable-crystalhd --disable-dxva2 --enable-runtime-cpudetect --disable-hardcoded-tables --disable-mipsdsp --disable-mipsdspr2 --disable-msa --enable-hwaccels --disable-cuda --disable-cuvid --disable-nvenc --disable-avisynth --disable-frei0r --disable-libopencore-amrnb --disable-libopencore-amrwb --disable-libdc1394 --disable-libgsm --disable-libilbc --disable-libvo-amrwbenc --disable-symver --disable-doc --enable-gpl --enable-nonfree --enable-ffmpeg --enable-ffplay --enable-libv4l2 --enable-swresample --enable-ffprobe --disable-libxcb --enable-postproc --enable-swscale --enable-indevs --enable-alsa --enable-outdevs --enable-pthreads --enable-zlib --disable-bzlib --disable-libfdk-aac --disable-libcdio --enable-gnutls --disable-openssl --enable-libdrm --enable-libopenh264 --enable-vaapi --enable-vdpau --disable-mmal --disable-omx --disable-omx-rpi --disable-libopencv --disable-libopus --enable-libvpx --disable-libass --disable-libbluray --disable-libmfx --disable-librtmp --disable-libmp3lame --disable-libmodplug --enable-libspeex --disable-libtheora --disable-iconv --enable-libfreetype --enable-fontconfig --enable-libopenjpeg --enable-libx264 --disable-libx265 --disable-libdav1d --disable-x86asm --disable-mmx --disable-sse --disable-sse2 --disable-sse3 --disable-ssse3 --disable-sse4 --disable-sse42 --disable-avx --disable-avx2 --enable-armv6 --enable-vfp --enable-neon --disable-altivec --extra-libs=-latomic --enable-pic --cpu=cortex-a7 --enable-encoder=cedrus264
  libavutil      57.  7.100 / 57.  7.100
#!/bin/sh
sudo apt update
sudo apt -y install libvdpau-dev
sudo apt -y install libpixman-1-dev
cd libcedrus
sudo make install
cd ..
cd libvdpau-sunxi
sudo make install
sudo ldconfig
cd ..
sudo apt -y install v4l-utils libmp3lame-dev libpulse-dev libv4l-dev
cd x264
sudo make install
sudo ldconfig
cd ..
cd FFmpeg-Cedrus
sudo make install
cd ..

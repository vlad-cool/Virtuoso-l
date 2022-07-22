#copy with 'scp install_kivy.sh "pi"@192.168.2.5:V24m' (mkdir project first) 
#run with 'sudo -E bash install_kivy.sh'

sudo apt -y update
sudo apt -y upgrade
sudo apt -y install python3-pip
sudo apt -y install python3-dev
sudo apt -y install libglfw3-dev libgl1-mesa-dev libglu1-mesa-dev
sudo apt -y install build-essential
sudo apt -y install ffmpeg
sudo apt -y install libavcodec-dev
sudo apt -y install libavfilter-dev
sudo apt -y install libavdevice-dev
sudo apt -y install pkg-config libgles2-mesa-dev \
   libgstreamer1.0-dev \
   gstreamer1.0-plugins-{bad,base,good,ugly} \
   gstreamer1.0-{omx,alsa} libmtdev-dev \
   xclip xsel libjpeg-dev
sudo apt -y install libsdl2-dev libsdl2-image-dev libsdl2-mixer-dev libsdl2-ttf-dev

pip3 install --upgrade pip wheel setuptools virtualenv
pip3 install setuptools
pip3 install --upgrade pip setuptools
pip3 install cython
pip3 install clang
pip3 install ffpyplayer
export PATH=/home/vlad/.local/bin:$PATH #!!!
sudo systemctl enable multi-user.target --force
sudo systemctl set-default multi-user.target
sudo apt -y update
sudo apt -y upgrade

pip3 install kivy
pip3 install pyinstaller
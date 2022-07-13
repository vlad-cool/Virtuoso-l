#copy with 'scp install_kivy.sh "user"@192.168.2.5:project' (mkdir project first) 
#run with 'sudo bash install_kivy.sh'

sudo apt -y update
sudo apt -y upgrade
sudo apt -y install python3-pip
sudo apt -y install python3-dev
sudo apt -y install libglfw3-dev libgl1-mesa-dev libglu1-mesa-dev
pip3 install cython
export PATH=/home/vlad/.local/bin:$PATH
sudo apt -y update
sudo apt -y upgrade
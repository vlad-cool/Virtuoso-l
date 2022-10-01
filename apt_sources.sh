echo 'deb http://old-releases.ubuntu.com/ubuntu hirsute main restricted universe multiverse'           | sudo tee    /etc/apt/sources.list
echo 'deb http://old-releases.ubuntu.com/ubuntu hirsute-security main restricted universe multiverse'  | sudo tee -a /etc/apt/sources.list
echo 'deb http://old-releases.ubuntu.com/ubuntu hirsute-updates main restricted universe multiverse'   | sudo tee -a /etc/apt/sources.list
echo 'deb http://old-releases.ubuntu.com/ubuntu hirsute-backports main restricted universe multiverse' | sudo tee -a /etc/apt/sources.list

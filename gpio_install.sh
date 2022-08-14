#copy with 'scp gpio_install.sh pi@192.168.2.5:V24m' (mkdir project first) 
#run this script with 'sudo -E bash gpio_install.sh'

#git clone https://github.com/BPI-SINOVOIP/BPI-WiringPi2-Python.git -b BPI_M1_M1Plus
#cd BPI-WiringPi2-Python
#sudo python3 setup.py install

pip3 install wiringpi

sudo apt -y update
sudo apt -y upgrade
sudo apt -y install python3-pip
sudo apt -y install python3-dev
sudo apt -y install build-essential

sudo apt -y install python-serial
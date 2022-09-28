pip3 install kivy
pip3 install pyserial

alias gcc='gcc -fcommon'
git clone https://github.com/BPI-SINOVOIP/BPI-WiringPi2.git
cd BPI-WiringPi2
sudo mkdir -p /var/lib/bananapi
sudo touch /var/lib/bananapi/board.sh
echo "BOARD=bpi-m2z"      | sudo tee    /var/lib/bananapi/board.sh
echo "BOARD_AUTO=bpi-m2z" | sudo tee -a /var/lib/bananapi/board.sh
echo "BOARD_OLD=bpi-m64"  | sudo tee -a /var/lib/bananapi/board.sh

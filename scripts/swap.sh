#creates swap files (8x256MB=2GB)
#run with 'bash swap.sh'

cd

sudo dd if=/dev/zero of=/swap_file bs=256MB count=8
sudo chmod 600 /swap_file
sudo mkswap /swap_file
echo /swap_file swap swap defaults 0 0 | sudo tee -a /etc/fstab

#creates swap files (4x256MB=1GB)
#run with 'bash swap.sh'

cd

sudo dd if=/dev/zero of=/swap_file bs=256MB count=4
sudo chmod 600 /swap_file
sudo mkswap /swap_file
sudo chmod 600 /swap_file
sudo swapon /swap_file
echo /swap_file swap swap defaults 0 0 | sudo tee -a /etc/fstab

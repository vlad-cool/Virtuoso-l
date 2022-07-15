sudo dd if=/dev/zero of=/swap_file bs=256MB count=8
sudo chmod 600 /swap_file
sudo mkswap /swap_file
sudo chmod 600 /swap_file
sudo swapon /swap_file

#add '/swap_file swap swap defaults 0 0' to /etc/fstab
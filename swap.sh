sudo dd if=/dev/zero of=/swap_file bs=256MB count=16
sudo chmod 600 /swap_file
sudo mkswap /swap_file
sudo swapon /swap_file
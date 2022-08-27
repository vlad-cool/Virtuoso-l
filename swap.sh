#creates swap files (8x256MB=2GB)
#run with 'sudo bash swap.sh'
#add '/swap_file swap swap defaults 0 0' to /etc/fstab

cd

dd if=/dev/zero of=/swap_file bs=256MB count=8
chmod 600 /swap_file
mkswap /swap_file
chmod 600 /swap_file
swapon /swap_file

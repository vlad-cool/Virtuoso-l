# Setup
- open terminal and type `sudo armbian-config`
- in Network setup Wifi (or wired connection)
- in Network check IP adress (NOT ON THE TOP OF THE MENU, choose IP with arrows and press Enter) and remember it
- in System setup SSH (only first 2 *)
- exit armbian-config
- mkdir V24m

# Swap
Due to small amount of ram on bananapi swap file required to build some things
- on pc, connected to the same network: scp swap.sh pi@(*ip*):V24m
- `sudo bash ./V24m/swap.sh`
- `sudo nano /etc/fstab` to open text editor
- add `/swap_file swap swap defaults 0 0` to opened file
- save (Ctrl + O -> Enter) & exit (Ctrl + X)

# Acceleration
- on pc, connected to the same network: scp acceleration.sh pi@(*ip*):V24m
- `bash ./V24m/acceleration.sh`
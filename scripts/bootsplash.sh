mv Virtuoso_splash1920x360 Virtuoso_splash
mv Virtuoso_splash1920x480 Virtuoso_splash
sudo cp -r Virtuoso_splash /usr/share/plymouth/themes/
sudo update-alternatives --install /usr/share/plymouth/themes/default.plymouth default.plymouth /usr/share/plymouth/themes/Virtuoso_splash/Virtuoso_splash.plymouth 220
sudo update-alternatives --config default.plymouth
sudo update-initramfs -v -u
sudo systemctl mask plymouth-quit-wait.service
sudo systemctl mask plymouth-quit.service
rm -r Virtuoso_splash
sudo apt -y install feh

sudo cp -r V24m_splash /usr/share/plymouth/themes/
sudo update-alternatives --install /usr/share/plymouth/themes/default.plymouth default.plymouth /usr/share/plymouth/themes/V24m_splash/V24m_splash.plymouth 220
sudo update-alternatives --config default.plymouth
sudo update-initramfs -v -u
sudo systemctl mask plymouth-quit-wait.service
sudo systemctl mask plymouth-quit.service
sudo apt install feh

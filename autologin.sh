sudo apt -y install mingetty
sudo mkdir -p /etc/systemd/system/getty@tty1.service.d

echo [Service]                                               | sudo tee    /etc/systemd/system/getty@tty1.service.d/override.conf
echo ExecStart=                                              | sudo tee -a /etc/systemd/system/getty@tty1.service.d/override.conf
echo ExecStart=-/sbin/mingetty --autologin pi --noclear tty1 | sudo tee -a /etc/systemd/system/getty@tty1.service.d/override.conf

systemctl enable getty@tty1.service

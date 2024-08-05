#!/bin/sh
echo '# Default Armbian config'      | sudo tee    /etc/X11/xorg.conf.d/01-armbian-defaults.conf
echo 'Section "Monitor"'             | sudo tee -a /etc/X11/xorg.conf.d/01-armbian-defaults.conf
echo '    Identifier "HDMI-1"'       | sudo tee -a /etc/X11/xorg.conf.d/01-armbian-defaults.conf
echo '    Modeline "1920x360_60.00"' | sudo tee -a /etc/X11/xorg.conf.d/01-armbian-defaults.conf
echo 'EndSection'                    | sudo tee -a /etc/X11/xorg.conf.d/01-armbian-defaults.conf

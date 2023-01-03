#!/bin/sh

echo '# Default Armbian config                            ' | sudo tee    /etc/X11/xorg.conf.d/01-armbian-defaults.conf
echo 'Section "Monitor"                                   ' | sudo tee -a /etc/X11/xorg.conf.d/01-armbian-defaults.conf
echo '    Identifier "HDMI-1"                             ' | sudo tee -a /etc/X11/xorg.conf.d/01-armbian-defaults.conf
echo '    Option "Rotate" "left"                          ' | sudo tee -a /etc/X11/xorg.conf.d/01-armbian-defaults.conf
echo 'EndSection                                          ' | sudo tee -a /etc/X11/xorg.conf.d/01-armbian-defaults.conf
echo '                                                    ' | sudo tee -a /etc/X11/xorg.conf.d/01-armbian-defaults.conf
echo 'Section "InputClass"                                ' | sudo tee -a /etc/X11/xorg.conf.d/01-armbian-defaults.conf
echo '    Identifier "Coordinate Transformation Matrix"   ' | sudo tee -a /etc/X11/xorg.conf.d/01-armbian-defaults.conf
echo '    MatchIsTouchscreen "on"                         ' | sudo tee -a /etc/X11/xorg.conf.d/01-armbian-defaults.conf
echo '    MatchDevicePath "/dev/input/event*"             ' | sudo tee -a /etc/X11/xorg.conf.d/01-armbian-defaults.conf
echo '    MatchDriver "libinput"                          ' | sudo tee -a /etc/X11/xorg.conf.d/01-armbian-defaults.conf
echo '    Option "CalibrationMatrix" "0 -1 1 1 0 0 0 0 1" ' | sudo tee -a /etc/X11/xorg.conf.d/01-armbian-defaults.conf
echo 'EndSection                                          ' | sudo tee -a /etc/X11/xorg.conf.d/01-armbian-defaults.conf

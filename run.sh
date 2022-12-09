xrandr -o left
xinput set-prop "wch.cn USB2IIC_CTP_CONTROL" --type=float "Coordinate Transformation Matrix" 0 -1 1 1 0 0 0 0 1
cd /home/pi/V24m
sudo ./app.py

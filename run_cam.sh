#!/bin/bash
sudo media-ctl --device /dev/media1 --set-v4l2 '"ov5640 0-003c":0[fmt:YUYV8_2X8/640x480@1/60]'
./camera.py


#ffplay /dev/video1 -vf "rotate=90"
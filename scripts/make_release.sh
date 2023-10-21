#!/bin/bash

FILES="bin/venv assets bin/get_pin bin/get_rc5 bin/send_pin bin/send_rc5 app.py gpio_control.py video_control.py main1920x480.kv main1920x360.kv .bash_login start_ffmpeg.sh start_x.sh video_cutter.sh VERSION LICENSE"
VERSION=$(cat VERSION)

if [ ! -f releases/$VERSION.tar.gz ]; then
    tar -cvf releases/$VERSION.tar.gz --transform='s/^.\///' $FILES
else
    echo Release archive exists, change VERSION, aborting
fi

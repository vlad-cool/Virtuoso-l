#!/bin/sh

# V24m configuring
export is_banana=False
export input_support=True
export config_file=config.json
export kivy_file=main1920x480.kv
## Video
export video_support=True
export video_path=/home/vlad/Documents/VSCode/V24m
export video_path_tmp=./video/tmp
export video_encoder=libx264

cd src
../venv/bin/python3 app.py
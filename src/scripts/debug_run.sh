#!/bin/sh

export is_banana=False
export input_support=True
export config_file=config.json
export kivy_file=main1920x480.kv
export video_support=True
export video_path=$(pwd)

python3 src/app.py

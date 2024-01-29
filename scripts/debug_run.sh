#!/bin/sh

# V24m configuring
export is_banana=False
export input_support=False
export config_file=config.json
export kivy_file=main1920x360.kv
## Video
export video_support=False
export video_path=./video
export video_path_tmp=./video/tmp
export video_encoder=libx264

cd src
../venv/bin/python3 app.py
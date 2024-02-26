#!/bin/sh

# V24m configuring
export IS_BANANA=False
export INPUT_SUPPORT=True
export CONFIG_FILE=config.json
export KIVY_FILE=main1920x480.kv
## Video
export VIDEO_SUPPORT=True
export VIDEO_PATH=/home/vlad/Documents/VSCode/V24m/test_videos
export VIDEO_PATH_TMP=./video/tmp
export VIDEO_ENCODER=libx264

cd src
../venv/bin/python3 -m cProfile -s time -o local_prof app.py
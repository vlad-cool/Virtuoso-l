#!/bin/sh

# V24m configuring
export IS_BANANA=False
export INPUT_SUPPORT=True
export CONFIG_FILE=config.json
export KIVY_FILE=main1920x360.kv
## Video
export VIDEO_SUPPORT=True
export VIDEO_PATH=/home/vlad/Documents/VSCode/V24m/test_videos
export VIDEO_PATH_TMP=./video/tmp
export VIDEO_ENCODER=libx264

cd src/template
make

cd ..
../venv/bin/python3 app.py
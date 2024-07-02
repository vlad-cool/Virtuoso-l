#!/bin/sh

# Virtuoso configuring
export IS_BANANA=False
export INPUT_SUPPORT=True
export CONFIG_FILE=config.json
export KIVY_FILE=main1920x480.kv
## Video
export VIDEO_SUPPORT=True
export VIDEO_PATH=/home/vlad/Documents/VSCode/Virtuoso-l/test_videos
export VIDEO_PATH_TMP=./video/tmp
export VIDEO_ENCODER=libx264
export CAMERA_PATH=/dev/video1
## Updates
export UPDATE_DIR=~/Downloads

set -e

cd src/template
make

cd ..
../venv/bin/python3 app.py
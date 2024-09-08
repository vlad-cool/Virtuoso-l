#!/bin/sh

case "$1" in
    1920x360)
        export KCFG_graphics_width=1920
        export KCFG_graphics_height=360
        export KIVY_FILE=main1920x360.kv
    ;;
    1920x480)
        export KCFG_graphics_width=1920
        export KCFG_graphics_height=480
        export KIVY_FILE=main1920x480.kv
    ;;
    1920x550)
        export KCFG_graphics_width=1920
        export KCFG_graphics_height=550
        export KIVY_FILE=main1920x550.kv
    ;;
    *)
        if [ -z "$1" ]; then
            echo "No argument provided. Default behavior."
            export KCFG_graphics_width=1920
            export KCFG_graphics_height=480
            export KIVY_FILE=main1920x480.kv
        else
            echo "Invalid option: $1"
            exit
        fi
    ;;
esac

# Virtuoso configuring
export IS_BANANA=False
export INPUT_SUPPORT=True
export CONFIG_FILE=config.json
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
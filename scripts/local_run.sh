#!/bin/sh

case "$1" in
    1920x360)
        export KCFG_graphics_width=1920
        export KCFG_graphics_height=360
        export KIVY_FILE=main1920x360.kv
        export VIDEO_SUPPORT=True
    ;;
    1920x480)
        export KCFG_graphics_width=1920
        export KCFG_graphics_height=480
        export KIVY_FILE=main1920x480.kv
        export VIDEO_SUPPORT=True
    ;;
    1920x550)
        export KCFG_graphics_width=1920
        export KCFG_graphics_height=550
        export KIVY_FILE=main1920x550.kv
        export VIDEO_SUPPORT=True
    ;;
    1920x360_no_video)
        export KCFG_graphics_width=1920
        export KCFG_graphics_height=260
        export KIVY_FILE=main1920x360_no_video.kv
        export VIDEO_SUPPORT=False
    ;;
    *)
        if [ -z "$1" ]; then
            echo "No argument provided. Default behavior."
            export KCFG_graphics_width=1920
            export KCFG_graphics_height=360
            export KIVY_FILE=main1920x360_no_video.kv
            export VIDEO_SUPPORT=False
        else
            echo "Invalid option: $1"
            exit
        fi
    ;;
esac

# Virtuoso configuring
export IS_BANANA=False
export INPUT_SUPPORT=True
export CONFIG_FILE=/tmp/Virtuoso_config.json
## Video
# export VIDEO_PATH=$(pwd)/video
# export VIDEO_PATH_TMP=$(pwd)/video/tmp
# export VIDEO_ENCODER=libx264
# export CAMERA_PATH=/dev/video0
# export COMPRESS_METADATA=bz2
# export VIDEO_LAG=0

export RECORDER_LOG_OUT=$(pwd)/logs/RECORDER_LOG_OUT
export RECORDER_LOG_ERR=$(pwd)/logs/RECORDER_LOG_ERR
export CUTTER_LOG_OUT=$(pwd)/logs/CUTTER_LOG_OUT
export CUTTER_LOG_ERR=$(pwd)/logs/CUTTER_LOG_ERR

## Updates
export UPDATE_DIR=~/Downloads

set -e

cd src/template
make

cd ..
../venv/bin/python3 app.py

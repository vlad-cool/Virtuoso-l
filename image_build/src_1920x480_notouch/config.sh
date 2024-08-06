# Virtuoso configuring
## General
export IS_BANANA=True
export INPUT_SUPPORT=False
export CONFIG_FILE=/home/pi/Virtuoso/config.json
export KIVY_FILE=main1920x480.kv
## Video
export VIDEO_SUPPORT=True
export VIDEO_PATH=/home/pi/Virtuoso/Videos
export VIDEO_PATH_TMP=/home/pi/Virtuoso/app/Videos_tmp
export VIDEO_ENCODER=cedrus264
export CAMERA_PATH=/dev/video0
mkdir -p $VIDEO_PATH
mkdir -p $VIDEO_PATH_TMP
## Logs
export MAIN_LOG_OUT=/dev/null
export MAIN_LOG_ERR=/dev/null
export RECORDER_LOG_OUT=/dev/null
export RECORDER_LOG_ERR=/dev/null
export CUTTER_LOG_OUT=/dev/null
export CUTTER_LOG_ERR=/dev/null
## Updates
export UPDATE_DIR=/home/pi/Virtuoso
## Splash
export SPLASH=Virtuoso_splash1920x480

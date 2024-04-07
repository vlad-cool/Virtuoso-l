# V24m configuring
## General
export IS_BANANA=True
export INPUT_SUPPORT=True
export CONFIG_FILE=/home/pi/V24m/config.json
export KIVY_FILE=main1920x480.kv
## Video
export VIDEO_SUPPORT=True
export VIDEO_PATH=/home/pi/V24m/Videos
export VIDEO_PATH_TMP=/home/pi/V24m/app/Videos_tmp
mkdir -p $VIDEO_PATH
mkdir -p $VIDEO_PATH_TMP
export VIDEO_ENCODER=cedrus264
## Logs
export MAIN_LOG=/home/pi/V24m/logs/main.log
export FFMPEG_LOG=/home/pi/V24m/logs/ffmpeg.log
export CUTTER_LOG=/home/pi/V24m/logs/cutter.log
## Updates
export UPDATE_DIR=/home/pi/V24m

if [ -z "$SSH_TTY" ] && [ -z "$TMUX" ] && [[ "$(tty)"="/dev/tty1" ]]
then
    startx ./start_x.sh &> $MAIN_LOG
    # /usr/sbin/reboot
fi

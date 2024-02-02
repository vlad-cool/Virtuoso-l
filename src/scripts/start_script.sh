# V24m configuring
## General
export is_banana=True
export input_support=True
export config_file=/home/pi/V24m/config.json
export kivy_file=main1920x480.kv
## Video
export video_support=True
export video_path=/home/pi/V24m/Videos
export video_path_tmp=/home/pi/V24m/app/Videos_tmp
mkdir -p $video_path
mkdir -p $video_path_tmp
export video_encoder=cedrus264
## Logs
export MAIN_LOG=main.log
export FFMPEG_LOG=ffmpeg.log
export CUTTER_LOG=cutter.log

if [ -z "$SSH_TTY" ] && [ -z "$TMUX" ] && [[ "$(tty)"="/dev/tty1" ]]
then
    startx ./start_x.sh &> $MAIN_LOG
    #/usr/sbin/reboot
fi

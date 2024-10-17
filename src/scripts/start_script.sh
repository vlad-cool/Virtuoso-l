source $HOME/config.sh

v4l2-ctl --set-fmt-video=width=640,height=480 --device=/dev/video0 > /dev/null 2>&1

if [ -z "$SSH_TTY" ] && [ -z "$TMUX" ] && [[ "$(tty)"="/dev/tty1" ]]
then
    startx ./start_x.sh > $MAIN_LOG_OUT 2> $MAIN_LOG_ERR
fi

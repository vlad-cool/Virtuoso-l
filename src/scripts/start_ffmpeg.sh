#!/bin/bash
ffmpeg -y -f v4l2 -video_size 640x480 -i /dev/video0 -vf "transpose=2,transpose=2" -r 30 -c:v $ENCODER -pix_fmt nv12 $TMP_DIR/$VIDEO_NAME.mp4 > $FFMPEG_LOG 2>&1
#!/bin/bash
# media-ctl --device /dev/media0 --set-v4l2 '"ov5640 0-003c":0[fmt:UYVY8_2X8/640x480@1/30'
ffmpeg -y -f v4l2 -video_size 640x480 -i /dev/video0 -vf "transpose=2,transpose=2" -c:v $VIDEO_ENCODER -pix_fmt nv12 $VIDEO_PATH_TMP/$VIDEO_NAME.mp4 > $FFMPEG_LOG 2>&1

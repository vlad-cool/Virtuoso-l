#!/bin/bash
set -e

FPS=30
RESOLUTION=640x480

VIDEO=$VIDEO_PATH_TMP/$VIDEO_NAME.mp4
ffmpeg -y -f v4l2 -video_size $RESOLUTION -framerate $FPS -i /dev/video0 -vf "transpose=2,transpose=2" -c:v $VIDEO_ENCODER -pix_fmt nv12 $VIDEO

END_TIME=$(./get_time)

DURATION=$(gst-discoverer-1.0 $VIDEO | grep Duration | head -n 1)
IFS=':' read -ra parts <<< "$DURATION"
DURATION=$(bc <<< "scale=3; ${parts[1]} * 60 * 60 + ${parts[2]} * 60 + ${parts[3]}")

START_TIME=$(bc <<< "scale=3; $END_TIME - $DURATION")

read n

max=0
for file in $VIDEO_PATH/*.mp4; do
    file=$(basename "$file")
    num=${file%.*}
    if [ "$num" -gt "$max" ]; then
        max=$num
    fi
done

j=$max

for ((i=0; i<n; i++))
do
    j=$(($j+1))
    read CLIP_START_TIME
    read CLIP_DURATION
    read CLIP_METADATA
    
    CLIP_START_POS=$(bc <<< "scale=3; $CLIP_START_TIME - $START_TIME")

    cat > EDL <<< "0 $CLIP_START_POS 0"

    VIDEO_CUT=$VIDEO_PATH_TMP/$VIDEO_NAME.CUT_$i.mp4

    mencoder $VIDEO -hr-edl-seek -edl EDL -endpos $CLIP_DURATION -ovc x264 -info comment=$CLIP_METADATA -o $VIDEO_CUT > $CUTTER_LOG_OUT 2> $CUTTER_LOG_ERR

    mv $VIDEO_CUT $VIDEO_PATH/$j.mp4
done

max=$j

for file in $VIDEO_PATH/*.mp4; do
    file=$(basename "$file")
    num=${file%.*}
    if [ $(($max-$num)) -gt "99" ]; then
        rm $VIDEO_PATH/$num.mp4
    fi
done

rm $TMP_DIR/$VIDEO

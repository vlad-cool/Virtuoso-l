#!/bin/bash
set -e
VIDEO=$VIDEO_PATH_TMP/$VIDEO_NAME.mp4
ffmpeg -y -f v4l2 -video_size 640x480 -i /dev/video0 -vf "transpose=2,transpose=2" -c:v $VIDEO_ENCODER -pix_fmt nv12 $VIDEO
END_TIME=$(./get_time)

DURATION=$(gst-discoverer-1.0 $VIDEO | grep Duration | head -n 1)
IFS=':' read -ra parts <<< "$DURATION"
DURATION=$(bc <<< "scale=3; ${parts[1]} * 60 * 60 + ${parts[2]} * 60 + ${parts[3]}")

START_TIME=$(bc <<< "scale=3; $END_TIME - $DURATION")

echo $START_TIME $DURATION $END_TIME

read n

echo n $n

max=0
for file in $VIDEO_PATH/*.mp4; do
    file=$(basename "$file")
    num=${file%.*}
    if [ "$num" -gt "$max" ]; then
        max=$num
    fi
done

echo AAA

# for file in $VIDEO_PATH/*.mp4; do
#     file=$(basename "$file")
#     num=${file%.*}
#     if [ $(($max-$num)) -gt "99" ]; then
#         rm $VIDEO_PATH/$num.mp4
#     fi
# done

echo BBB

j=$max

# format_time() {
#     total_seconds=$1
#     min_seconds=$2
#     max_seconds=$3

#     number=3.6789

#     total_seconds=$(printf "%.0f" $total_seconds)
#     min_seconds=$(printf "%.0f" $min_seconds)
#     max_seconds=$(printf "%.0f" $max_seconds)

#     if (( $(echo "$total_seconds < $min_seconds" | bc -l) )); then
#         total_seconds=$min_seconds
#     elif (( $(echo "$total_seconds > $max_seconds" | bc -l) )); then
#         total_seconds=$max_seconds
#     fi

#     echo $total_seconds

#     # hours=$(bc <<< "scale=0; $total_seconds/3600")
#     # minutes=$(bc <<< "scale=0; ($total_seconds/60)%60")
#     # seconds=$(bc <<< "scale=0; $total_seconds%60")
#     # 
#     # printf "%02d:%02d:%.2f\n" $hours $minutes $seconds
# }

echo CCC

for ((i=0; i<n; i++))
do
    j=$(($j+1))
    read CLIP_START_TIME
    read CLIP_DURATION
    read CLIP_METADATA

    # echo "################################"
    # echo "scale=3; $CLIP_START_TIME - $START_TIME"
    # echo "################################"
    
    CLIP_START_POS=$(bc <<< "scale=3; $CLIP_START_TIME - $START_TIME")

    # CLIP_START_POS=$(format_time $(bc <<< "scale=3; $CLIP_START_TIME - $START_TIME") 0 $CLIP_DURATION)

    # echo ABOBA $CLIP_START_POS $CLIP_DURATION
    # echo "################################"

    # echo "$VIDEO -ss $CLIP_START_POS -endpos $(format_time $CLIP_DURATION $CLIP_DURATION $CLIP_DURATION) -ovc copy -info comment=$CLIP_METADATA -o $VIDEO_PATH/$j.mp4 > $CUTTER_LOG 2>&1"
    # echo "################################"

    cat > EDL <<< "0 $CLIP_START_POS 0"

    # mencoder $VIDEO -ss $CLIP_START_POS -endpos $(format_time CLIP_DURATION CLIP_DURATION CLIP_DURATION) -ovc copy -info comment=$CLIP_METADATA -o $VIDEO_PATH/$j.mp4 > $CUTTER_LOG 2>&1
    mencoder $VIDEO -hr-edl-seek -edl EDL -endpos $CLIP_DURATION -ovc x264 -info comment=$CLIP_METADATA -o $VIDEO_PATH/$j.mp4 > $CUTTER_LOG 2>&1
    
    echo ABOBA

    echo "mencoder $VIDEO -hr-edl-seek -edl EDL -endpos $CLIP_DURATION -ovc x264 -info comment=$CLIP_METADATA -o $VIDEO_PATH/$j.mp4 > $CUTTER_LOG 2>&1"
    echo OBABO
done

echo DDD

# mencoder test_videos/0.mp4 -ss 4.5 -endpos 10 -ovc x264 -o test.mp4
# mencoder test_videos/0.mp4 -hr-edl-seek -ss 4.5 -endpos 10 -ovc x264 -o test.mp4
# mencoder test_videos/0.mp4 -hr-edl-seek -edl test_edl -endpos 10 -ovc x264 -o test.mp4

# rm $TMP_DIR/$VIDEO

max=$j

echo EEE

for file in $VIDEO_PATH/*.mp4; do
    file=$(basename "$file")
    num=${file%.*}
    if [ $(($max-$num)) -gt "99" ]; then
        rm $VIDEO_PATH/$num.mp4
    fi
done

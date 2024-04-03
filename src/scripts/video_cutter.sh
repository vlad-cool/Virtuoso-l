#!/bin/bash
read VIDEO_NAME
read n

max=0
for file in $VIDEO_PATH/*.mp4; do
    file=$(basename "$file")
    num=${file%.*}
    if [ "$num" -gt "$max" ]; then
        max=$num
    fi
done

for file in $VIDEO_PATH/*.mp4; do
    file=$(basename "$file")
    num=${file%.*}
    if [ $(($max-$num)) -gt "99" ]; then
        rm $VIDEO_PATH/$num.mp4
    fi
done

j=$(($max+1))

for ((i=0; i<n; i++))
do
    read START_TIME
    read END_TIME
    read METADATA
    mencoder $VIDEO_PATH_TMP/$VIDEO_NAME.mp4 -info comment=$METADATA -ss $START_TIME -endpos $END_TIME -ovc copy -o $VIDEO_PATH/$j.mp4 > $CUTTER_LOG 2>&1
    # ffmpeg -i $VIDEO_PATH_TMP/$VIDEO_NAE.mp4 -movflags use_metadata_tags -metadata fencing=$METADATA $VIDEO_PATH_TMP/$VIDEO_NAME.mp4
    j=$(($j+1))
done

rm $TMP_DIR/$VIDEO_NAME.mp4

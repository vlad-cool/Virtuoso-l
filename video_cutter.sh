#!/bin/bash
read VIDEO_NAME
read n

j=1

while [[ -e $OUT_DIR/${j}.mp4 ]]
do
    j=$(($j+1))
done

for ((i=0; i<n; i++))
do
    read START_TIME
    read CLIP_DURATION
    mencoder $TMP_DIR/$VIDEO_NAME.mp4 -ss $START_TIME -endpos $CLIP_DURATION -ovc copy  -o $OUT_DIR/$j.mp4 > $CUTTER_LOG 2>&1
    j=$(($j+1))
done

rm $HOME/Videos/V24m/tmp/$VIDEO_NAME.mp4
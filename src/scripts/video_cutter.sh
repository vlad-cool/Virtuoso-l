#!/bin/bash
read VIDEO_NAME
read n

j=0

while [[ -e $VIDEO_PATH/${j}.mp4 ]]
do
    j=$(($j+1))
done

for ((i=0; i<n; i++))
do
    read START_TIME
    read END_TIME
    echo $VIDEO_PATH_TMP/$VIDEO_NAME.mp4 -ss $START_TIME -endpos $END_TIME -ovc copy  -o $VIDEO_PATH/$j.mp4 | tee -a cutter_commands
    mencoder $VIDEO_PATH_TMP/$VIDEO_NAME.mp4 -ss $START_TIME -endpos $END_TIME -ovc copy  -o $VIDEO_PATH/$j.mp4 >> $CUTTER_LOG 2>&1
    j=$(($j+1))
done

rm $TMP_DIR/$VIDEO_NAME.mp4

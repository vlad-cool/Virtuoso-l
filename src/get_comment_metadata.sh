#!/bin/sh
exiftool $1 | grep "Comment"

import subprocess
import platform
import time
import sys
import os

clip_duration = 10 #seconds
post_record = 2 #seconds
segment_duration = 30 #seconds

os.environ["LOG"] = "/dev/null"

if platform.machine() == "armv7l":
    os.environ["OUT_DIR"] = os.environ["HOME"] + "/Videos/V24m"
    os.environ["TMP_DIR"] = os.environ["HOME"] + "/Videos/V24m/tmp"
    os.environ["ENCODER"] = "cedrus264"
else:
    output_dir = "$."
    os.environ["TMP_DIR"] = "./tmp"
    os.environ["ENCODER"] = "libx264"

recording = False
ffmpeg_proc = None
name = 0

start_time = 0

clips = []

def format_time(t):
    if t < 0:
        return "00:00:00"
    seconds = t % 60
    t //= 60
    t = int(t)
    minutes = t % 60
    hours = t // 60
    return f"{hours}:{minutes}:{seconds}"

def start_recording():
    global recording
    global ffmpeg_proc
    global start_time
    if recording:
        return
    try:
        start_time = time.clock_gettime(time.CLOCK_BOOTTIME)
        os.environ["VIDEO_NAME"] = str(name)
        ffmpeg_proc = subprocess.Popen(["./start_ffmpeg.sh"], bufsize=0, text=True, stdin=subprocess.PIPE)
        recording = True
    except:
        exc_type, exc_value = sys.exc_info()
        with open("errors.txt", "a") as f:
            f.write(f"Произошла ошибка: {exc_type} - {exc_value}\n")


def stop_recording():
    global recording
    global ffmpeg_proc
    global start_time
    global name
    if not recording:
        return
    try:
        ffmpeg_proc.stdin.write("q\n")
        recording = False

        split_video()
        start_time = 0
        name += 1
    except:
        exc_type, exc_value = sys.exc_info()
        with open("errors.txt", "a") as f:
            f.write(f"Произошла ошибка: {exc_type} - {exc_value}\n")

def save_clip():
    global recording
    if recording:
        clips.append(time.clock_gettime(time.CLOCK_BOOTTIME))

def split_video():
    split_proc = subprocess.Popen(["./video_cutter.sh"], bufsize=0, text=True, stdin=subprocess.PIPE)
    split_proc.stdin.write(f"{name}\n")
    split_proc.stdin.write(f"{len(clips)}\n")

    for clip in clips:
        split_proc.stdin.write(f"{format_time(clip + post_record - clip_duration - start_time)}\n")
        split_proc.stdin.write(f"{format_time(clip + post_record - start_time)}\n")

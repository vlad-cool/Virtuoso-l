import subprocess
import time
import os

clip_duration = 10  # seconds
post_record = 2  # seconds

enabled = False

recording = False
ffmpeg_proc = None
cutter_proc = None
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
    global enabled
    if not enabled:
        return
    global recording
    global ffmpeg_proc
    global start_time
    if recording:
        return
    start_time = time.clock_gettime(time.CLOCK_BOOTTIME)
    os.environ["VIDEO_NAME"] = str(name)
    ffmpeg_proc = subprocess.Popen(
        ["./start_ffmpeg.sh"], bufsize=0, text=True, stdin=subprocess.PIPE
    )
    recording = True

def stop_recording():
    global recording
    global ffmpeg_proc
    global start_time
    global name
    if not recording or ffmpeg_proc is None:
        return
    ffmpeg_proc.stdin.write("q\n")
    recording = False

    split_video()
    start_time = 0
    name += 1

def save_clip(metadata=""):
    global enabled
    if not enabled:
        return
    global recording
    if recording:
        clips.append((time.clock_gettime(time.CLOCK_BOOTTIME), metadata))

def split_video():
    global enabled
    if not enabled:
        return
    global clips
    global cutter_proc
    cutter_proc = subprocess.Popen(
        ["./video_cutter.sh"], bufsize=0, text=True, stdin=subprocess.PIPE
    )
    cutter_proc.stdin.write(f"{name}\n")
    cutter_proc.stdin.write(f"{len(clips)}\n")

    for clip in clips:
        cutter_proc.stdin.write(f"{format_time(clip[0] + post_record - clip_duration - start_time)}\n")
        cutter_proc.stdin.write(f"{format_time(clip_duration)}\n")
        cutter_proc.stdin.write(f"{clip[1]}\n")
    clips = []

def toggle_recording():
    global enabled
    enabled = not enabled
    return enabled

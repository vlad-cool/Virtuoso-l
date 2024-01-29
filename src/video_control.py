import subprocess
import platform
import time
import os
import system_info

clip_duration = 10  # seconds
post_record = 2  # seconds

os.environ["LOG"] = "/dev/null"

os.environ["OUT_DIR"] = system_info.video_path
os.environ["TMP_DIR"] = system_info.video_path_tmp
os.environ["ENCODER"] = system_info.video_encoder

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


def save_clip():
    global recording
    if recording:
        print(f"clip saved {time.clock_gettime(time.CLOCK_BOOTTIME)}", flush=True)
        clips.append(time.clock_gettime(time.CLOCK_BOOTTIME))


def split_video():
    global clips
    split_proc = subprocess.Popen(
        ["./video_cutter.sh"], bufsize=0, text=True, stdin=subprocess.PIPE
    )
    split_proc.stdin.write(f"{name}\n")
    split_proc.stdin.write(f"{len(clips)}\n")

    for clip in clips:
        split_proc.stdin.write(
            f"{format_time(clip + post_record - clip_duration - start_time)}\n"
        )
        split_proc.stdin.write(f"{format_time(clip + post_record - start_time)}\n")
    clips = []

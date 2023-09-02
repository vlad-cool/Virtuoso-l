import subprocess
import platform
import time
import os

#MAybe while true in start_ffmpeg.sh?

clip_duration = 10 #seconds
post_record = 2 #seconds
segment_duration = 30 #seconds
gap = 1 #seconds, gap between clips

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
    recording = True
    start_time = time.clock_gettime(time.CLOCK_BOOTTIME)
    os.environ["VIDEO_NAME"] = str(name)

    ffmpeg_proc = subprocess.Popen(["./start_ffmpeg.sh"], bufsize=0, text=True, stdin=subprocess.PIPE)

def stop_recording():
    global recording
    global ffmpeg_proc
    global start_time
    global name
    if not recording:
        return
    recording = False

    ffmpeg_proc.stdin.write(f"q")
    #ffmpeg_proc.poll()
    split_video()
    start_time = 0
    name += 1

def save_clip():
    t = time.clock_gettime(time.CLOCK_BOOTTIME)
    if len(clips) > 0 and t - clips[-1] < gap:
        return
    clips.append(time.clock_gettime(time.CLOCK_BOOTTIME))

def split_video():
    split_proc = subprocess.Popen(["./video_cutter.sh"], bufsize=0, text=True, stdin=subprocess.PIPE)
    split_proc.stdin.write(f"{name}\n")
    split_proc.stdin.write(f"{len(clips)}\n")

    print(name)
    print(len(clips))
    
    for clip in clips:
        print(f"{format_time(clip + post_record - clip_duration - start_time)}")
        print(f"{format_time(clip + post_record - start_time)}")
        split_proc.stdin.write(f"{format_time(clip + post_record - clip_duration - start_time)}\n")
        split_proc.stdin.write(f"{format_time(clip + post_record - start_time)}\n")


def main():
    while True:
        cmd = input()
    
        if cmd == "exit":
            break
        if cmd == "start":
            start_recording()
        if cmd == "stop":
            stop_recording()
        if cmd == "save":
            save_clip()
        
        print("done")

def clean():
    pass

if __name__ == "__main__":
    main()
import platform
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
    if recording:
        return
    recording = True

    print("start")

def stop_recording():
    global recording
    if not recording:
        return
    recording = False

    print("stop")

def save_clip():
    print("save")

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

def split_video():
    pass

if __name__ == "__main__":
    main()
import subprocess
import time
import sys
import os

clip_duration = 10  # seconds
post_record = 4  # seconds

enabled = False

recording = False
recorder_proc = None
name = 0

clips = []

def start_recording():
    global enabled
    global recording
    if not enabled or recording:
        return
    global recorder_proc
    os.environ["VIDEO_NAME"] = str(name)
    
    try:    
        recorder_proc = subprocess.Popen(
            ["./recorder.sh"], bufsize=0, text=True, 
            stdin=subprocess.PIPE,
            stdout=open(os.environ["RECORDER_LOG_OUT"], "w"),
            stderr=open(os.environ["RECORDER_LOG_ERR"], "w")
        )
        recording = True
    except Exception as e:
        print(f"Failed to cut videos, an following exception occured: {e}", file=sys.stderr)

def stop_recording():
    global recording
    global recorder_proc
    global name
    if not recording:
        return
    if recorder_proc is None:
        recording = False
        return
    try:
        recorder_proc.stdin.write("q")
        recorder_proc.stdin.flush()
        split_video()
        name += 1
    except Exception as e:
        print(f"Failed to cut videos, an following exception occured: {e}", file=sys.stderr)
    finally:
        pass
        recording = False
        recorder_proc = None

def save_clip(metadata=""):
    global enabled
    global recording
    global clips
    if not enabled or not recording:
        return
    clips.append((time.clock_gettime(time.CLOCK_BOOTTIME), metadata))

def split_video():
    global enabled
    global clips
    global recorder_proc
    if not enabled:
        return

    recorder_proc.stdin.write(f"{len(clips)}\n")

    try:
        for clip in clips:
            recorder_proc.stdin.write(f"{(clip[0] + post_record - clip_duration)}\n")
            recorder_proc.stdin.write(f"{(clip_duration)}\n")
            recorder_proc.stdin.write(f"{clip[1]}\n")
    except Exception as e:
        print(f"Failed to cut videos, an following exception occured: {e}", file=sys.stderr)
    finally:
        clips = []

def toggle_recording():
    global enabled
    enabled = not enabled
    return enabled

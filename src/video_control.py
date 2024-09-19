import subprocess
import time
import sys
import os
import system_info
import base64
import bz2

clip_duration = 10  # seconds
post_record = 2 # seconds

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

def stop_recording(metadata):
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
        recorder_proc.stdin.write(f"{time.clock_gettime(time.CLOCK_BOOTTIME)}\n")
        split_video(metadata)
        name += 1
    except Exception as e:
        print(f"Failed to cut videos, an following exception occured: {e}", file=sys.stderr)
    finally:
        recording = False
        recorder_proc = None
        metadata.clear()

def save_clip():
    global enabled
    global recording
    global clips
    if not enabled or not recording:
        return
    clips.append(time.clock_gettime(time.CLOCK_BOOTTIME) - clip_duration + post_record)

def split_video(metadata):
    global enabled
    global clips
    global recorder_proc
    if not enabled:
        return

    recorder_proc.stdin.write(f"{len(clips)}\n")

    try:
        for clip in clips:
            recorder_proc.stdin.write(f"{(clip)}\n")
            recorder_proc.stdin.write(f"{(clip_duration)}\n")
            metadata_str = f"{clip}"
            for key, value in metadata.items():
                if key > clip and key < clip + clip_duration:
                    metadata_str += str(value)
            
            match system_info.comress_metadata.lower():
                case "bz2":
                    metadata_str = base64.b64encode(bz2.compress(metadata_str.encode())).decode()
            recorder_proc.stdin.write(f"{system_info.comress_metadata.lower()}#{metadata_str}" + "\n")
    except Exception as e:
        print(f"Failed to cut videos, an following exception occured: {e}", file=sys.stderr)
    finally:
        clips = []

def toggle_recording():
    global enabled
    enabled = not enabled
    return enabled

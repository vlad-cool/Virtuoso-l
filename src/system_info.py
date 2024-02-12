import os

def bool(s):
    return str.lower(s) == "true"

# General
is_banana = bool(os.environ["is_banana"])
input_support = bool(os.environ["input_support"])
config_file = os.environ["config_file"]
kivy_file = os.environ["kivy_file"]

# Video
video_support = bool(os.environ["video_support"])
if video_support:
    video_path = os.environ["video_path"]
    video_path_tmp = os.environ["video_path_tmp"]
    video_encoder = os.environ["video_encoder"]

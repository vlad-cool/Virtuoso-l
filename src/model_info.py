os.environ["LOG"] = "/dev/null"

import os

# General

is_banana = bool(os.environ["is_banana"])
input_support = bool(os.environ["is_banana"])
config_file = os.environ["config_file"]
kivy_file = os.environ["kivy_file"]

# Video

video_support = bool(os.environ["is_banana"])
if video_support:
    video_path = os.environ["video_path"]
    video_path_tmp = os.environ["video_path_tmp"]
    video_encoder = os.environ["video_encoder"]

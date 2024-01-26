import os

# General

is_banana = True
input_support = True
config_file = "config.json"
kivy_file = "main1920x480.kv"

# Video

video_support = True
video_path = os.path.join(os.environ["HOME"], "/Videos/V24m")
video_path_tmp = os.path.join(os.environ["HOME"], "/Videos/V24m/tmp")
video_encoder = "cedrus264"

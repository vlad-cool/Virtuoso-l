import os

# General

is_banana = True
input_support = False
config_file = "/mnt/V24m/config.json"
kivy_file = "main1920x360.kv"

# Video

video_support = False
video_path = os.path.join(os.environ["HOME"], "/Videos/V24m")
video_path_tmp = os.path.join(os.environ["HOME"], "/Videos/V24m/tmp")
video_encoder = "cedrus264"

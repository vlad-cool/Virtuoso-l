import os

def bool(s):
    return str.lower(s) == "true"

# General
is_banana = bool(os.environ["IS_BANANA"])
input_support = bool(os.environ["INPUT_SUPPORT"])
config_file = os.environ["CONFIG_FILE"]
kivy_file = os.environ["KIVY_FILE"]

# Video
video_support = bool(os.environ["VIDEO_SUPPORT"])
if video_support:
    video_path = os.environ["VIDEO_PATH"]
    video_path_tmp = os.environ["VIDEO_PATH_TMP"]
    video_encoder = os.environ["VIDEO_ENCODER"]
    camera_path = os.environ["CAMERA_PATH"]
    comress_metadata = os.environ["COMPRESS_METADATA"]

# Updates
update_dir = os.environ["UPDATE_DIR"]
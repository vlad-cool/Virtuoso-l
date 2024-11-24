#!venv/bin/python3
import os
import kivy
import time
import json
import glob
import ifcfg
import serial
import shutil
import pathlib
import subprocess
import system_info
from collections import OrderedDict
from kivy.app import App
from kivy.clock import Clock
from kivy.lang import Builder
from kivy.core.text import LabelBase
from kivy.network.urlrequest import UrlRequest
from kivy.uix.widget import Widget

import bz2
import base64

read_interval = .05

if system_info.video_support:
    import video_control

if system_info.is_banana:
    import gpio_control
else:
    import gpio_control_emu as gpio_control

kivy.require("2.1.0")

class IrKeys:
    CHANGE_TIME = 7
    TOGGLE_RECORDING = 24
    PLAY_PAUSE = 24
    PREVIOUS_VIDEO = 20
    NEXT_VIDEO = 21
    REWIND = 23
    FAST_FORWARD = 22
    CHANGE_MODE = 19
    AUTO_SCORE = 16
    AUTO_TIMER = 1
    LEFT_PASSIVE = 17
    RIGHT_PASSIVE = 18
    UPDATE_BTN = 5
    SWAP_SIDES = 0

class Updater:
    def __init__(self):
        self.download_proc = None

    def update_sync_btn_text(self, btn, text):
        btn.text = text

    def check_version(self, btn, req, result):
        if btn.update_state != "waiting":
            return

        version_path = pathlib.Path("VERSION")
        old_version = ""
        if version_path.is_file():
            with open("VERSION", "r") as version_file:
                old_version = version_file.readline()
        else:
            if version_path.is_dir():
                shutil.rmtree(version_path)
            old_version = "v0.0.0"


        print(req, result, flush=True)

        new_version = result["tag_name"]
        if old_version[0] == 'v':
            old_version = old_version[1:]
        if new_version[0] == 'v':
            new_version = new_version[1:]

        old_version = old_version.replace("\n", "")
        new_version = new_version.replace("\n", "")

        old_version_lst = list(map(int, old_version.split(".")))
        new_version_lst = list(map(int, new_version.split(".")))

        old_major = old_version_lst[0]
        old_minor = old_version_lst[1]
        old_patch = old_version_lst[2]

        new_major = new_version_lst[0]
        new_minor = new_version_lst[1]
        new_patch = new_version_lst[2]

        if (old_major < new_major) or (old_major == new_major and old_minor < new_minor) or (old_major == new_major and old_minor == new_minor and old_patch < new_patch):
            for res in result["assets"]:
                if res["name"] == "Virtuoso_update.zip":
                    self.update_url = result["assets"][0]["browser_download_url"]
                    btn.text = f"New version found\n{old_version}->{new_version}"
                    btn.update_state = "wait_for_update"            
                    if not system_info.input_support:
                        Clock.schedule_once(lambda _ : self.update(self.btn), 2)
                    return
        btn.text = "No update candidate"
        btn.update_state = "no_update"
        if not system_info.input_support:
            Clock.schedule_once(lambda _: app.carousel("main"), 2)

    def update_failed(self, btn, req, result):
        btn.text = "Couldn't get\nversion information"
        Clock.schedule_once(lambda _: self.update_sync_btn_text(btn, "Check for updates"), 10)
        btn.update_state = "no_update"
        if not system_info.input_support:
            Clock.schedule_once(lambda _: app.carousel("main"), 2)

    def download_failed(self, btn, req, result):
        btn.text = "Download failed"
        Clock.schedule_once(lambda _: self.update_sync_btn_text(btn, "Check for updates"), 10)
        btn.update_state = "no_update"
        if not system_info.input_support:
            Clock.schedule_once(lambda _: app.carousel("main"), 2)

    def update_downloaded(self, btn, req, result):
        btn.text = "Update downloaded\nPress to install"
        btn.update_state = "wait_for_reboot"

    def check_download(self):
        if self.download_proc is None:
            return
        if self.download_proc.poll() == 0:
            self.update_downloaded(self.btn, None, None)
            if not system_info.input_support:
                self.update(self.btn)
            self.download_proc = None
            return
        if self.download_proc.poll() is not None:
            self.download_proc = None
            self.download_failed(self.btn, None, None)
    
    def update_redirect_handler(self, btn, req, result):
        req = UrlRequest(
            result["url"],
            req_headers=req.req_headers,
            on_redirect = lambda req, result: self.update_redirect_handler(btn, req, result),
            on_success=lambda req, result: self.check_version(btn, req, result),
            on_failure=lambda req, result: self.update_failed(btn, req, result),
            on_error=lambda req, result: self.update_failed(btn, req, result),
        )

    def update(self, btn):
        self.btn = btn
        repo_owner = "vlad-cool"
        repo_name = "Virtuoso-l"

        url = f"https://api.github.com/repos/{repo_owner}/{repo_name}/releases/latest"

        if btn.update_state == "no_update":
            self.update_request = UrlRequest(url, req_headers={"User-Agent": "Virtuoso-l"}, on_redirect = lambda req, result: self.update_redirect_handler(btn, req, result), on_success=lambda req, result: self.check_version(btn, req, result), on_failure=lambda req, result: self.update_failed(btn, req, result), on_error=lambda req, result: self.update_failed(btn, req, result))
            print(self.update_request)
            btn.text = "Checking for updates..."
            btn.update_state = "waiting"
            return

        if btn.update_state == "wait_for_update":
            btn.text = "Downloading update"
            btn.update_state = "downloading_update"
            self.download_proc = subprocess.run(["rm", f"{system_info.update_dir}/Virtuoso_update.zip"])
            self.download_proc = subprocess.Popen(["wget", "-P", f"{system_info.update_dir}", self.update_url])
            return

        if btn.update_state == "wait_for_reboot":
            if system_info.is_banana:
                subprocess.run("/usr/sbin/reboot")
            return

class UartData:
    def __init__(self, data):
        self.yellow_red    = data[0][4]
        self.red           = data[0][3]
        self.white_green   = data[0][2]
        self.yellow_green  = data[0][1]
        self.green         = data[0][0]
        self.white_red     = data[1][4]
        self.apparel_sound = data[1][3]
        self.symbol        = data[1][2]
        self.on_timer      = data[2][4]
        self.timer_sound   = data[3][4]

        self.score_r = data[7][4]
        for i in data[5][4::-1]:
            self.score_r *= 2
            self.score_r += i

        self.score_l = data[6][4]
        for i in data[4][4::-1]:
            self.score_l *= 2
            self.score_l += i

        self.timer_m = 0
        for i in data[1][2::-1]:
            self.timer_m *= 2
            self.timer_m += i

        self.timer_d = 0
        for i in data[2][3::-1]:
            self.timer_d *= 2
            self.timer_d += i

        self.timer_s = 0
        for i in data[3][3::-1]:
            self.timer_s *= 2
            self.timer_s += i

        self.period = 0
        for i in data[6][3::-1]:
            self.period *= 2
            self.period += i

        self.warning_l = data[7][3] * 2 + data[7][2]
        self.warning_r = data[7][1] * 2 + data[7][0]

class PinsData:
    def __init__(self, pins):
        self.wireless   = pins[7]
        self.recording  = pins[18]
        self.poweroff   = pins[27]
        self.weapon     = pins[32] * 2 + pins[36]
        if not system_info.input_support:
            self.weapon_btn = pins[37]

class PassiveTimer:
    def stop(self):
        self.running = False

    def start(self):
        if self.clear_timer is not None:
            self.clear()
        if not self.running:
            self.prev_time = time.time()
            self.running = True

    def clear(self):
        self.stop()
        self.running     = False
        self.size        = 0
        self.time        = 0
        self.coun        = "60"
        self.prev_time   = 0
        self.clear_timer = None

    def get_time(self):
        return int(self.time)

    def get_size(self):
        return self.size

    def get_coun(self):
        return self.coun

    def update(self, timer_running):
        if self.running:
            cur_time = time.time()
            delta = cur_time - self.prev_time
            self.size += 1 * delta / 50
            self.size = min(self.size, 1)
            self.time += delta
            self.prev_time = cur_time

        if self.time < 60.0:
            if self.running:
                self.coun = str(60 - int(self.time - 0.0001))
                if len(self.coun) == 1:
                    self.coun = " " + self.coun[0]
        else:
            self.coun = " 0"
            if not timer_running:
                if self.clear_timer is None:
                    self.clear_timer = time.time()
                elif time.time() - self.clear_timer > 4:
                    self.clear()

    def __init__(self):
        self.clear()

class SwitchController:
    def __init__(self, n, timeout=1):
        self.switches_states = [False] * n
        self.switch_number = None
        self.new_state = None
        self.start_time = None
        self.timeout = timeout
        self.last_switch = None

    def switch_changed(self, switch_number):
        self.switch_number = switch_number
        self.update_state()

    def switch_state(self, new_state):
        self.new_state = new_state
        self.update_state()

    def update_state(self):
        if self.new_state is None or self.switch_number is None:
            self.start_time = time.clock_gettime(time.CLOCK_BOOTTIME)
        elif time.clock_gettime(time.CLOCK_BOOTTIME) - self.start_time > self.timeout:
            self.switch_number = None
            self.new_state = None
            self.start_time = None
        else:
            self.switches_states[self.switch_number] = self.new_state
            self.last_switch = self.switch_number
            self.switch_number = None
            self.new_state = None
            self.start_time = None

class VideoPlayer:
    def __init__(self, player, kivy_root):
        self.player = player
        self.root = kivy_root
        self.available_videos = OrderedDict()
        self.recording = False
        self.player.state = "play"
        self.video_id = -1
        self.show_preview()
        self.current_metadata = None
        self.video_start_time = 0

    def show_preview(self):
        if self.recording:
            self.player.state = "stop"
            self.player.source = ""
        else:
            self.player.state = "play"
            self.player.source = system_info.camera_path
        self.video_id = -1
        self.root.video_id = -1

    def play_pause(self):
        print(self.player.eos)
        if self.player.eos:
            self.player.seek(0)
            self.start_playback()
        elif self.player.state == "play":
            self.pause_playback()
        else:
            self.start_playback()
    
    def start_playback(self):
        if self.video_id != -1:
            self.player.state = "play"
    
    def pause_playback(self):
        if self.video_id != -1:
            self.player.state = "pause"
    
    def stop_playback(self):
        self.show_preview()

    def play_video(self, id):        
        id = (id + 1) % (len(self.available_videos) + 1) - 1
        self.root.video_info = False
        
        if id == -1:
            self.show_preview()
        else:
            self.player.camera = False
            self.player.unload()
            while self.player.loaded:
                pass
            self.player.source = list(self.available_videos)[id]
            self.video_id = id
            self.root.video_id = id
            self.start_playback()
            self.load_metadata()

    def fix_camera(self):
        self.player.camera = False
        self.player.unload()
        while self.player.loaded:
            pass
        self.player.source = ""
        self.video_id = -2
        self.root.video_id = -2

    def play_next_video(self):
        self.play_video(self.video_id + 1)

    def play_previous_video(self):
        self.play_video(self.video_id - 1)

    def rewind_video(self, s):
        if self.video_id != -1 and self.root.ids.video_player.loaded:
            self.root.ids.video_player.seek((self.root.ids.video_player.position + s) / self.root.ids.video_player.duration, True)

    def load_videos(self):
        # print("Loading videos")
        print(self.available_videos, flush=True)
        if system_info.video_support:
            videos = glob.glob(os.environ["VIDEO_PATH"] + "/*.mp4")
            for video in videos:
                if video not in self.available_videos:
                    self.available_videos[video] = subprocess.Popen(["./get_comment_metadata.sh", video], stdout=subprocess.PIPE)
                elif isinstance(self.available_videos[video], subprocess.Popen) and self.available_videos[video].poll() == 0:
                    self.available_videos[video] = self.available_videos[video].stdout.readline().decode()
            for video in self.available_videos:
                if video not in videos:
                    self.available_videos.pop(video)
            if self.video_id == -1:
                self.show_preview()

    def load_metadata(self):
        path = self.player.source
        print("#" * 50)
        print(path, flush=True)
        print("#" * 50)
        if path == system_info.camera_path:
            return
        if isinstance(self.available_videos[path], subprocess.Popen) and self.available_videos[path].poll() == 0:
                self.available_videos[path] = self.available_videos[path].stdout.readline().decode()
        else:
            self.available_videos[path] = subprocess.run(["./get_comment_metadata.sh", path], capture_output=True).stdout.decode()

        result = self.available_videos[path]

        if result == "":
            self.root.video_info = False
            self.current_metadata = None
            return
        try:
            result = result.replace("Comment", "")
            result = result.replace(" ", "")
            result = result[1:]
            
            splitted = result.split("#")
            
            if (len(splitted)) > 1:
                compress = splitted[0]
                result = splitted[1]
            
                match compress:
                    case "bz2":            
                        result = bz2.decompress(base64.b64decode(result.encode())).decode()
            
            self.video_start_time = float(result.split("v1")[0])
            self.current_metadata = list(map(VideoMetadata, result.split("v1")[1:]))

        except (ValueError, KeyError) as e:
            print(f"An error occurred: {e}")
            self.root.video_info = False
            self.current_metadata = None
    
    def recording_started(self):
        if self.video_id == -1:
            self.player.unload()
            while self.player.loaded:
                pass
            self.player.source = ""
        self.recording = True

    def recording_stopped(self):
        if self.video_id == -1:
            self.player.unload()
            while self.player.loaded:
                pass
            self.player.source = system_info.camera_path
        self.recording = False

    def update_metadata(self):
        if self.player.state == "play" and self.video_id != -1 and self.current_metadata is not None:
            for i in range(len(self.current_metadata) - 1):
                if float(self.current_metadata[i + 1].boottime) > self.video_start_time + 10 - self.root.ids.video_player.duration + self.root.ids.video_player.position:
                    print(f"Processing {i} metadata")
                    self.root.video_info_score_l_l = self.current_metadata[i].score_l_l
                    self.root.video_info_score_l_r = self.current_metadata[i].score_l_r
                    self.root.video_info_score_r_l = self.current_metadata[i].score_r_l
                    self.root.video_info_score_r_r = self.current_metadata[i].score_r_r
                    self.root.video_info_timer_0 = self.current_metadata[i].timer_0
                    self.root.video_info_timer_2 = self.current_metadata[i].timer_2
                    self.root.video_info_timer_3 = self.current_metadata[i].timer_3
                    self.root.video_info_period = self.current_metadata[i].period
                    self.root.video_info_priority = self.current_metadata[i].priority
                    self.root.video_info_warning_l = self.current_metadata[i].warning_l
                    self.root.video_info_warning_r = self.current_metadata[i].warning_r
                    self.root.video_info_passive_size = self.current_metadata[i].passive_size
                    self.root.video_info_passive_coun = self.current_metadata[i].passive_coun + " "
                    self.root.video_info_passive_1_state = self.current_metadata[i].passive_1_state
                    self.root.video_info_passive_2_state = self.current_metadata[i].passive_2_state
                    self.root.video_info_passive_3_state = self.current_metadata[i].passive_3_state
                    self.root.video_info_passive_4_state = self.current_metadata[i].passive_4_state
                    self.root.video_info_epee5 = self.current_metadata[i].epee5
                    self.root.video_info_weapon = self.current_metadata[i].weapon
                    self.root.video_info_color_passive = self.current_metadata[i].color_passive
                    self.root.video_info_color_timer = self.current_metadata[i].color_timer
                    self.root.video_info_led_red_state = self.current_metadata[i].led_red_state
                    self.root.video_info_led_red_white_state = self.current_metadata[i].led_red_white_state
                    self.root.video_info_led_green_state = self.current_metadata[i].led_green_state
                    self.root.video_info_led_green_white_state = self.current_metadata[i].led_green_white_state
                    self.root.video_info = True
                    return

class VideoMetadata:
    def __init__(self, src):
        if isinstance(src, Widget):
            self.version = "v1"
            self.boottime = time.clock_gettime(time.CLOCK_BOOTTIME)
            self.score_l_l = src.score_l_l
            self.score_l_r = src.score_l_r
            self.score_r_l = src.score_r_l
            self.score_r_r = src.score_r_r
            self.timer_0 = src.timer_0
            self.timer_2 = src.timer_2
            self.timer_3 = src.timer_3
            self.period = src.period
            self.priority = src.priority
            self.warning_l = src.warning_l
            self.warning_r = src.warning_r
            self.passive_size = src.passive_size
            self.passive_coun = src.passive_coun
            self.passive_1_state = src.passive_1_state
            self.passive_2_state = src.passive_2_state
            self.passive_3_state = src.passive_3_state
            self.passive_4_state = src.passive_4_state
            self.epee5 = src.epee5
            self.weapon = src.weapon
            self.color_passive = src.color_passive
            self.color_timer = src.color_timer
            self.led_red_state = src.led_red_state
            self.led_red_white_state = src.led_red_white_state
            self.led_green_state = src.led_green_state
            self.led_green_white_state = src.led_green_white_state
            # self.color_timer = 
        elif isinstance(src, str):
            # match src[0]:
            #     case "v1":
            src = src.split(",")
            # self.version = src[0]
            self.boottime = src[1]
            self.score_l_l = src[2]
            self.score_l_r = src[3]
            self.score_r_l = src[4]
            self.score_r_r = src[5]
            self.timer_0 = src[6]
            self.timer_2 = src[7]
            self.timer_3 = src[8]
            self.period = src[9]
            self.priority = src[10]
            self.warning_l = src[11]
            self.warning_r = src[12]
            self.passive_size = src[13]
            self.passive_coun = src[14]
            self.passive_1_state = src[15]
            self.passive_2_state = src[16]
            self.passive_3_state = src[17]
            self.passive_4_state = src[18]
            self.epee5 = src[19]
            self.weapon = src[20]
            self.color_passive = [float(src[21].replace("[", "")), float(src[22]), float(src[23]), float(src[24].replace("]", ""))]
            self.color_timer = [float(src[25].replace("[", "")), float(src[26]), float(src[27]), float(src[28].replace("]", ""))]
            
            try:
                self.led_red_state = src[29]
                self.led_red_white_state = src[30]
                self.led_green_state = src[31]
                self.led_green_white_state = src[32]
            except:
                self.led_red_state = 0
                self.led_red_white_state = 0
                self.led_green_state = 0
                self.led_green_white_state = 0
            
    def to_str(self):
        return f"{self.version},{self.boottime},{self.score_l_l},{self.score_l_r},{self.score_r_l},{self.score_r_r},{self.timer_0},{self.timer_2},{self.timer_3},{self.period},{self.priority},{self.warning_l},{self.warning_r},{self.passive_size},{self.passive_coun},{self.passive_1_state},{self.passive_2_state},{self.passive_3_state},{self.passive_4_state},{self.epee5},{self.weapon},{self.color_passive},{self.color_timer},{self.led_red_state},{self.led_red_white_state},{self.led_green_state},{self.led_green_white_state}"

class KivyApp(App):
    def load_videos(self, _):
        if system_info.video_support and video_control.recorder_proc is None or video_control.recorder_proc.poll() is not None:
            self.video_player.load_videos()

    def carousel(self, name):
        match name:
            case "main":
                self.root.index = 0
        
    def toggle_recording(self):
        if system_info.video_support and not self.root.timer_running:
            self.root.recording_enabled = video_control.toggle_recording()

    def on_position_change(self, player, pos):
        if system_info.video_support:
            if (self.old_pos > 2 and player.duration - pos <= 2):
                self.video_player.pause_playback()
                Clock.schedule_once(lambda _: self.video_player.start_playback(), 1)
            self.old_pos = player.duration - pos

    def sync_new_remote(self, btn):
        if system_info.input_support:
            if btn.sync_state == "no_sync":
                btn.sync_state = "waiting"
                btn.text = "press and hold\n3:00/1:00 button on remote"
                self.read_timer.cancel()
                self.ir_timer = Clock.schedule_interval(lambda _: self.wait_rc5(btn), read_interval)

    def update_btn_text(self, btn, text):
        btn.text = text

    def end_sync_remote(self, btn):
        gpio_control.button_emu(37, 1)
        btn.sync_state = "no_sync"
        Clock.schedule_once(lambda _: self.update_btn_text(btn, f"Syncing ended, address is {self.config['rc5_address']}"), 0.5)
        Clock.schedule_once(lambda _: self.update_btn_text(btn, "Sync new remote"), 10.5)
        self.ir_timer.cancel()
        self.read_timer = Clock.schedule_interval(self.get_data, read_interval)

    def wait_rc5(self, btn):
        cmds = gpio_control.read_rc5()
        for cmd in cmds:
            if cmd[1] == IrKeys.CHANGE_TIME:
                self.config["rc5_address"] = cmd[0]
                self.update_config()
                Clock.schedule_once(lambda _: self.end_sync_remote(btn), 1.5)
                break

    def system_poweroff(_):
        if system_info.is_banana:
            subprocess.run("/usr/sbin/poweroff")

    def system_reboot(_):
        if system_info.is_banana:
            subprocess.run("/usr/sbin/reboot")

    def update_config(self):
        with open(system_info.config_file, "w") as config_file:
            json.dump(self.config, config_file)

    def send_handler(self, code):
        gpio_control.ir_emu(self.config["rc5_address"], code)

    def carousel_handler(self, _, old_index, new_index, commands):
        self.send_handler(commands[(2 + new_index - old_index) % 3])

    def set_weapon(self, new_weapon, epee5=True):
        if system_info.input_support:
            gpio_control.button_emu(37, (3 + new_weapon - self.root.weapon) % 3)
            self.weapon = new_weapon

            if epee5:
                if self.root.weapon == 3 and new_weapon == 0:
                    self.root.epee5 = 1 - self.root.epee5
                    gpio_control.set(15, self.root.epee5)
                else:
                    self.root.epee5 = 0
                    gpio_control.set(15, self.root.epee5)

    def change_weapon_connection_type(_):
        if system_info.input_support:
            gpio_control.button_emu(27, 1)

    def passive_stop_card(self, state, btn_id):
        if self.root.timer_running != 1 and (self.passive_timer.time == 0 or self.passive_timer.time >= 60.0) and state == "down":
            self.passive_timer.clear()
        match btn_id:
            case 1:
                self.root.passive_1_state = state
            case 2:
                self.root.passive_2_state = state
            case 3:
                self.root.passive_3_state = state
            case 4:
                self.root.passive_4_state = state

    def byte_to_arr(self, byte):
        a = [0] * 8
        for i in range(8):
                a[i] = byte % 2
                byte //= 2
        return a

    def update_millis(self, dt):
        if self.root.timer_running == 0:
            return
        self.timer_millis += dt
        t = 99 - int(self.timer_millis * 100) % 100
        app.root.timer_2 = str(t // 10)
        app.root.timer_3 = str(t %  10)

    def data_update(self, data):
        root = self.root
        uart_data = UartData(data)

        if uart_data.red + uart_data.green + uart_data.white_red + uart_data.white_green > 0 and root.timer_running:
            self.passive_timer.clear()

        if uart_data.score_l < 10:
            root.score_l_l = str(uart_data.score_l)
            root.score_l_r = " "
        else:
            root.score_l_l = str(uart_data.score_l // 10)
            root.score_l_r = str(uart_data.score_l % 10)

        if uart_data.score_r < 10:
            root.score_r_l = " "
            root.score_r_r = str(uart_data.score_r)
        else:
            root.score_r_l = str(uart_data.score_r // 10)
            root.score_r_r = str(uart_data.score_r % 10)

        timer_m = uart_data.timer_m
        timer_d = uart_data.timer_d
        timer_s = uart_data.timer_s

        if uart_data.green:
            gpio_control.set(35, 1)
            root.led_red_state = 1
        elif self.led_schedule is None or root.priority != +1:
            gpio_control.set(35, 0)
            root.led_red_state = 0
        
        if uart_data.red:
            gpio_control.set(29, 1)
            root.led_green_state = 1
        elif self.led_schedule is None or root.priority != -1:
            gpio_control.set(29, 0)
            root.led_green_state = 0
            
        if uart_data.white_green:
            gpio_control.set(38, 1)
            root.led_green_white_state = 1
        else:
            gpio_control.set(38, 0)
            root.led_green_white_state = 0
        
        if uart_data.white_red:
            gpio_control.set(31, 1)
            root.led_red_white_state = 1
        else:
            gpio_control.set(31, 0)
            root.led_red_white_state = 0

        def turn_green_off(_):
            root.led_green_state = 0
            gpio_control.set(29, 0)
            self.led_schedule = None

        def turn_red_off(_):
            root.led_red_state = 0
            gpio_control.set(35, 0)
            self.led_schedule = None

        if uart_data.period == 14:
            if root.priority != -1:
                root.priority = -1 # GREEN
                gpio_control.set(35, 1)
                gpio_control.set(29, 0)
                root.led_green_state = 0
                root.led_red_state = 1
                if self.led_schedule is not None:
                    self.led_schedule.cancel()
                    self.led_schedule = None
                self.led_schedule = Clock.schedule_once(turn_red_off, 2)
        elif uart_data.period == 15:
            if root.priority != +1:
                root.priority = +1 # RED
                gpio_control.set(29, 1)
                gpio_control.set(35, 0)
                root.led_green_state = 1
                root.led_red_state = 0
                if self.led_schedule is not None:
                    self.led_schedule.cancel()
                    self.led_schedule = None
                self.led_schedule = Clock.schedule_once(turn_green_off, 2)
        elif uart_data.period == 13:
            if root.priority != 0:
                root.priority = 0
                gpio_control.set(29, 0)
                gpio_control.set(35, 0)
                root.led_green_state = 0
                root.led_red_state = 0
                if self.led_schedule is not None:
                    self.led_schedule.cancel()
                    self.led_schedule = None
        elif uart_data.period >= 1 and uart_data.period <= 9:
            root.period = uart_data.period
        if uart_data.period in [12, 13] and self.prev_uart_data is not None and self.prev_uart_data.period not in [12, 13]:
            self.passive_timer.clear()
        if uart_data.period == 12 and timer_m == 0:
            timer_m = 4

        self.symbol = uart_data.symbol
        if uart_data.symbol == 0:
            if self.old_sec != str(timer_s):
                root.time_updated = True
                root.flash_timer = time.time()
            else:
                root.time_updated = False

            if uart_data.on_timer == 0:
                root.color_timer = root.color_timer_orange
            elif timer_m == 0 and timer_d == 0:
                root.color_timer = root.color_timer_blue
                if self.timer_interval is None:
                    self.timer_interval = Clock.schedule_interval(self.update_millis, 0.02)
                    self.timer_millis = 0
                    root.timer_1 = ":"
                    root.timer_2 = "9"
                    root.timer_0 = "9"
                root.timer_0 = str(timer_s - 1)

            elif timer_m > 0 or root.color_timer == root.color_timer_orange:
                root.color_timer = root.color_timer_white

            if (timer_m > 0 or timer_d > 0 or (timer_m == 0 and timer_d == 0 and timer_s == 0)) and self.timer_interval is not None:
                self.timer_interval.cancel()
                self.timer_interval = None
            if self.timer_interval is None:
                self.old_sec = str(timer_s)
                root.timer_0 = str(timer_m)
                root.timer_1 = ":"
                root.timer_2 = str(timer_d)
                root.timer_3 = str(timer_s)
                root.timer_text = ""
            root.timer_running = uart_data.on_timer

        else:
            root.timer_0 = ""
            root.timer_1 = ""
            root.timer_2 = ""
            root.timer_3 = ""

            sym = timer_d * 16 + timer_s

            if sym == 17:
                self.auto_status.switch_state(False)
            elif sym == 196:
                self.auto_status.switch_state(True)

        root.warning_l = uart_data.warning_l
        root.warning_r = uart_data.warning_r

        if root.timer_running == 1 and ((timer_m > 0 and timer_m + timer_d + timer_s > 1) or self.passive_timer.prev_time != 0) and not self.root.weapon == 1:
            self.passive_timer.start()
        else:
            self.passive_timer.stop()

        self.prev_uart_data = uart_data

    def get_data(self, _):
        self.auto_status.update_state()
        self.updater.check_download()
        if system_info.video_support:
            self.video_player.update_metadata()

        root = self.root
        root.current_time = time.time()
        if system_info.is_banana:
            data = [[0] * 8] * 8
            while self.data_rx.inWaiting() > 0:
                flag = True
                while flag:
                    recieved_byte = int.from_bytes(self.data_rx.read(), "big")
                    flag = recieved_byte >> 5 != 0
                data[0] = self.byte_to_arr(recieved_byte)
                for i in range(1, 8):
                    if self.data_rx.inWaiting() == 0:
                        break
                    recieved_byte = int.from_bytes(self.data_rx.read(), "big")
                    if recieved_byte >> 5 != i:
                        break
                    data[i] = self.byte_to_arr(recieved_byte)
                else:
                    self.data_update(data)
        else:
            data = [[0, 0, 0, 0, 0, 0, 0, 0][::-1],
                    [0, 0, 1, 0, 0, 0, 0, 0][::-1],
                    [0, 1, 0, 0, 0, 0, 0, 0][::-1],
                    [0, 1, 1, 0, 0, 0, 0, 0][::-1],
                    [1, 0, 0, 1, 1, 1, 1, 1][::-1],
                    [1, 0, 1, 1, 1, 1, 1, 1][::-1],
                    [1, 1, 0, 1, 0, 0, 0, 0][::-1],
                    [1, 1, 1, 1, 0, 1, 1, 0][::-1],]
            self.data_update(data)

        pins_data = PinsData(gpio_control.read_pins())

        if pins_data.poweroff == 0:
            self.system_poweroff()

        # Recording section
        # -----------------
        if system_info.video_support:
            if (video_control.recorder_proc is not None) and (video_control.recorder_proc.poll() is None):
                metadata = VideoMetadata(root)
                self.clip_data_dict[metadata.boottime] = metadata.to_str()
            
            if ((self.prev_pins_data is None or self.prev_pins_data.recording == 0) and pins_data.recording == 1) or (pins_data.recording == 1 and not (video_control.recorder_proc is not None and video_control.recorder_proc.poll() is None)):
                if self.stop_recording_scheduler is not None:
                    self.stop_recording_scheduler.cancel()
                if video_control.recorder_proc is None:
                    self.video_player.recording_started()
                    video_control.start_recording()
            
            elif self.prev_pins_data is not None and self.prev_pins_data.recording == 1 and pins_data.recording == 0:
                video_control.save_clip()
                self.stop_recording_scheduler = Clock.schedule_once(lambda _: (video_control.stop_recording(self.clip_data_dict), self.video_player.recording_stopped()), 4)
        # -----------------

        root.weapon = pins_data.weapon
        if root.weapon == 1:
            self.passive_timer.clear()
        
        root.weapon_connection_type = pins_data.wireless

        self.passive_timer.update(self.root.timer_running)
        root.passive_size = self.passive_timer.get_size()
        root.passive_time = self.passive_timer.get_time()
        root.passive_coun = self.passive_timer.get_coun()
        root.color_passive = root.color_passive_red if root.passive_time > 50 else root.color_passive_yel

        if system_info.video_support:
            root.recording = (video_control.recorder_proc is not None) and (video_control.recorder_proc.poll() is None)

        self.prev_pins_data = pins_data

        cmds = gpio_control.read_rc5()

        if not system_info.input_support:
            if pins_data.weapon_btn == 0:
                for cmd in cmds:
                    if cmd[1] == IrKeys.CHANGE_TIME:
                        self.config["rc5_address"] = cmd[0]
                        self.update_config()
                
        for cmd in cmds:
            print(cmd)
            if cmd[0] != self.config["rc5_address"]:
                continue
            if cmd[2] == False:
                continue
            
            if not system_info.input_support and pins_data.weapon_btn == 0 and cmd[1] == IrKeys.UPDATE_BTN:
                self.root.index = 2 if system_info.video_support else 1
                self.root.ids["settings_update"].state = "down"
                self.updater.update(self.root.ids["update_btn"])
            
            print(f"{root.index}, {cmd[1]}")
            
            match root.index:
                case 0:
                    match cmd[1]:
                        case IrKeys.CHANGE_TIME:
                            if self.root.timer_running != 1:
                                self.passive_timer.clear()
                        case IrKeys.LEFT_PASSIVE:
                            if self.root.timer_running != 1 and (self.passive_timer.time == 0 or self.passive_timer.time >= 60.0):
                                self.passive_timer.clear()
                                if root.passive_2_state == "normal":
                                    root.passive_2_state = "down"
                                elif root.passive_1_state == "normal":
                                    root.passive_1_state = "down"
                                else:
                                    root.passive_2_state = "normal"
                                    root.passive_1_state = "normal"
                        case IrKeys.RIGHT_PASSIVE:
                            if self.root.timer_running != 1 and (self.passive_timer.time == 0 or self.passive_timer.time >= 60.0):
                                self.passive_timer.clear()
                                if root.passive_4_state == "normal":
                                    root.passive_4_state = "down"
                                elif root.passive_3_state == "normal":
                                    root.passive_3_state = "down"
                                else:
                                    root.passive_4_state = "normal"
                                    root.passive_3_state = "normal"
                        case IrKeys.TOGGLE_RECORDING:
                            if system_info.video_support:
                                self.toggle_recording()
                        case IrKeys.CHANGE_MODE:
                            if system_info.video_support:
                                root.index = 1
                case 1:
                    if system_info.video_support:
                        match cmd[1]:
                            case IrKeys.PLAY_PAUSE:
                                self.video_player.play_pause()
                            case IrKeys.PREVIOUS_VIDEO:
                                self.video_player.play_previous_video()
                            case IrKeys.NEXT_VIDEO:
                                self.video_player.play_next_video()
                            case IrKeys.REWIND:
                                self.video_player.rewind_video(-1)
                            case IrKeys.FAST_FORWARD:
                                self.video_player.rewind_video(1)
                            case IrKeys.CHANGE_MODE:
                                root.index = 0
                case 2:
                    pass
            if cmd[1] == IrKeys.SWAP_SIDES:
                root.passive_1_state, root.passive_3_state = root.passive_3_state, root.passive_1_state
                root.passive_2_state, root.passive_4_state = root.passive_4_state, root.passive_2_state
            if cmd[1] == IrKeys.AUTO_SCORE:
                self.auto_status.switch_changed(1)
            if cmd[1] == IrKeys.AUTO_TIMER:
                self.auto_status.switch_changed(0)

        if self.symbol == 1:
            if self.auto_status.switches_states[0]:
                if self.auto_status.last_switch == 0:
                    root.timer_text = "Auto timer on"
                root.auto_timer_status = "Auto timer\non"
            else:
                if self.auto_status.last_switch == 0:
                    root.timer_text = "Auto timer off"
                root.auto_timer_status = "Auto timer\noff"
            if self.auto_status.switches_states[1]:
                if self.auto_status.last_switch == 1:
                    root.timer_text = "Auto score on"
                root.auto_score_status = "Auto score\non"
            else:
                if self.auto_status.last_switch == 1:
                    root.timer_text = "Auto score off"
                root.auto_score_status = "Auto score\noff"

    def update_network_data(self, _):
        for name, interface in ifcfg.interfaces().items():
            if name == "wlan0":
                if interface["inet"] is None:
                    self.root.wireless_ip = "No connection"
                else:
                    self.root.wireless_ip = f"IP address is {interface['inet']}"
            if name == "eth0":
                if interface["inet"] is None:
                    self.root.wired_ip = "No connection"
                else:
                    self.root.wired_ip = f"IP address is {interface['inet']}"

    def build(self):
        self.metadata_procs = {}
        
        self.clip_data_dict = OrderedDict()

        self.updater = Updater()

        self.stop_recording_scheduler = None

        self.auto_status = SwitchController(2)

        self.symbol = 0

        self.old_sec              = "0"
        self.passive_timer        = PassiveTimer()
        self.timer_interval       = None
        self.timer_millis         = 0

        self.led_schedule   = None
        self.prev_uart_data = None
        self.prev_pins_data = None

        self.read_timer = None

        self.old_pos = 0

        self.config = {"rc5_address": -1}

        if system_info.is_banana:
            self.data_rx = serial.Serial("/dev/ttyS2", 38400)
        else:
            self.data_rx = None

        config_path = pathlib.Path(system_info.config_file)
        if config_path.is_file():
            with open(system_info.config_file, "r") as config_file:
                try:
                    self.config = json.load(config_file)
                except (ValueError, KeyError) as e:
                    print(f"An error occurred when loading config file: {e}")
                    self.update_config()
        else:
            if config_path.is_dir():
                shutil.rmtree(config_path)
            self.update_config()

        gpio_control.setup()

        return Builder.load_file(system_info.kivy_file)

    def on_start(self):
        if system_info.video_support:
            self.video_player = VideoPlayer(self.root.ids["video_player"], self.root)
            self.video_player.load_videos()
        
        self.get_data(0)
        self.set_weapon(0, False)
        
        self.read_timer = Clock.schedule_interval(self.get_data, read_interval)
        self.video_loader_timer = Clock.schedule_interval(self.load_videos, 2)
        Clock.schedule_interval(self.update_network_data, 2)
        
        self.root.flash_timer  = time.time()
        self.root.current_time = time.time()
        
        if system_info.video_support:
            self.root.video_path = os.environ["VIDEO_PATH"]
            self.root.ids["video_player"].bind(
                position=self.on_position_change
            )

            Clock.schedule_once(lambda _ : self.video_player.fix_camera(), 1)
            Clock.schedule_once(lambda _ : self.video_player.play_video(1), 2)
            Clock.schedule_once(lambda _ : self.video_player.play_video(-1), 3)

    def on_stop(self):
        if self.data_rx is not None:
            self.data_rx.close()

if __name__ == "__main__":
    LabelBase.register(name="agencyb", fn_regular="assets/AGENCYB.TTF")
    LabelBase.register(name="agencyr", fn_regular="assets/AGENCYR.TTF")
    app = KivyApp()
    app.run()
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

read_interval = .05

if system_info.video_support:
    import video_control

if system_info.is_banana:
    import gpio_control
else:
    import gpio_control_emu as gpio_control

kivy.require("2.1.0")

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

        if (old_major < new_major) or (old_major == new_major and old_minor < new_minor) or (old_major == old_minor and old_minor == new_minor and old_patch < new_patch):
            for res in result["assets"]:
                if res["name"] == "V24m_update.zip":
                    self.update_url = result["assets"][0]["browser_download_url"]
                    btn.text = f"New version found\n{old_version}->{new_version}"
                    btn.update_state = "wait_for_update"
                    return
        btn.text = "No update candidate"
        btn.update_state = "no_update"

    def update_failed(self, btn, req, result):
        btn.text = "Couldn't get version information"
        Clock.schedule_once(lambda _: self.update_sync_btn_text(btn, "Check for updates"), 10)
        btn.update_state = "no_update"

    def download_failed(self, btn, req, result):
        btn.text = "Download failed"
        Clock.schedule_once(lambda _: self.update_sync_btn_text(btn, "Check for updates"), 10)
        btn.update_state = "no_update"

    def update_downloaded(self, btn, req, result):
        btn.text = "Update downloaded\nPress to install"
        btn.update_state = "wait_for_reboot"

    def check_download(self):
        if self.download_proc is None:
            return
        if self.download_proc.poll() == 0:
            self.update_downloaded(self.btn, None, None)
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
        repo_name = "V24m"

        url = f"https://api.github.com/repos/{repo_owner}/{repo_name}/releases/latest"

        if btn.update_state == "no_update":
            self.update_request = UrlRequest(url, req_headers={"User-Agent": "V24m"}, on_redirect = lambda req, result: self.update_redirect_handler(btn, req, result), on_success=lambda req, result: self.check_version(btn, req, result), on_failure=lambda req, result: self.update_failed(btn, req, result), on_error=lambda req, result: self.update_failed(btn, req, result))
            print(self.update_request)
            btn.text = "Checking for updates..."
            btn.update_state = "waiting"
            return

        if btn.update_state == "wait_for_update":
            btn.text = "Downloading update"
            btn.update_state = "downloading_update"
            self.download_proc = subprocess.run(["rm", f"{system_info.update_dir}/V24m_update.zip"])
            self.download_proc = subprocess.Popen(["wget", "-P", f"{system_info.update_dir}/V24m", self.update_url])
            return

        if btn.update_state == "wait_for_reboot":
            if system_info.is_banana:
                subprocess.run("/usr/sbin/reboot")
            return

class UartData:
    def __init__(self, data):
        self.yellow_white  = data[0][4]
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
        if not self.running:
            self.prev_time = time.time()
            self.running = True

    def clear(self):
        self.stop()
        self.running   = False
        self.size      = 0
        self.time      = 0
        self.coun      = "60"
        self.prev_time = 0

    def get_time(self):
        return int(self.time)

    def get_size(self):
        return self.size

    def get_coun(self):
        return self.coun

    def update(self):
        if not self.running:
            return

        cur_time = time.time()
        delta = cur_time - self.prev_time
        self.size += 1 * delta / 50
        self.size = min(self.size, 1)
        self.time += delta
        if self.time < 60.0:
            self.coun = str(60 - int(self.time) - 0.0001)
            if len(self.coun) == 1:
                self.coun = " " + self.coun
        else:
            self.coun = " 0"
        self.prev_time = cur_time

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

    def show_preview(self):
        if not self.recording:
            self.player.state = "play"
            self.video_id = -1
        self.player.unload()
        while self.player.loaded:
            pass
        self.player.source = system_info.camera_path if not self.recording else ""

    def play_pause(self):
        print(self.player.eos)
        if self.player.eos:
            self.player.seek(0)
            self.start_playback()
        elif self.player.state == "pause":
            self.start_playback()
        else:
            self.pause_playback()
    
    def start_playback(self):
        if self.video_id != -1:
            self.player.state = "play"
    
    def pause_playback(self):
        if self.video_id != -1:
            self.player.state = "pause"
    
    def stop_playback(self):
        self.video_id = -1
        self.root.video_id = -1
        self.show_preview()

    def play_video(self, id):
        if len(self.available_videos) == 0:
            return
        id = id % len(self.available_videos)
        self.player.camera = False
        self.video_id = id
        self.player.unload()
        while self.player.loaded:
            pass
        self.player.source = list(self.available_videos)[id]
        self.start_playback()
        self.load_metadata()
        self.root.video_id = id

    def play_next_video(self):
        self.play_video(self.video_id + 1)

    def play_previous_video(self):
        self.play_video(self.video_id - 1)

    def rewind_video(self, s):
        if self.video_id != -1 and self.root.ids.video_player.loaded:
            self.root.ids.video_player.seek((self.root.ids.video_player.position + s) / self.root.ids.video_player.duration, True)

    def load_videos(self):
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
            if (len(videos) == 0):
                return

    def load_metadata(self):
        path = self.player.source
        if path == system_info.camera_path:
            return
        if path not in self.available_videos:
            self.available_videos[path] = subprocess.run(["./get_comment_metadata.sh", path], capture_output=True).stdout.decode()
        elif isinstance(self.available_videos[path], subprocess.Popen):
            self.available_videos[path] = self.available_videos[path].stdout.readline().decode()

        result = self.available_videos[path]

        if result == "":
            self.root.video_info = False
            return
        try:
            result = result.replace("Comment", "")
            result = result.replace(" ", "")
            result = result[1:]
            data = result.split(";")

            clip_data = {}

            for s in data:
                if s == "\n" or s == "":
                    continue
                a, b = s.split(":")
                clip_data[a] = b

            self.root.video_info_score_l_l = clip_data["score_l_l"]
            self.root.video_info_score_l_r = clip_data["score_l_r"]
            self.root.video_info_score_r_l = clip_data["score_r_l"]
            self.root.video_info_score_r_r = clip_data["score_r_r"]
            self.root.video_info_timer_0 = clip_data["timer_0"]
            self.root.video_info_timer_2 = clip_data["timer_2"]
            self.root.video_info_timer_3 = clip_data["timer_3"]
            self.root.video_info_period = clip_data["period"]
            self.root.video_info_priority = clip_data["priority"]
            self.root.video_info_warning_l = clip_data["warning_l"]
            self.root.video_info_warning_r = clip_data["warning_r"]
            self.root.video_info_passive_size = clip_data["passive_size"]
            self.root.video_info_passive_coun = clip_data["passive_coun"]
            self.root.video_info_passive_1_state = clip_data["passive_1_state"]
            self.root.video_info_passive_2_state = clip_data["passive_2_state"]
            self.root.video_info_passive_3_state = clip_data["passive_3_state"]
            self.root.video_info_passive_4_state = clip_data["passive_4_state"]
            self.root.video_info_epee5 = clip_data["epee5"]
            self.root.video_info_weapon = clip_data["weapon"]
            self.root.video_info_color_passive = list(map(float, (clip_data["color_passive"].replace("[", "").replace("]", "").split(","))))

            self.root.video_info = True
        except (ValueError, KeyError) as e:
            print(f"An error occurred: {e}")
            self.root.video_info = False
    
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

class KivyApp(App):
    def toggle_recording(self):
        if not self.root.timer_running:
            self.root.recording_enabled = video_control.toggle_recording()

    def on_position_change(self, player, pos):
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
        cmds = gpio_control.read_all_rc5()
        for cmd in cmds:
            if cmd[1] == 7:
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
        if self.root.timer_running != 1 and state == "down":
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

        if uart_data.period == 15:
            if root.priority != 1:
                root.priority = 1 # GREEN
                gpio_control.set(29, 0)
                gpio_control.set(35, 1)
                if self.led_schedule is not None:
                    self.led_schedule.cancel()
                    self.led_schedule = None
                self.led_schedule = Clock.schedule_once(lambda dt: gpio_control.set(35, 0), 2)
        elif uart_data.period == 14:
            if root.priority != -1:
                root.priority = -1 # RED
                gpio_control.set(35, 0)
                gpio_control.set(29, 1)
                if self.led_schedule is not None:
                    self.led_schedule.cancel()
                    self.led_schedule = None
                self.led_schedule = Clock.schedule_once(lambda dt: gpio_control.set(29, 0), 2)
        elif uart_data.period == 13:
            if root.priority != 0:
                root.priority = 0
                gpio_control.set(35, 0)
                gpio_control.set(29, 0)
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

        root = self.root
        root.current_time = time.time()
        if system_info.is_banana:
            data = [[0] * 8] * 8
            while self.data_rx.inWaiting() // 8 > 0:
                for _ in range(8):
                    byte = int.from_bytes(self.data_rx.read(), "big")
                    data[byte // 2 ** 5] = self.byte_to_arr(byte)

                self.data_update(data)
        else:
            data = [[0, 0, 0, 0, 0, 0, 0, 0][::-1],
                    [0, 0, 1, 0, 0, 0, 0, 0][::-1],
                    [0, 1, 0, 1, 0, 0, 0, 0][::-1],
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
            if ((self.prev_pins_data is None or self.prev_pins_data.recording == 0) and pins_data.recording == 1) or (pins_data.recording == 1 and not (video_control.ffmpeg_proc is not None and video_control.ffmpeg_proc.poll() is None)):
                if self.stop_recording_scheduler is not None:
                    self.stop_recording_scheduler.cancel()
                if video_control.ffmpeg_proc is None or video_control.ffmpeg_proc is None:
                    self.video_player.recording_started()
                    video_control.start_recording()

            elif self.prev_pins_data is not None and self.prev_pins_data.recording == 1 and pins_data.recording == 0:
                clip_data = ""
                clip_data += f"score_l_l:{root.score_l_l};"
                clip_data += f"score_l_r:{root.score_l_r};"
                clip_data += f"score_r_l:{root.score_r_l};"
                clip_data += f"score_r_r:{root.score_r_r};"
                clip_data += f"timer_0:{root.timer_0};"
                clip_data += f"timer_2:{root.timer_2};"
                clip_data += f"timer_3:{root.timer_3};"
                clip_data += f"period:{root.period};"
                clip_data += f"priority:{root.priority};"
                clip_data += f"warning_l:{root.warning_l};"
                clip_data += f"warning_r:{root.warning_r};"
                clip_data += f"passive_size:{root.passive_size};"
                clip_data += f"passive_coun:{root.passive_coun};"
                clip_data += f"passive_1_state:{root.passive_1_state};"
                clip_data += f"passive_2_state:{root.passive_2_state};"
                clip_data += f"passive_3_state:{root.passive_3_state};"
                clip_data += f"passive_4_state:{root.passive_4_state};"
                clip_data += f"epee5:{root.epee5};"
                clip_data += f"weapon:{root.weapon};"
                clip_data += f"color_passive:{root.color_passive};"

                video_control.save_clip(metadata=json.dumps(clip_data).replace(" ", ""))
                self.stop_recording_scheduler = Clock.schedule_once(lambda _: (video_control.stop_recording(), self.video_player.recording_stopped()), 2)
            if video_control.cutter_proc is not None and video_control.cutter_proc.poll() is None:
                self.video_player.load_videos()
        # -----------------

        root.weapon = pins_data.weapon
        if root.weapon == 1:
            self.passive_timer.clear()
        root.weapon_connection_type = pins_data.wireless

        self.passive_timer.update()
        root.passive_size = self.passive_timer.get_size()
        root.passive_time = self.passive_timer.get_time()
        root.passive_coun = self.passive_timer.get_coun()
        root.color_passive = root.color_passive_red if root.passive_time > 50 else root.color_passive_yel

        if system_info.video_support:
            if video_control.ffmpeg_proc is not None and video_control.ffmpeg_proc.poll() is not None:
                video_control.ffmpeg_proc = None
            root.recording = (video_control.ffmpeg_proc is not None) and (video_control.ffmpeg_proc.poll() is None)

        self.prev_pins_data = pins_data

        cmds = []

        if not system_info.input_support:
            if pins_data.weapon_btn == 0:
                cmds = gpio_control.read_all_rc5()
                for cmd in cmds:
                    if cmd[1] == 7:
                        self.config["rc5_address"] = cmd[0]
                        self.update_config()
            else:
                cmds = gpio_control.read_rc5(self.config["rc5_address"])
        else:
            cmds = gpio_control.read_rc5(self.config["rc5_address"])
        for cmd in cmds:
            if cmd[2]:
                if cmd[1] == 7:
                    if self.root.timer_running != 1:
                        self.passive_timer.clear()
                if cmd[1] == 17: # Left passive
                    if self.root.timer_running != 1:
                        self.passive_timer.clear()
                        if root.passive_2_state == "normal":
                            root.passive_2_state = "down"
                        elif root.passive_1_state == "normal":
                            root.passive_1_state = "down"
                        else:
                            root.passive_2_state = "normal"
                            root.passive_1_state = "normal"
                if cmd[1] == 18: # Right passive
                    if self.root.timer_running != 1:
                        self.passive_timer.clear()
                        if root.passive_4_state == "normal":
                            root.passive_4_state = "down"
                        elif root.passive_3_state == "normal":
                            root.passive_3_state = "down"
                        else:
                            root.passive_4_state = "normal"
                            root.passive_3_state = "normal"
                carousel = self.root
                if cmd[1] == 24: # Play pause button | enable disable recording
                    if carousel.index == 0:
                        self.toggle_recording()
                    elif carousel.index == 1:
                        self.video_player.play_pause()
                if carousel.index == 1:
                    if cmd[1] == 20: # Previous video
                        self.video_player.play_previous_video()
                    if cmd[1] == 21: # Next video
                        self.video_player.play_next_video()
                    if cmd[1] == 23: # Rewind back
                        self.video_player.rewind_video(-1)
                    if cmd[1] == 22: # Rewind front
                        self.video_player.rewind_video(1)
                if cmd[1] == 19: # Change mode
                    if carousel.index == 0:
                        carousel.index = 1
                    else:
                        carousel.index = 0
                if cmd[1] == 16:
                    self.auto_status.switch_changed(1)
                if cmd[1] == 1:
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
        self.get_data(0)
        self.video_player = VideoPlayer(self.root.ids["video_player"], self.root)
        self.set_weapon(0, False)
        # Clock.schedule_once(lambda _: self.set_weapon(0, epee5=False), 1)
        self.read_timer = Clock.schedule_interval(self.get_data, read_interval)
        Clock.schedule_interval(self.update_network_data, 2)
        self.root.flash_timer  = time.time()
        self.root.current_time = time.time()
        self.video_player.load_videos()
        self.root.video_path = os.environ["VIDEO_PATH"]
        self.root.ids["video_player"].bind(
            position=self.on_position_change
        )

    def on_stop(self):
        if self.data_rx is not None:
            self.data_rx.close()

if __name__ == "__main__":
    LabelBase.register(name="agencyb", fn_regular="assets/AGENCYB.TTF")
    LabelBase.register(name="agencyr", fn_regular="assets/AGENCYR.TTF")
    app = KivyApp()
    app.run()
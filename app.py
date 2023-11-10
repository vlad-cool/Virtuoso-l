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
import platform
import subprocess
from kivy.clock import Clock
from kivy.lang import Builder
from kivy.app import App
from kivy.core.text import LabelBase
from kivy.uix.button import Button
from kivy.uix.label import Label
from kivy.network.urlrequest import UrlRequest

# for merging #

read_interval = .05
is_banana = platform.machine() == "armv7l"

video_support = False
input_support = False

if is_banana:
    import gpio_control
    if video_support:
        import video_control
    else:
        import video_control_emu as video_control
else:
    import gpio_control_emu as gpio_control
    import video_control_emu as video_control

kivy.require("2.1.0")

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
        self.wireless  = pins[7]
        self.recording = pins[18]
        self.poweroff  = pins[27]
        self.weapon    = pins[32] * 2 + pins[36]

class PassiveTimer:
    def stop(self):
        self.running = False

    def start(self):
        if self.running == False:
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
        return int(self.size)

    def get_coun(self):
        return self.coun

    def update(self):
        if self.running == False:
            return

        cur_time = time.time()
        delta = cur_time - self.prev_time
        self.size += self.max_size * delta / 50
        self.size = min(self.size, self.max_size)
        self.time += delta
        if self.time < 60.0:
            self.coun = str(60 - int(self.time) - 0.0001)
            if len(self.coun) == 1:
                self.coun = " " + self.coun
        else:
            self.coun = " 0"
        self.prev_time = cur_time

    def __init__(self, passive_max_size):
        self.max_size = passive_max_size
        self.clear()

class KivyApp(App):
    Symbols = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"]

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
        
        old_version_lst = list(map(int, old_version[1:].split(".")))

        new_version = result["tag_name"]
        new_version_lst = [1, 7, 9]

        old_major = old_version_lst[0]
        old_minor = old_version_lst[1]
        old_patch = old_version_lst[2]
        
        new_major = new_version_lst[0]
        new_minor = new_version_lst[1]
        new_patch = new_version_lst[2]

        if (old_major < new_major) or (old_major == new_major and old_minor < new_minor) or (old_major == old_minor and old_minor == new_minor and old_patch < new_patch):
            btn.text = f"New version found\n{old_version}->{new_version}"
            btn.update_state = "wait_for_update"
    
    def update_failed(self, btn, req, result):
        print(req, result)
        btn.text = "Couldn't get version information"
        Clock.schedule_once(lambda _: self.update_sync_btn_text(btn, "Check for updates"), 10)
        btn.update_state = "no_update"

    def update(self, btn):
        if btn.update_state == "no_update":
            self.update_request = UrlRequest("https://api.github.com/repos/idorecall/selection-menu/releases/latest", req_headers={"User-Agent": "V24m"}, on_success=lambda req, result: self.check_version(btn, req, result), on_failure=lambda req, result: self.update_failed(btn, req, result), on_error=lambda req, result: self.update_failed(btn, req, result))
            btn.text = "Checking for updates..."
            btn.update_state = "waiting"
        
        if btn.update_state == "wait_for_update":
            self.root.clear_widgets()
            self.root.add_widget(Label(text="Updating, please wait", font_size=360, font_name="agencyb"))

    def play_video(self, vid):
        self.root.ids["video_player"].source = vid
        self.root.ids["video_player"].state = "play"

    def sync_new_remote(self, btn):
        if btn.sync_state == "no_sync":
            btn.sync_state = "waiting"
            btn.text = "press and hold\n3:00/1:00 button on remote"
            self.read_timer.cancel()
            self.ir_timer = Clock.schedule_interval(lambda _: self.wait_rc5(btn), read_interval)

    def update_sync_btn_text(self, btn, text):
        btn.text = text

    def end_sync_remote(self, btn):
        gpio_control.button_emu(37, 1)
        btn.sync_state = "no_sync"
        Clock.schedule_once(lambda _: self.update_sync_btn_text(btn, f"Syncing ended, address is {self.config['rc5_address']}"), 0.5)
        Clock.schedule_once(lambda _: self.update_sync_btn_text(btn, "Sync new remote"), 10.5)
        self.ir_timer.cancel()
        self.read_timer = Clock.schedule_interval(self.get_data, read_interval)

    def wait_rc5(self, btn):
        cmds = gpio_control.read_all_rc5()
        for cmd in cmds:
            if cmd[1] == 7:
                self.config["rc5_address"] = cmd[0]
                self.update_config()
                Clock.schedule_once(lambda _: self.end_sync_remote(btn), 1.5)

    def load_video_list(self):
        if is_banana and input_support:
            videos = glob.glob(os.environ["HOME"] + "/Videos/V24m/*.mp4")
            videos.sort(key=lambda x: int(x[21:-4]))

            while len(self.root.ids["video_list"].children) > 0:
                self.root.ids["video_list"].remove_widget(self.root.ids["video_list"].children[0])

            for video in videos:
                self.root.ids["video_list"].add_widget(Button(text=video, on_press=lambda _, video=video: app.play_video(video), size=(400, 120), size_hint=(None, None)))

            self.root.ids["video_player"].state = "pause"

    def play_pause_video(self):
        if video_support:
            self.root.video_playing = not self.root.video_playing

    def system_poweroff(_):
        if is_banana:
            subprocess.run("/usr/sbin/poweroff")

    def system_reboot(_):
        if is_banana:
            subprocess.run("/usr/sbin/reboot")

    def update_config(self):
        with open("config/config.json", "w") as config_file:
            json.dump(self.config, config_file)

    def send_handler(self, code):
        gpio_control.ir_emu(self.config["rc5_address"], code)

    def carousel_handler(self, _, old_index, new_index, commands):
        self.send_handler(commands[(2 + new_index - old_index) % 3])

    def set_weapon(self, new_weapon):
        gpio_control.button_emu(37, (3 + new_weapon - self.root.weapon) % 3)
        self.weapon = new_weapon

        if self.root.weapon == 3 and new_weapon == 0:
            self.root.epee5 = 1 - self.root.epee5
            gpio_control.set(15, self.root.epee5)
        else:
            self.root.epee5 = 0
            gpio_control.set(15, self.root.epee5)

    def change_weapon_connection_type(_):
        gpio_control.button_emu(27, 1)

    def passive_stop_card(self, state):
        if self.root.timer_running != 1 and state == "down":
            self.passive_timer.clear()

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

        if uart_data.symbol == 0:
            if self.old_sec != str(timer_s):
                root.time_updated = True
                root.flash_timer = time.time()
            else:
                root.time_updated = False

            if uart_data.on_timer == 0:
                root.color_timer = app.color_timer_orange
            elif timer_m == 0 and timer_d == 0:
                root.color_timer = app.color_timer_blue
                if self.timer_interval is None:
                    self.timer_interval = Clock.schedule_interval(self.update_millis, 0.02)
                    self.timer_millis = 0
                    root.timer_1 = ":"
                    root.timer_2 = "9"
                    root.timer_0 = "9"
                root.timer_0 = str(timer_s - 1)

            elif timer_m > 0 or root.color_timer == app.color_timer_orange:
                root.color_timer = app.color_timer_white

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
            root.timer_text = KivyApp.Symbols[timer_d] + KivyApp.Symbols[timer_s]

        root.warning_l = uart_data.warning_l
        root.warning_r = uart_data.warning_r

        if root.timer_running == 1 and ((timer_m > 0 and timer_m + timer_d + timer_s > 1) or self.passive_timer.prev_time != 0) and not self.root.weapon == 1:
            self.passive_timer.start()
        else:
            self.passive_timer.stop()

        self.prev_uart_data = uart_data

    def get_data(self, _):
        root = self.root
        root.current_time = time.time()
        if is_banana:
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
                    [1, 0, 0, 0, 0, 0, 0, 0][::-1],
                    [1, 0, 1, 0, 0, 0, 1, 1][::-1],
                    [1, 1, 0, 0, 0, 0, 0, 0][::-1],
                    [1, 1, 1, 0, 0, 1, 1, 0][::-1],]
            self.data_update(data)

        pins_data = PinsData(gpio_control.read_pins())
        

        if pins_data.poweroff == 0:
            self.system_poweroff()

        # Recording section
        # -----------------
        if ((self.prev_pins_data is None or self.prev_pins_data.recording == 0) and pins_data.recording == 1) or (pins_data.recording == 1 and not (video_control.ffmpeg_proc is not None and video_control.ffmpeg_proc.poll() is None)):
            video_control.start_recording()
        elif self.prev_pins_data is not None and self.prev_pins_data.recording == 1 and pins_data.recording == 0:
            video_control.save_clip()
            Clock.schedule_once(lambda _: video_control.stop_recording(), 3)
        # -----------------

        root.weapon = pins_data.weapon
        if root.weapon == 1:
            self.passive_timer.clear()
        root.weapon_connection_type = pins_data.wireless

        self.passive_timer.update()
        root.passive_size = self.passive_timer.get_size()
        root.passive_time = self.passive_timer.get_time()
        root.passive_coun = self.passive_timer.get_coun()
        root.color_passive = self.color_passive_red if root.passive_time > 50 else self.color_passive_yel

        if video_control.ffmpeg_proc is not None and video_control.ffmpeg_proc.poll() is not None:
            video_control.ffmpeg_proc = None
        root.recording = (video_control.ffmpeg_proc is not None) and (video_control.ffmpeg_proc.poll() is None)

        self.prev_pins_data = pins_data

        cmds = gpio_control.read_rc5(self.config["rc5_address"])
        if len(cmds) > 0:
            self.root.debug = str(cmds)
        
        if 7 in cmds:
            if self.root.timer_running != 1:
                self.passive_timer.clear()
        if 17 in cmds: # Left passive
            if self.root.timer_running != 1:
                self.passive_timer.clear()
                if root.ids["passive_2"].state == "normal":
                    root.ids["passive_2"].state = "down"
                elif root.ids["passive_1"].state == "normal":
                    root.ids["passive_1"].state = "down"
                else:
                    root.ids["passive_2"].state = "normal"
                    root.ids["passive_1"].state = "normal"

        if 18 in cmds: # Right passive
            if self.root.timer_running != 1:
                self.passive_timer.clear()
                if root.ids["passive_4"].state == "normal":
                    root.ids["passive_4"].state = "down"
                elif root.ids["passive_3"].state == "normal":
                    root.ids["passive_3"].state = "down"
                else:
                    root.ids["passive_4"].state = "normal"
                    root.ids["passive_3"].state = "normal"

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
        self.color_left_score     = [227 / 255,  30 / 255,  36 / 255, 1.0] # red
        self.color_right_score    = [  0 / 255, 152 / 255,  70 / 255, 1.0] # green

        self.color_period         = [  0 / 255, 160 / 255, 227 / 255, 1.0] # blue

        self.color_timer_white    = [223 / 255, 223 / 255, 223 / 255, 1.0] # white
        self.color_timer_orange   = [239 / 255, 127 / 255,  26 / 255, 1.0] # orange
        self.color_timer_blue     = [  0 / 255, 160 / 255, 227 / 255, 1.0] # blue

        self.color_warn_red_ena   = [227 / 255,  30 / 255,  36 / 255, 1.0] # red
        self.color_warn_red_dis   = [227 / 255,  30 / 255,  36 / 255, 0.2] # dark red
        self.color_warn_yel_ena   = [204 / 255, 204 / 255,   0 / 255, 1.0] # yellow
        self.color_warn_yel_dis   = [ 51 / 255,  51 / 255,   0 / 255, 1.0] # dark yellow
        self.color_warn_text_ena  = [230 / 255, 230 / 255, 230 / 255, 1.0] # white
        self.color_warn_text_dis  = [102 / 255, 102 / 255, 102 / 255, 1.0] # grey

        self.color_passive_yel    = [204 / 255, 204 / 255,   0 / 255, 1.0] # yellow
        self.color_passive_red    = [227 / 255,  30 / 255,  36 / 255, 1.0] # red
        self.color_passive_white  = [223 / 255, 223 / 255, 223 / 255, 1.0] # white
        self.color_passive_inact  = [ 76 / 255,  76 / 255,  76 / 255, 1.0] # gray

        self.color_left_p_ena     = [227 / 255,  30 / 255,  36 / 255, 1.0] # red
        self.color_left_p_dis     = [227 / 255,  30 / 255,  36 / 255, 0.2] # dark red
        self.color_right_p_ena    = [  0 / 255, 152 / 255,  70 / 255, 1.0] # green
        self.color_right_p_dis    = [  0 / 255, 152 / 255,  70 / 255, 0.2] # dark green

        self.color_weapon_ena     = [179 / 255, 179 / 255, 179 / 255, 1.0] # light gray
        self.color_weapon_dis     = [ 76 / 255,  76 / 255,  76 / 255, 1.0] # dark gray
        self.color_rec            = [255 / 255,   0 / 255,   0 / 255, 1.0] # red
        self.color_black          = [  0 / 255,   0 / 255,   0 / 255, 1.0] # black

        self.card_radius = 10

        self.old_sec              = "0"
        self.passive_timer        = PassiveTimer(500)
        self.timer_interval       = None
        self.timer_millis         = 0

        self.led_schedule   = None
        self.prev_uart_data = None
        self.prev_pins_data = None

        self.read_timer = None

        self.config = {"rc5_address": -1}

        if is_banana:
            self.data_rx = serial.Serial("/dev/ttyS2", 38400)
        else:
            self.data_rx = None

        config_path = pathlib.Path("config/config.json")
        if config_path.is_file():
            with open("config/config.json", "r") as config_file:
                self.config = json.load(config_file)
        else:
            if config_path.is_dir():
                print("Config file is directory, removing")
                shutil.rmtree(config_path)
            print("No config file, creating!")
            self.update_config()

        if not input_support:
            self.config["rc5_address"] = gpio_control.update_addr(self.data_rx, self.config["rc5_address"])
            self.update_config()
        return Builder.load_file("main.kv")

    def on_start(self):
        self.read_timer = Clock.schedule_interval(self.get_data, read_interval)
        Clock.schedule_interval(self.update_network_data, 2)
        self.root.flash_timer  = time.time()
        self.root.current_time = time.time()
        self.load_video_list()

    def on_stop(self):
        if self.data_rx is not None:
            self.data_rx.close()

if __name__ == "__main__":
    LabelBase.register(name="agencyb", fn_regular="assets/AGENCYB.TTF")
    LabelBase.register(name="agencyr", fn_regular="assets/AGENCYR.TTF")
    app = KivyApp()
    app.run()

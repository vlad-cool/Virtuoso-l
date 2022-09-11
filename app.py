#!/usr/bin/env python3
import kivy
from kivy.core.window import Window
from kivy.lang import Builder
from kivy.clock import Clock
from kivy.app import App
import platform
import subprocess
from collections import deque
from kivy.core.text import LabelBase
import serial
import time
import get_address
import gpio_control

read_interval = .2
send_interval = .2

if platform.machine() == "armv7l": #for bananapi, it have much better performance when running vertically
    Window.rotation = 90

kivy.require('2.1.0')

class KivyApp(App):
    def write_address(a):
        rc5_address = get_address()
        with open("rc5_address", "w") as address_file:
            address_file.write(f"{rc5_address}\n")

    def send_handler(self, code):
        self.send_queue.append(str(self.rc5_address * (2**6) + code))

    def carousel_handler(self, a, old_index, new_index, commands):
        self.send_handler(commands[(2 + new_index - old_index) % 3])
        
    def set_weapon(self, a, new_weapon):
        if platform.machine() != "armv7l":
            print(f"weapon: {new_weapon}")
            return

        subprocess.Popen(f"sudo ./weapon {3 + new_weapon - self.weapon}", shell=True)
        self.weapon = new_weapon

    def set_weapon_connection_type(a, type):
        if platform.machine() != "armv7l":
            print(f"weapon connection type: {type}")
            return

        subprocess.Popen(f"sudo ./change_weapon_type", shell=True)

    def send_data(self, dt):
        if self.send_proc is not None and self.send_proc.poll() is None:
            return
        if len(self.send_queue) > 0:
            if platform.machine() != "armv7l":
                print(self.send_queue[0])
                self.send_queue.popleft()
                return
            self.send_proc = subprocess.Popen(f"sudo ./output {self.send_queue[0]} {self.toggle_bit}", shell=True)
            self.toggle_bit = 1 - self.toggle_bit
            self.send_queue.popleft()

    def byte_to_arr(byte):
        a = [0] * 8
        for i in range(8):
                byte[i] = a % 2
                byte //= 2
        return a[::-1]

    def get_data(self, dt):
        data = []

        if platform.machine() == "armv7l":
            while self.data_rx.inWaiting // 8 > 0:
                for i in range(8):
                    byte = int.from_bytes(self.data_rx.read(), "big")
                    data[byte // 2 ** 5] = self.byte_to_arr(byte)
        
            self.weapon                 = gpio_control.read_pin(32) * 2 + gpio_control.read_pin(36)
            self.weapon_connection_type = gpio_control.read_pin(7)
            self.video_timer            = gpio_control.read_pin(18)

        else:
            data = [[0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 1, 0, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 0, 0],
                    [0, 1, 1, 0, 0, 0, 0, 0],
                    [1, 0, 0, 0, 0, 0, 0, 0],
                    [1, 0, 1, 0, 0, 0, 0, 0],
                    [1, 1, 0, 0, 0, 0, 0, 0],
                    [1, 1, 1, 0, 0, 0, 0, 0],]
        
        score_r = data[8][3]
        for i in data[6][3:]:
            score_r *= 2
            score_r += i

        score_l = data[7][3]
        for i in data[5][3:]:
            score_l *= 2
            score_l += i

        if score_l < 10:
            self.score_l_l = str(score_l)
            self.score_l_r = " "
        else:
            self.score_l_l = str(score_l // 10)
            self.score_l_r = str(score_l % 10)

        if score_r < 10:
            self.score_r_l = " "
            self.score_r_r = str(score_r)
        else:
            self.score_r_l = str(score_r // 10)
            self.score_r_r = str(score_r % 10)

        self.timer_0 = 0
        self.timer_2 = 0
        self.timer_3 = 0
        period = 0

        for i in data[2][6:]:
            timer_0 *= 2
            timer_0 += i

        for i in data[3][4:]:
            timer_2 *= 2
            timer_2 += i

        for i in data[4][4:]:
            timer_3 *= 2
            timer_3 += i

        for i in range(4):
            period *= 2
            period +=data[7][4 + i]

        if period == 15:
            self.priority = 1
        elif period == 14:
            self.priority = -1
        elif period == 13:
            self.priority = 0
        elif period >= 1 and period <= 9:
            self.priority = 0
            self.period = period
            
        if self.passive_timer == -1 and data[0][4] == 1:
            self.passive_timer = time.time()
        elif self.passive_timer != -1 and (data[3][3] == 0 and data[8][0] != 0):
            self.passive_timer = -1

        if self.passive_timer != -1:
            current_time = int(time.time() - self.passive_timer)

            self.passive_yel_size = min(self.passive_yel_max_size * current_time // 30, self.passive_yel_max_size)
            self.passive_red_size = min(max(self.passive_red_max_size * (current_time - 30) // 30, 0), self.passive_red_max_size)

        return

        


        
        
        
        if data[8][5]:
            app.root.ids["warning_bot_l"].state = "down"
            app.root.ids["warning_top_l"].state = "normal"
        elif data[8][4]:
            app.root.ids["warning_bot_l"].state = "down"
            app.root.ids["warning_top_l"].state = "down"
        else:
            app.root.ids["warning_bot_l"].state = "normal"
            app.root.ids["warning_top_l"].state = "normal"

        if data[8][7]:
            app.root.ids["warning_bot_r"].state = "down"
            app.root.ids["warning_top_r"].state = "normal"
        elif data[8][6]:
            app.root.ids["warning_bot_r"].state = "down"
            app.root.ids["warning_top_r"].state = "down"
        else:
            app.root.ids["warning_bot_r"].state = "normal"
            app.root.ids["warning_top_r"].state = "normal"

        
        if data[8][4] == 1:
            app.root.ids["warning_bot_l"].state = "down"
            app.root.ids["warning_top_l"].state = "down"
        elif data[8][5] == 1:
            app.root.ids["warning_bot_l"].state = "down"
            app.root.ids["warning_top_l"].state = "normal"
        else:
            app.root.ids["warning_bot_l"].state = "normal"
            app.root.ids["warning_top_l"].state = "normal"

        if data[8][6] == 1:
            app.root.ids["warning_bot_r"].state = "down"
            app.root.ids["warning_top_r"].state = "down"
        elif data[8][7] == 1:
            app.root.ids["warning_bot_r"].state = "down"
            app.root.ids["warning_top_r"].state = "normal"
        else:
            app.root.ids["warning_bot_r"].state = "normal"
            app.root.ids["warning_top_r"].state = "normal"

        if timer_s % 2 == 0 or data[3][3] == 0:
            if app.root.ids["timer_dot"].text != ":" and passive_timer != -1:

                app.root.ids["passive_red"].size[0] = 0
                pass
            app.root.ids["timer_dot"].text = ":"
        else:
            app.root.ids["timer_dot"].text = " "

    def build(self):
        self.send_queue = deque()
        self.toggle_bit = 1
        self.weapon = 0
        self.weapon_connection_type = 0
        self.video_timer = 0
        self.send_proc = None
        self.passive_timer = -1
        self.rc5_address = 0
        self.passive_yel_max_size = 40
        self.passive_red_max_size = 40
        self.passive_yel_size = 0
        self.passive_red_size = 0
        self.score_l_l = "0"
        self.score_l_r = " "
        self.score_r_l = " "
        self.score_r_r = "0"
        self.timer_0 = "0"
        self.timer_1 = ":"
        self.timer_2 = "0"
        self.timer_3 = "0"
        self.timer_running = 0
        self.period = 0
        self.priority = 0

        self.color_left_score     = [0.8, 0.0, 0.0, 1]
        self.color_right_score    = [0.0, 0.8, 0.0, 1]
        self.color_period         = [0.1, 0.1, 0.8, 1]
        self.color_timer_enabled  = [1.0, 1.0, 1.0, 1]
        self.color_timer_disabled = [0.8, 0.4, 0.0, 1]

        if platform.machine() == "armv7l":
            self.data_rx = serial.Serial("/dev/ttyS2", 38400)
        else:
            self.data_rx = None
        
        return Builder.load_file("main.kv")

    def on_start(self):

        Clock.schedule_interval(self.get_data, read_interval)
        Clock.schedule_interval(self.send_data, send_interval)
        with open("./rc5_address", "r") as address_file:
            self.rc5_address = int(address_file.readline())

    def on_stop(self):
        self.data_rx.close()

if __name__ == "__main__":
    LabelBase.register(name="agencyb", fn_regular='AGENCYB.TTF')
    LabelBase.register(name="agencyr", fn_regular='AGENCYR.TTF')
    app = KivyApp()
    app.run()

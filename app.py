#!/usr/bin/env python3
import kivy
import time
import serial
import get_address
import gpio_control
from kivy.core.window import Window
from kivy.lang        import Builder
from kivy.clock       import Clock
from kivy.app         import App
from kivy.core.text   import LabelBase
from collections      import deque
from platform         import machine

read_interval = .2
send_interval = .2

if machine() == "armv7l": #for bananapi, it have much better performance when running vertically
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
        if machine() != "armv7l":
            print(f"weapon: {new_weapon}")
            return

        gpio_control.button_emu(37, (3 + new_weapon - self.weapon) % 3)
        self.weapon = new_weapon

    def change_weapon_connection_type(a):
        if machine() != "armv7l":
            print("weapon connection type changed")
            return

        gpio_control.button_emu(27, 1)

    def send_data(self, dt):
        if self.send_proc is not None and self.send_proc.poll() is None:
            return
        if len(self.send_queue) > 0:
            if machine() != "armv7l":
                print(self.send_queue[0])
                self.send_queue.popleft()
                return
            #self.send_proc = subprocess.Popen(f"sudo ./output {self.send_queue[0]} {self.toggle_bit}", shell=True)
            self.toggle_bit = 1 - self.toggle_bit
            self.send_queue.popleft()

    def byte_to_arr(byte):
        a = [0] * 8
        for i in range(8):
                byte[i] = a % 2
                byte //= 2
        return a[::-1]

    def get_data(self, dt):
        data = [] * 8

        if machine() == "armv7l":
            while self.data_rx.inWaiting // 8 > 0:
                for i in range(8):
                    byte = int.from_bytes(self.data_rx.read(), "big")
                    data[byte // 2 ** 5] = self.byte_to_arr(byte)
        
            self.weapon                 = gpio_control.read_pin(32) * 2 + gpio_control.read_pin(36)
            self.video_timer            = gpio_control.read_pin(18)
            self.weapon_connection_type = gpio_control.read_pin(7)

        else:
            data = [[0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 1, 0, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 0, 0],
                    [0, 1, 1, 0, 0, 0, 0, 0],
                    [1, 0, 0, 0, 0, 0, 0, 0],
                    [1, 0, 1, 0, 0, 0, 0, 0],
                    [1, 1, 0, 0, 0, 0, 0, 0],
                    [1, 1, 1, 0, 0, 0, 0, 0],]
        
        score_r = data[7][3]
        for i in data[5][3:]:
            score_r *= 2
            score_r += i

        score_l = data[6][3]
        for i in data[4][3:]:
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

        timer_m = 0
        timer_d = 0
        timer_s = 0

        for i in data[1][6:]:
            timer_m *= 2
            timer_m += i

        for i in data[2][4:]:
            timer_d *= 2
            timer_d += i

        for i in data[3][4:]:
            timer_s *= 2
            timer_s += i
        
        if self.timer_3 != str(timer_s):
            self.flash_timer = time.time()

        self.timer_0 = str(timer_m)
        self.timer_2 = str(timer_d)
        self.timer_3 = str(timer_s)
        self.timer_running = data[2][3]

        period = 0
        
        for i in range(4):
            period *= 2
            period += data[6][4 + i]

        if period == 15:
            self.priority = 1
        elif period == 14:
            self.priority = -1
        elif period == 13:
            self.priority = 0
        elif period >= 1 and period <= 9:
            self.priority = 0
            self.period = period
            
        if self.passive_timer == -1 and self.timer_running == 1:
            self.passive_timer = time.time()
        elif self.passive_timer != -1 and self.timer_running == 0:
            self.passive_timer = -1

        if self.passive_timer != -1:
            current_time = int(time.time() - self.passive_timer)

            self.passive_yel_size = min(self.passive_yel_max_size * current_time // 30, self.passive_yel_max_size)
            self.passive_red_size = min(max(self.passive_red_max_size * (current_time - 30) // 30, 0), self.passive_red_max_size)

        self.warning_l = data[7][4] * 2 + data[7][5]
        self.warning_r = data[7][7] * 2 + data[7][6]

    def build(self):
        self.send_queue = deque()
        self.toggle_bit = 1
        self.rc5_address = 0
        self.send_proc = None

        self.weapon = 0
        self.weapon_connection_type = 0
        
        self.video_timer = 0

        self.passive_yel_max_size = 40
        self.passive_red_max_size = 40
        self.passive_yel_size = 0
        self.passive_red_size = 0
        self.passive_timer = -1

        self.score_l_l = "0"
        self.score_l_r = " "
        self.score_r_l = " "
        self.score_r_r = "0"

        self.timer_0 = "0"
        self.timer_1 = ":"
        self.timer_2 = "0"
        self.timer_3 = "0"

        self.flash_timer = time.time()
        self.timer_running = 0
        self.period = 0
        self.priority = 0
        self.warning_l = 0
        self.warning_r = 0

        self.color_left_score     = [0.8, 0.0, 0.0, 1]
        self.color_right_score    = [0.0, 0.8, 0.0, 1]
        self.color_period         = [0.1, 0.1, 0.8, 1]
        self.color_timer_enabled  = [1.0, 1.0, 1.0, 1]
        self.color_timer_disabled = [0.8, 0.4, 0.0, 1]

        if machine() == "armv7l":
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
        if self.data_rx is not None:
            self.data_rx.close()

if __name__ == "__main__":
    LabelBase.register(name="agencyb", fn_regular='AGENCYB.TTF')
    LabelBase.register(name="agencyr", fn_regular='AGENCYR.TTF')
    app = KivyApp()
    app.run()

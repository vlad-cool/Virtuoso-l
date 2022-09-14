#!/usr/bin/env python3
import kivy
import time
import serial
import get_address
import gpio_control
from kivy.clock       import Clock
from kivy.core.window import Window
from kivy.lang        import Builder
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
        self.send_queue.append(self.rc5_address * (2**6) + code)

    def carousel_handler(self, a, old_index, new_index, commands):
        self.send_handler(commands[(2 + new_index - old_index) % 3])
        
    def set_weapon(self, new_weapon):
        if machine() != "armv7l":
            print(f"weapon: {new_weapon}")
            return

        gpio_control.button_emu(37, (3 + new_weapon - self.root.weapon) % 3)
        self.weapon = new_weapon

    def change_weapon_connection_type(a):
        if machine() != "armv7l":
            print("weapon connection type changed")
            return

        gpio_control.button_emu(27, 1)

    def send_data(self, dt):
        #if self.send_proc is not None and self.send_proc.poll() is None:
        #    return
        if len(self.send_queue) > 0:
            if machine() != "armv7l":
                print(self.send_queue[0])
                self.send_queue.popleft()
                return
            #self.send_proc = subprocess.Popen(f"sudo ./output {self.send_queue[0]} {self.toggle_bit}", shell=True)
            gpio_control.ir_emu(self.send_queue[0], self.toggle_bit)
            self.toggle_bit = 1 - self.toggle_bit
            self.send_queue.popleft()

    def byte_to_arr(self, byte):
        a = [0] * 8
        for i in range(8):
                a[i] = byte % 2
                byte //= 2
        return a[::-1]

    def data_update(self, data):
        root = self.root

        score_r = data[7][3]
        for i in data[5][3:]:
            score_r *= 2
            score_r += i

        score_l = data[6][3]
        for i in data[4][3:]:
            score_l *= 2
            score_l += i

        if score_l < 10:
            root.score_l_l = str(score_l)
            root.score_l_r = " "
        else:
            root.score_l_l = str(score_l // 10)
            root.score_l_r = str(score_l % 10)

        if score_r < 10:
            root.score_r_l = " "
            root.score_r_r = str(score_r)
        else:
            root.score_r_l = str(score_r // 10)
            root.score_r_r = str(score_r % 10)

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
        
        if self.old_sec != str(timer_s):
            root.flash_timer = time.time()
            pass

        self.old_sec = str(timer_s)
        root.timer_0 = str(timer_m)
        root.timer_2 = str(timer_d)
        root.timer_3 = str(timer_s)
        root.timer_running = data[2][3]

        period = 0
        
        for i in range(4):
            period *= 2
            period += data[6][4 + i]

        if period == 15:
            root.priority = 1
        elif period == 14:
            root.priority = -1
        elif period == 13:
            root.priority = 0
        elif period >= 1 and period <= 9:
            root.priority = 0
            root.period = period
            
        if root.passive_timer == -1 and root.timer_running == 1:
            root.passive_timer = time.time()
        elif root.passive_timer != -1 and root.timer_running == 0:
            root.passive_timer = -1

        if root.passive_timer != -1:
            current_time = int(time.time() - root.passive_timer)

            root.passive_yel_size = min(self.passive_yel_max_size * current_time // 30, self.passive_yel_max_size)
            root.passive_red_size = min(max(self.passive_red_max_size * (current_time - 30) // 30, 0), self.passive_red_max_size)

        root.warning_l = data[7][4] * 2 + data[7][5]
        root.warning_r = data[7][6] * 2 + data[7][7]


    def get_data(self, dt):
        self.root.current_time = time.time()
        if machine() == "armv7l":
            while self.data_rx.inWaiting() // 8 > 0:
                data = [[0] * 8] * 8
                for i in range(8):
                    byte = int.from_bytes(self.data_rx.read(), "big")
                    data[byte // 2 ** 5] = self.byte_to_arr(byte)

                self.data_update(data)

            self.root.weapon                 = gpio_control.read_pin(32) * 2 + gpio_control.read_pin(36)
            self.root.video_timer            = gpio_control.read_pin(18)
            self.root.weapon_connection_type = gpio_control.read_pin(7)

        else:
            data = [[0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 1, 0, 0, 0, 0, 0],
                    [0, 1, 0, 1, 0, 0, 0, 0],
                    [0, 1, 1, 0, 0, 0, 0, 0],
                    [1, 0, 0, 0, 0, 0, 0, 0],
                    [1, 0, 1, 0, 0, 0, 1, 1],
                    [1, 1, 0, 0, 0, 0, 0, 0],
                    [1, 1, 1, 0, 0, 1, 1, 0],]
            self.data_update(data)
        
    def build(self):
        self.send_proc            = None
        self.send_queue           = deque()
        self.toggle_bit           = 1
        self.rc5_address          = 0
        self.old_sec = "0"

        self.passive_yel_max_size = 40
        self.passive_red_max_size = 40

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
        self.root.flash_timer  = time.time()
        self.root.current_time = time.time()
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

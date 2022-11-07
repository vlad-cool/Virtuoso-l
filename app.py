#!venv/bin/python3
from asyncio import new_event_loop
import kivy
import time
import serial
import subprocess
import gpio_control
from kivy.clock       import Clock
from kivy.core.window import Window
from kivy.lang        import Builder
from kivy.app         import App
from kivy.core.text   import LabelBase
from platform         import machine

read_interval = .2
send_interval = .2

if machine() == "armv7l": #for bananapi, it have much better performance when running vertically
    Window.rotation = 90

kivy.require('2.1.0')

class KivyApp(App):
    #Symbols = ["A", "B", "C", "D", "E", "F", "Sc", "On", "Off", " ", "1", "2", "3", "4", "5", "6"]
    Symbols = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"]

    def run_app(self, s):
        print(s)
        if self.proc is None or self.proc.poll():
            self.proc = subprocess.Popen("./kivy_test.py",shell=False)

    def system_poweroff(a):
        subprocess.run(["sudo", "poweroff"])

    def system_reboot(a):
        subprocess.run(["sudo", "reboot"])

    def write_address(self):
        rc5_address = gpio_control.get_address(self.data_rx)
        with open("rc5_address", "w") as address_file:
            address_file.write(f"{rc5_address}\n")

    def send_handler(self, code):
        gpio_control.ir_emu(self.rc5_address * (2**6) + code)

    def carousel_handler(self, a, old_index, new_index, commands):
        self.send_handler(commands[(2 + new_index - old_index) % 3])
        
    def set_weapon(self, new_weapon):
        if machine() != "armv7l":
            print(f"weapon: {new_weapon}")
            return

        gpio_control.button_emu(37, (3 + new_weapon - self.root.weapon) % 3)
        self.weapon = new_weapon

        if self.root.weapon == 3 and new_weapon == 0:
            self.root.epee5 = 1 - self.root.epee5
            gpio_control.gpio.digitalWrite(15, self.root.epee5)
        else:
            self.root.epee5 = 0
            gpio_control.gpio.digitalWrite(15, self.root.epee5)

    def change_weapon_connection_type(a):
        if machine() != "armv7l":
            print("weapon connection type changed")
            return

        gpio_control.button_emu(27, 1)

    def hide_passive(self, dt):
        self.root.passive_yel_size = 0
        self.root.passive_red_size = 0

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
        
        if data[1][5] == 0:
            
            if self.old_sec != str(timer_s):
                root.flash_timer = time.time()
                pass

            self.old_sec = str(timer_s)
            root.timer_0 = str(timer_m)
            root.timer_1 = ":"
            root.timer_2 = str(timer_d)
            root.timer_3 = str(timer_s)
            root.timer_running = data[2][3]
            root.timer_text = ""
        else:
            root.timer_0 = ""
            root.timer_1 = ""
            root.timer_2 = ""
            root.timer_3 = ""
            root.timer_text = KivyApp.Symbols[timer_d] + KivyApp.Symbols[timer_s]

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
            root.period = period
            
        if root.passive_timer == -1 and root.timer_running == 1:
            root.passive_timer = time.time()
        elif root.passive_timer != -1 and root.timer_running == 0:
            root.passive_timer = -1
            Clock.schedule_once(self.hide_passive, 2)

        if root.passive_timer != -1:
            current_time = int(time.time() - root.passive_timer)
            root.passive_yel_size = min(self.passive_yel_max_size * current_time // 30, self.passive_yel_max_size)
            root.passive_red_size = min(max(self.passive_red_max_size * (current_time - 30) // 20, 0), self.passive_red_max_size)

        root.warning_l = data[7][4] * 2 + data[7][5]
        root.warning_r = data[7][6] * 2 + data[7][7]

    def get_data(self, dt):
        if machine() == "armv7l":
            self.root.current_time = time.time()
            data = [[0] * 8] * 8
            while self.data_rx.inWaiting() // 8 > 0:
                for i in range(8):
                    byte = int.from_bytes(self.data_rx.read(), "big")
                    data[byte // 2 ** 5] = self.byte_to_arr(byte)

                print("data_got!")
                print(data[2][3])
                print(str(data).replace("], ", "]]\n["))
                self.data_update(data)
            if 37 not in gpio_control.button_emulating or self.root.timer_running:
                self.root.weapon                 = 0
                self.root.weapon                 = gpio_control.read_pin(32) * 2 + gpio_control.read_pin(36)
            if 27 not in gpio_control.button_emulating or self.root.timer_running:
                self.root.weapon_connection_type = 0
                self.root.weapon_connection_type = gpio_control.read_pin(7)
            self.root.video_timer            = gpio_control.read_pin(18)

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
        self.toggle_bit           = 1
        self.rc5_address          = 0
        self.old_sec = "0"

        self.passive_yel_max_size = 115
        self.passive_red_max_size = 115

        self.color_left_score     = [227 / 255,  30 / 255,  36 / 255, 1.0] # red
        self.color_right_score    = [  0 / 255, 152 / 255,  70 / 255, 1.0] # green
        self.color_period         = [  0 / 255, 160 / 255, 227 / 255, 1.0] # blue
        self.color_timer_enabled  = [255 / 255, 255 / 255, 255 / 255, 1.0] # white
        self.color_timer_disabled = [239 / 255, 127 / 255,  26 / 255, 1.0] # orange

        self.color_warn_red_ena   = [0.8, 0.0, 0.0, 1] # red
        self.color_warn_red_dis   = [0.2, 0.0, 0.0, 1] # dark red
        self.color_warn_yel_ena   = [0.8, 0.8, 0.0, 1] # yellow
        self.color_warn_yel_dis   = [0.2, 0.2, 0.0, 1] # dark yellow

        self.color_passive_yel    = [0.8, 0.8, 0.0, 1] # yellow
        self.color_passive_red    = [0.8, 0.0, 0.0, 1] # red

        self.color_left_p_ena     = [227 / 255,  30 / 255,  36 / 255, 1.0] # red
        self.color_left_p_dis     = [227 / 255,  30 / 255,  36 / 255, 0.2] # dark red
        self.color_right_p_ena    = [  0 / 255, 152 / 255,  70 / 255, 1.0] # green
        self.color_right_p_dis    = [  0 / 255, 152 / 255,  70 / 255, 0.2] # dark green

        self.color_weapon_ena     = [0.7, 0.7, 0.7, 1] # light gray
        self.color_weapon_dis     = [0.3, 0.3, 0.3, 1] # dark gray

        self.card_radius = 10

        self.proc = None

        if machine() == "armv7l":
            self.data_rx = serial.Serial("/dev/ttyS2", 38400)
        else:
            self.data_rx = None
        
        return Builder.load_file("main.kv")

    def on_start(self):
        Clock.schedule_interval(self.get_data, read_interval)
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

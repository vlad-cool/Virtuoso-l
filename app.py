#!venv/bin/python3
import kivy
import time
import serial
import subprocess
import gpio_control
from kivy.clock       import Clock
from kivy.lang        import Builder
from kivy.app         import App
from kivy.core.text   import LabelBase
from platform         import machine

read_interval = .05

kivy.require('2.1.0')

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
            self.coun = str(60 - int(self.time))
            if len(self.coun) == 1:
                self.coun = " " + self.coun
        else:
            self.coun = " 0"
        self.prev_time = cur_time
    
    def __init__(self, passive_max_size):
        self.max_size = passive_max_size
        self.clear()

class KivyApp(App):
    #Symbols = ["A", "B", "C", "D", "E", "F", "Sc", "On", "Off", " ", "1", "2", "3", "4", "5", "6"]
    Symbols = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"]

    def test_bootsplash(abobus):
        subprocess.Popen("sudo plymouthd", shell=True)
        time.sleep(10)
        subprocess.Popen("sudo plymouth --show-splash", shell=True)
        time.sleep(10)
        subprocess.Popen("sudo plymouth quit", shell=True)

    def start_camera(self):
        if self.camera_proc is None:
            self.camera_proc = subprocess.Popen("./run_cam.sh", shell=True)

    def run_app(self, s):
        if self.proc is None or self.proc.poll():
            self.proc = subprocess.Popen("./kivy_test.py", shell=False)

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
            return

        gpio_control.button_emu(27, 1)

    def passive_stop_card(self, state):
        if self.root.timer_running != 1 and state == "down":
            self.passive_timer.clear()

    def byte_to_arr(self, byte):
        a = [0] * 8
        for i in range(8):
                a[i] = byte % 2
                byte //= 2
        return a[::-1]

    def update_millis(self, dt):
        if self.root.timer_running == 0:
            return
        self.timer_millis += dt
        t = 99 - int(self.timer_millis * 100) % 100
        app.root.timer_2 = str(t // 10)
        app.root.timer_3 = str(t %  10)

    def data_update(self, data):
        root = self.root

        if data[0][4] + data[0][5] + data[0][7] + data[1][3] > 0 and root.timer_running:
            self.passive_timer.clear()

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
        if period in [12, 13] and self.raw_period not in [12, 13]:
            self.passive_timer.clear()
        if period == 12 and timer_m == 0:
            timer_m = 4
        self.raw_period = period
        
        if data[1][5] == 0:
            if self.old_sec != str(timer_s):
                root.flash_timer = time.time()
                pass

            if data[2][3] == 0:
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
            root.timer_running = data[2][3]

        else:
            root.timer_0 = ""
            root.timer_1 = ""
            root.timer_2 = ""
            root.timer_3 = ""
            root.timer_text = KivyApp.Symbols[timer_d] + KivyApp.Symbols[timer_s]

        root.warning_l = data[7][4] * 2 + data[7][5]
        root.warning_r = data[7][6] * 2 + data[7][7]

        if root.timer_running == 1 and ((timer_m > 0 and timer_m + timer_d + timer_s > 1) or self.passive_timer.prev_time != 0):
            self.passive_timer.start()
        else:
            self.passive_timer.stop()

    def get_data(self, dt):
        if machine() == "armv7l":
            self.root.current_time = time.time()
            data = [[0] * 8] * 8
            while self.data_rx.inWaiting() // 8 > 0:
                for i in range(8):
                    byte = int.from_bytes(self.data_rx.read(), "big")
                    data[byte // 2 ** 5] = self.byte_to_arr(byte)

                self.data_update(data)
            if 37 not in gpio_control.button_emulating or self.root.timer_running:
                self.root.weapon = 0
                self.root.weapon = gpio_control.read_pin(32) * 2 + gpio_control.read_pin(36)
            if 27 not in gpio_control.button_emulating or self.root.timer_running:
                self.root.weapon_connection_type = 0
                self.root.weapon_connection_type = gpio_control.read_pin(7)
            self.root.video_timer = gpio_control.read_pin(18)

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

        root = self.root

        self.passive_timer.update()
        root.passive_size = self.passive_timer.get_size()
        root.passive_time = self.passive_timer.get_time()
        root.passive_coun = self.passive_timer.get_coun()
        root.color_passive = self.color_passive_red if root.passive_time > 50 else self.color_passive_yel
        
    def build(self):
        self.color_left_score     = [227 / 255,  30 / 255,  36 / 255, 1.0] # red
        self.color_right_score    = [  0 / 255, 152 / 255,  70 / 255, 1.0] # green
        self.color_period         = [  0 / 255, 160 / 255, 227 / 255, 1.0] # blue
        self.color_timer_white    = [223 / 255, 223 / 255, 223 / 255, 1.0] # white
        self.color_timer_orange   = [239 / 255, 127 / 255,  26 / 255, 1.0] # orange
        self.color_timer_blue     = [  0 / 255, 160 / 255, 227 / 255, 1.0] # blue

        self.color_warn_red_ena   = [227 / 255,  30 / 255,  36 / 255, 1.0] # red
        self.color_warn_red_dis   = [227 / 255,  30 / 255,  36 / 255, 0.2] # dark red
        self.color_warn_yel_ena   = [0.8, 0.8, 0.0, 1] # yellow
        self.color_warn_yel_dis   = [0.2, 0.2, 0.0, 1] # dark yellow
        self.color_warn_text_up   = [0.9, 0.9, 0.9, 1]
        self.color_warn_text_down = [0.4, 0.4, 0.4, 1]
        
        self.color_passive_yel    = [0.8, 0.8, 0.0, 1] # yellow
        self.color_passive_red    = [227 / 255,  30 / 255,  36 / 255, 1.0] # red
        self.color_passive_white  = [223 / 255, 223 / 255, 223 / 255, 1.0] # white

        self.color_left_p_ena     = [227 / 255,  30 / 255,  36 / 255, 1.0] # red
        self.color_left_p_dis     = [227 / 255,  30 / 255,  36 / 255, 0.2] # dark red
        self.color_right_p_ena    = [  0 / 255, 152 / 255,  70 / 255, 1.0] # green
        self.color_right_p_dis    = [  0 / 255, 152 / 255,  70 / 255, 0.2] # dark green

        self.color_weapon_ena     = [0.7, 0.7, 0.7, 1] # light gray
        self.color_weapon_dis     = [0.3, 0.3, 0.3, 1] # dark gray

        self.card_radius = 10

        self.proc                 = None
        self.camera_proc          = None
        self.toggle_bit           = 1
        self.rc5_address          = 0
        self.raw_period           = 0
        self.old_sec              = "0"
        self.passive_timer        = PassiveTimer(500)
        self.timer_interval       = None
        self.timer_millis         = 0

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

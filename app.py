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
import time
import get_address

send_queue = deque()
read_interval = .2
send_interval = .2
rc5_address = 0
toggle_bit = 1
weapon = 0
carousel_codes = [[3, 2, 2], [15, 9, 9], [6, 14, 13], [0, 0, 4], [0, 0, 11], [0, 0, 8]]
carousel_indexes = [0] * len(carousel_codes)
get_proc = None
send_proc = None
send_proc_running = False
passive_timer = -1

with open("./rc5_address", "r") as address_file:
    rc5_address = int(address_file.readline())

if platform.machine() == "armv7l": #for bananapi, it have much better performance when running vertically
    Window.rotation = 90

kivy.require('2.1.0')

class KivyApp(App):
    def write_address(a):
        with open("rc5_address", "w") as address_file:
            address_file.write(f"{get_address()}\n")

    def carousel_btn_handler(a, carousel_number):
        global carousel_indexes
        global carousel_codes
        global send_queue
        send_queue.append(str(rc5_address * (2**6) + carousel_codes[carousel_number][2]))

    def carousel_handler(a, new_index, carousel_number):
        global carousel_indexes
        global carousel_codes
        global send_queue
        send_queue.append(str(rc5_address * (2**6) + carousel_codes[carousel_number][(3 + new_index - carousel_indexes[carousel_number]) % 3 - 1]))
        carousel_indexes[carousel_number] = new_index % 3

    def set_weapon(a, new_weapon):
        if platform.machine() != "armv7l":
            print(f"weapon: {new_weapon}")
            return
        global weapon

        if (3 + new_weapon - weapon) % 3 == 1:
            subprocess.Popen("sudo ./weapon 1", shell=True)
        if (3 + new_weapon - weapon) % 3 == 2:
            subprocess.Popen("sudo ./weapon 2", shell=True)

        weapon = new_weapon

    def build(self):
        return Builder.load_file('main.kv')

    def on_start(seelf):
        pass

    def on_stop(self):
        pass

def get_data(dt):
    global weapon
    global send_queue
    global passive_timer
    data = []
    app = App.get_running_app()
    for i in range(9):
        data.append([0] * 8)
    with open("./gpio_in", "rb") as gpio_in:
        for j in range(9):
            b = gpio_in.read(1)
            a = int.from_bytes(b, "big")
            if a == 0 and j == 0:
                return
            for i in range(8):
                data[j][7 - i] = a % 2
                a //= 2
            if j == 0 and data[0][3] == 1:
                break
        if data[0][3] == 0:
            app.root.ids["test_output"].text = str(data)[1:-1].replace(", ", "").replace("[", "").replace("]", "\n") #debug output

    if 3 - data[0][7] * 2 - data[0][6] < 3:
        for i in range(3):
            app.root.ids[f"weapon_{i}"].state = "normal"
        weapon = (1 - data[0][7]) * 2 + 1 - data[0][6]
        app.root.ids[f"weapon_{weapon}"].state = "down"

    if data[8][0] == 0:
        return

    score_r = data[8][3]
    for i in data[6][3:]:
        score_r *= 2
        score_r += i

    score_l = data[7][3]
    for i in data[5][3:]:
        score_l *= 2
        score_l += i

    timer_m = 0
    timer_d = 0
    timer_s = 0

    for i in data[2][6:]:
        timer_m *= 2
        timer_m += i

    for i in data[3][4:]:
        timer_d *= 2
        timer_d += i

    for i in data[4][4:]:
        timer_s *= 2
        timer_s += i

    period = 0
    for i in range(4):
        period *= 2
        period +=data[7][4 + i]

    if period == 15:
        app.root.ids["priority_r"].state = "down"
        app.root.ids["priority_l"].state = "normal"
        app.root.ids["period"].text = ""
    elif period == 14:
        app.root.ids["priority_r"].state = "normal"
        app.root.ids["priority_l"].state = "down"
        app.root.ids["period"].text = ""
    elif period == 14:
        app.root.ids["priority_r"].state = "normal"
        app.root.ids["priority_l"].state = "normal"
        app.root.ids["period"].text = ""
    elif period >= 1 and period <= 9:
        app.root.ids["priority_r"].state = "normal"
        app.root.ids["priority_l"].state = "normal"
        app.root.ids["period"].text = str(period)

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

    if score_l < 10:
        app.root.ids["score_l_l"].text = str(score_l)
        app.root.ids["score_l_r"].text = " "
    else:
        app.root.ids["score_l_l"].text = str(score_l // 10)
        app.root.ids["score_l_r"].text = str(score_l % 10)

    if score_r < 10:
        app.root.ids["score_r_l"].text = " "
        app.root.ids["score_r_r"].text = str(score_r)
    else:
        app.root.ids["score_r_l"].text = str(score_r // 10)
        app.root.ids["score_r_r"].text = str(score_r % 10)

    app.root.ids["timer_min"].text = str(timer_m)
    app.root.ids["timer_dec"].text = str(timer_d)
    app.root.ids["timer_sec"].text = str(timer_s)

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

    if passive_timer == -1 and data[3][3] == 1:
        passive_timer = time.time()
    elif passive_timer != -1 and data[3][3] == 0:
        passive_timer = -1
        app.root.ids["passive_red"].state = "normal"

    if timer_s % 2 == 0 or data[3][3] == 0:
        if app.root.ids["timer_dot"].text != ":" and passive_timer != -1 and data[3][3] == 1:

            pass
        app.root.ids["timer_dot"].text = ":"
    else:
        app.root.ids["timer_dot"].text = " "

def send_data(dt):
    global send_queue
    global toggle_bit
    global send_proc
    global send_proc_running

    if send_proc_running and send_proc.poll() is None:
        return
    send_proc_running = False
    if len(send_queue) > 0:
        if platform.machine() != "armv7l":
            print(send_queue[0])
            send_queue.popleft()
            return
        send_proc = subprocess.Popen(f"sudo ./output {send_queue[0]} {toggle_bit}", shell=True)
        toggle_bit = 1 - toggle_bit
        send_queue.popleft()
        send_proc_running = True

if __name__ == "__main__":
    LabelBase.register(name="agencyb", fn_regular='AGENCYB.TTF')
    LabelBase.register(name="agencyr", fn_regular='AGENCYR.TTF')
    Clock.schedule_interval(get_data, read_interval)
    Clock.schedule_interval(send_data, send_interval)
    KivyApp().run()

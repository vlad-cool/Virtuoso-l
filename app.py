#!/usr/bin/env python3
import kivy
from kivy.core.window import Window
from kivy.lang import Builder
from kivy.app import App
from kivy.clock import Clock
import platform
import pickle

read_interval = .1 #(seconds) time between trying to read data from gpio_in pipe
global app_config #app config dictionaty
global gpio_in

try:
    with (open("config", "rb")) as f: #loading app config
        app_config = pickle.load(f)
except:
    app_config = {"weapon" : 1} #default config dictionary

if platform.machine() == "armv7l": #for bananapi, it have much better performance when running vertically
    Window.rotation = 90

kivy.require('2.1.0')

class KivyApp(App):
    def build(self):
        return Builder.load_file('main.kv')

    def on_start(self):
        app = App.get_running_app()
        #app.root.ids[f"weapon_{weapon}"].state = "down"

    def on_stop(self):
        pass


def get_data(dt):
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
        else:
            app.root.ids["test_output"].text = str(data)[1:-1].replace(", ", "").replace("[", "").replace("]", "\n") #debug output

    if data[8][0] == 0:
        return

    score_r = data[8][3]
    for i in data[6][3:]:
        score_r *= 2
        score_r += i

    score_r = str(score_r)
    if len(score_r) == 1:
        score_r = "  " + score_r

    score_l = data[7][3]
    for i in data[5][3:]:
        score_l *= 2
        score_l += i

    score_l = str(score_l)
    if len(score_l) == 1:
        score_l = score_l + "  "

    timer_m = 0
    timer_d = 0
    timer_s = 0
    timer = ""

    for i in data[2][6:]:
        timer_m *= 2
        timer_m += i

    for i in data[3][4:]:
        timer_d *= 2
        timer_d += i

    for i in data[4][4:]:
        timer_s *= 2
        timer_s += i

    timer = f"{timer_m}:{timer_d}{timer_s}"

    if 3 - data[0][7] * 2 - data[0][6] < 3:
        for i in range(3):
            app.root.ids[f"weapon_{i}"].state = "normal"
        weapon = (1 - data[0][7]) * 2 + 1 - data[0][6]
        app.root.ids[f"weapon_{weapon}"].state = "down"

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

    app.root.ids["score_l"].text = score_l
    app.root.ids["score_r"].text = score_r
    app.root.ids["timer"].text = timer


if __name__ == "__main__":
    Clock.schedule_interval(get_data, read_interval)
    KivyApp().run()

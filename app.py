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

weapon = app_config["weapon"]

if platform.machine() == "armv7l": #for bananapi, it have much better performance when running vertically
    Window.rotation = 90

kivy.require('2.1.0')  

class KivyApp(App):
    def build(self):
        return Builder.load_file('main.kv')

    def on_start(self):
        app = App.get_running_app()
        app.root.ids[f"weapon_{weapon}"].state = "down"

    def on_stop(self):
        gpio_in.close()


def get_data(dt):
    data = []
    app = App.get_running_app()
    for i in range(9):
        data.append([0] * 8)
    for j in range(9):
        b = gpio_in.read(1)
        a = int.from_bytes(b, "big")
        if a == 0 and j == 0:
            break
        for i in range(8):
            data[j][7 - i] = a % 2
            a //= 2
        if j == 0 and data[0][3] == 1:
            break
    else:
        app.root.ids["test_output"].text = str(data)[1:-1].replace(", ", "").replace("[", "").replace("]", "\n")

    if (1 - data[0][7]) * 2 + 1 - data[0][6] < 3:
            for i in range(3):
                app.root.ids[f"weapon_{i}"].state = "normal"
            weapon = (1 - data[0][7]) * 2 + 1 - data[0][6]
            app.root.ids[f"weapon_{weapon}"].state = "down"

if __name__ == "__main__":
    gpio_in = open("./gpio_in", "rb")
    Clock.schedule_interval(get_data, read_interval)
    KivyApp().run()
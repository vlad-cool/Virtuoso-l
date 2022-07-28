#!/usr/bin/env python3 
from re import L
import kivy
from kivy.core.window import Window
from kivy.app import App
from kivy.lang import Builder
import pickle
import os

global app_config #app config dictionaty

with (open("config", "rb")) as f: #loading app config
    try:
        app_config = pickle.load(f)
    except:
        app_config = {"weapon" : 1} #default config dictionary

weapon = app_config["weapon"]

if os.name != "nt": #for bananapi, it have much better performance when running vertically
    Window.rotation = 270

kivy.require('2.1.0')  

class KivyApp(App):
    def build(self):
        return Builder.load_file('main.kv')

    def on_start(self, **kwargs):
        print("Loaded")
        app = App.get_running_app()
        print(app.root.ids)
        app.root.ids[f"weapon_{weapon}"].state = "down"

if __name__ == "__main__":
    KivyApp().run()

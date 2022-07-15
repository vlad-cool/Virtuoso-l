import kivy
#from kivy.core.window import Window
from kivy.app import App
from kivy.config import Config
from kivy.lang import Builder

#print(kivy.__version__)

Config.set('graphics', 'width', '1920')
Config.set('graphics', 'height', '480')
Config.set('graphics', 'borderless', '0')

Config.set('kivy', 'exit_on_escape', '1') #Debug
#Config.set('kivy', 'show_fps', '1')

Config.set('modules', 'monitor', '')

Config.write()

kivy.require('2.1.0')  

#kivy.core.window.WindowBase(borderless="1")

class KivyApp(App):
    def build(self):
        return Builder.load_file('main.kv')


if __name__ == "__main__":
    KivyApp().run()
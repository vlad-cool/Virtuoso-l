#!V24m/venv/bin/python3
from kivy.config import Config
Config.set('graphics', 'width', '1920')
Config.set('graphics', 'height', '480')
Config.set('graphics', 'fullscreen', '1')
Config.set('graphics', 'multisamples', '0')
Config.set('kivy', 'exit_on_escape', '1')
Config.set('input', 'device_%(name)s', 'probesysfs,provider=mtdev,param=rotation=90,param=invert_x=1')
Config.set('input', 'mouse', '')
Config.set('input', '%(name)s', '')
Config.write()

#!V24m/venv/bin/python3
from kivy.config import Config
Config.set('graphics', 'width', '480')
Config.set('graphics', 'height', '1920')
Config.set('graphics', 'multisamples', '0')
Config.set('kivy', 'exit_on_escape', '1')
Config.set('modules', 'monitor', '')
Config.set('input', 'device_%(name)s', 'probesysfs,provider=mtdev')
Config.write()

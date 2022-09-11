from threading import Thread
from ctypes    import *
from time      import sleep
from platform  import machine

if machine() != "armv7l":
    exit()

off_time = 250
on_time  = 250

gpio = cdll.LoadLibrary("wiringPi.h")

gpio.wiringPiSetupPhys();

gpio.pinMode(8, 1)
gpio.pinMode(10, 1)
gpio.pinMode(12, 1)
gpio.pinMode(16, 1)
gpio.pinMode(19, 1)
gpio.pinMode(21, 1)
gpio.pinMode(23, 1)
gpio.pinMode(24, 1)

gpio.digitalWrite(8, 1)
gpio.digitalWrite(10, 1)
gpio.digitalWrite(12, 1)
gpio.digitalWrite(16, 1)
gpio.digitalWrite(19, 1)
gpio.digitalWrite(21, 1)
gpio.digitalWrite(23, 1)
gpio.digitalWrite(24, 1)

gpio.pinMode(18, 0)

gpio.pinMode(26, 1)
gpio.pinMode(37, 1)

gpio.digitalWrite(26, 1)
gpio.digitalWrite(37, 1)

gpio.pinMode(32, 0)
gpio.pinMode(36, 0)
gpio.pinMode(7, 0)

def run_in_thread(func):
    def inner1(*args):
        thread = Thread(target=func, args=args)
        thread.start()
    return inner1

@run_in_thread
def button_emu(pin, times):
    for i in range(times):
        gpio.digitalWrite(pin, 0)
        sleep(off_time / 1000)
        gpio.digitalWrite(pin, 1)
        sleep(on_time  / 100)

@run_in_thread
def ir_emu(code):
    pass

def read_pin(pin):
    return gpio.digitalRead(pin)

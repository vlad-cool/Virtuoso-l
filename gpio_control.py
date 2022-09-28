from threading import Thread
from ctypes    import *
from time      import sleep, time_ns
from platform  import machine

off_time = 250
on_time  = 250
TIMING = 889

if machine() == "armv7l":
    gpio = cdll.LoadLibrary("/usr/lib/libwiringPiDev.so")

    gpio.wiringPiSetupPhys()

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
    def inner1(*args, **kwargs):
        thread = Thread(target=func, args=args, kwargs=kwargs)
        thread.start()
    return inner1

def ir_emu_blocking(to_transmit, toggle_bit, pin=26):
    print(to_transmit, pin)
    data = [0] * 14
    to_transmit += 12288
    to_transmit += toggle_bit * 2048

    for i in range(14):
        data[13 - i] = to_transmit % 2
        to_transmit //= 2

    print(data)

    for i in range(14):
        gpio.digitalWrite(pin, 0 + data[i])
        
        t = time_ns() // 1000

        while time_ns() // 1000 - t < TIMING:
            pass
        
        t = time_ns() // 1000

        gpio.digitalWrite(pin, 1 - data[i])

        while time_ns() // 1000 - t < TIMING:
            pass

    gpio.digitalWrite(pin, 1)

@run_in_thread
def button_emu(pin, times):
    for i in range(times):
        gpio.digitalWrite(pin, 0)
        sleep(off_time / 1000)
        gpio.digitalWrite(pin, 1)
        sleep(on_time  / 1000)

@run_in_thread
def ir_emu(to_transmit, toggle_bit, pin=26):
    ir_emu_blocking(to_transmit, toggle_bit, pin)

def read_pin(pin):
    return gpio.digitalRead(pin)

def get_address():
    spacing_time = .3
    data = []
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
    val = data[5][7]
    timer = data[3][3]
    if timer:
        command = 13 #timer start stop
    elif val:
        command = 3  #left -
    else:
        command = 2  #left +


    for k in range(32):
        ir_emu_blocking(k * 2**6 + command)
        sleep(spacing_time + timer)
        for j in range(9):
            data[j] = [0] * 8
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
        if (val != data[5][7] or timer != data[3][3]):
            if timer:
                ir_emu_blocking(k * 2**6 + command)
            else:
                ir_emu_blocking(k * 2**6 + 5 - command)

            return k
        print(k, data)
    return -1

from multiprocessing import Process
from ctypes          import *
from time            import sleep, time_ns
from platform        import machine

off_time = 250
on_time  = 250
TIMING = 889

button_emulating = []
it_send_queue    = []
it_get_queue     = []
ir_emulating     = 0
ir_toggle_bit    = 0

if machine() == "armv7l":
    gpio = CDLL("/usr/lib/libwiringPi.so", mode=1)

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

    gpio.pinMode(15, 1)
    gpio.digitalWrite(15, 0)

def run_in_thread(func):
    def inner1(*args, **kwargs):
        thread = Process(target=func, args=args, kwargs=kwargs)
        thread.start()
    return inner1

def ir_emu_blocking(to_transmit, pin=26):
    global ir_toggle_bit
    ir_toggle_bit = 1 - ir_toggle_bit
    print(to_transmit, pin)
    data = [0] * 14
    to_transmit += 12288
    to_transmit += ir_toggle_bit * 2048

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
    global button_emulating
    while pin in button_emulating:
        sleep(.3)
    button_emulating.append(pin)
    for i in range(times):
        gpio.digitalWrite(pin, 0)
        sleep(off_time / 1000)
        gpio.digitalWrite(pin, 1)
        sleep(on_time  / 1000)
    button_emulating.remove(pin)

@run_in_thread
def ir_emu_inner(to_transmit):
    global ir_emulating
    while ir_emulating == 1:
        sleep(.3)
    ir_emulating = 1
    ir_emu_blocking(to_transmit, 26)
    ir_emulating = 0

def ir_emu(address, to_transmit):
    ir_emu_inner(address * (2**6) + to_transmit)

def read_pin(pin):
    return gpio.digitalRead(pin)

def byte_to_arr(byte):
    a = [0] * 8
    for i in range(8):
            a[i] = byte % 2
            byte //= 2
    return a[::-1]

def get_address(data_rx):
    spacing_time = 1.3
    toggle_bit = 0

    button_emu(37, 3)
    sleep(2)

    data = [[0] * 8] * 8
    while data_rx.inWaiting() // 8 > 0:
        data = [[0] * 8] * 8
        for i in range(8):
            byte = int.from_bytes(data_rx.read(), "big")
            data[byte // 2 ** 5] = byte_to_arr(byte)
    
    val = data[4][7]
    timer = data[2][3]
    if timer:
        command = 13 #timer start stop
    elif val:
        command = 3  #left -
    else:
        command = 2  #left +

    for k in range(32):
        ir_emu_blocking(k * 2**6 + command)
        toggle_bit = 1 - toggle_bit
        sleep(spacing_time + timer)
        
        while data_rx.inWaiting() // 8 > 0:
            for i in range(8):
                byte = int.from_bytes(data_rx.read(), "big")
                data[byte // 2 ** 5] = byte_to_arr(byte)
        
        print(str(data).replace(']', ']\n'))
        if (val != data[4][7] or timer != data[2][3]):
            if timer:
                ir_emu_blocking(k * 2**6 + command)
            else:
                ir_emu_blocking(k * 2**6 + 5 - command)

            return k
    return -1

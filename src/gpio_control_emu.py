from time import sleep
from ast import literal_eval

off_time = 250
on_time  = 250
TIMING = 889

ir_commands = []

button_emulating = []

def setup():
    print("Setted up!")

def toggle(pin):
    print(f"toggled pin {pin}\n")

def set(pin, value):
    print(f"setted pin {pin} value to {value}\n")

def button_emu(pin, times):
    for _ in range(times):
        print(f"button {pin} pressed\n")

def ir_emu(address, command):
    print(f"transmitted signal, address: {address}, command: {command}\n")

def ir_emu_blocking(address, command):
    print(f"transmitted blocking signal, address: {address}, command: {command}\n")

def read_pins():
    return literal_eval("{3: 1, 7: 0, 18: 0, 27: 1, 32: 0, 36: 1, }\n")

def read_rc5(_):
    return []

def read_all_rc5():
    return []

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
        for _ in range(8):
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
        ir_emu_blocking(k, command)
        toggle_bit = 1 - toggle_bit
        sleep(spacing_time + timer)

        while data_rx.inWaiting() // 8 > 0:
            for _ in range(8):
                byte = int.from_bytes(data_rx.read(), "big")
                data[byte // 2 ** 5] = byte_to_arr(byte)

        print(str(data).replace(']', ']\n'))
        if (val != data[4][7] or timer != data[2][3]):
            if timer:
                ir_emu_blocking(k, command)
            else:
                ir_emu_blocking(k, 5 - command)

            return k
    return -1

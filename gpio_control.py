from time import sleep
from ast import literal_eval
import subprocess

off_time = 250
on_time  = 250

button_emulating = []

send_pin_proc = subprocess.Popen("./send_pin", bufsize=0, text=True, stdin=subprocess.PIPE)
send_rc5_proc = subprocess.Popen("./send_rc5", bufsize=0, text=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE)
get_pin_proc = subprocess.Popen("./get_pin", bufsize=0, text=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE)
get_rc5_proc = subprocess.Popen("./get_rc5", bufsize=0, text=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE)

def static_vars(**kwargs):
    def decorate(func):
        for k in kwargs:
            setattr(func, k, kwargs[k])
        return func
    return decorate

def toggle(pin):
    send_pin_proc.stdin.write(f"toggle {pin}\n")

def set(pin, value):
    send_pin_proc.stdin.write(f"set {pin} {value}\n")

def button_emu(pin, times):
    for _ in range(times):
        send_pin_proc.stdin.write(f"button {pin}\n")

def ir_emu(address, command):
    send_rc5_proc.stdin.write(f"transmit {address} {command}\n")

def ir_emu_blocking(address, command):
    send_rc5_proc.stdin.write(f"transmit {address} {command}\nping\n")
    send_rc5_proc.stdout.readline()

def read_pins():
    get_pin_proc.stdin.write("get\n")
    return literal_eval(get_pin_proc.stdout.readline())

@static_vars(toggle=-1)
def read_rc5(address):
    get_rc5_proc.stdin.write("get\n")
    ir_commands = []
    raw_rc5 = get_rc5_proc.stdout.readline()
    while raw_rc5 != "end\n":
        raw_rc5 = list(map(int, raw_rc5.split()[::2]))
        new = raw_rc5[2] != read_rc5.toggle
        read_rc5.toggle = raw_rc5[2]
        addr = 0
        cmd = 0
        for bit in raw_rc5[3:8]:
            addr *= 2
            addr += bit
        for bit in raw_rc5[8:14]:
            cmd *= 2
            cmd += bit

        if addr == address:
            ir_commands.append((addr, cmd, new))
        raw_rc5 = get_rc5_proc.stdout.readline()
    return ir_commands

@static_vars(toggle=-1)
def read_all_rc5():
    get_rc5_proc.stdin.write("get\n")
    ir_commands = []
    raw_rc5 = get_rc5_proc.stdout.readline()
    while raw_rc5 != "end\n":
        raw_rc5 = list(map(int, raw_rc5.split()[::2]))
        new = raw_rc5[2] != read_all_rc5.toggle
        read_all_rc5.toggle = raw_rc5[2]
        addr = 0
        cmd = 0
        for bit in raw_rc5[3:8]:
            addr *= 2
            addr += bit
        for bit in raw_rc5[8:14]:
            cmd *= 2
            cmd += bit

        ir_commands.append((addr, cmd, new))
        raw_rc5 = get_rc5_proc.stdout.readline()
    return ir_commands

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

def update_addr(data_rx, address):
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

    if address == -1:
        address = 0

    for k in [address] + list(range(32)):
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
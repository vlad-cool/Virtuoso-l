from time import sleep
from ast import literal_eval
import subprocess
import select

off_time = 250
on_time  = 250
TIMING = 889

ir_commands = []

button_emulating = []

send_pin_proc = subprocess.Popen("./send_pin", bufsize=1, text=True, stdin=subprocess.PIPE)
send_rc5_proc = subprocess.Popen("./send_rc5", bufsize=1, text=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE)
get_pin_proc = subprocess.Popen("./get_pin", bufsize=1, text=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE)
get_rc5_proc = subprocess.Popen("./get_rc5", bufsize=1, text=True, stdout=subprocess.PIPE)

def toggle(pin):
    send_pin_proc.stdin.write(f"toggle {pin}\n")

def toggle(pin, value):
    send_pin_proc.stdin.write(f"set {pin} {value}\n")

def button_emu(pin, times):
    for _ in range(times):
        send_pin_proc.stdin.write(f"button {pin}\n")

def ir_emu(address, command):
    send_rc5_proc.stdin.write(f"transmit {address} {command}\n")

def ir_emu_blocking(address, command):
    send_rc5_proc.stdin.write(f"transmit {address} {command}\n")
    send_rc5_proc.stdin.write(f"ping\n")
    send_rc5_proc.stdout.readline()

def read_pins():
    get_pin_proc.stdin.write("get\n")
    return literal_eval(get_pin_proc.stdout.readline())

def read_rc5():
    global ir_commands
    a = select.select([get_rc5_proc.stdout], [], [], 0)[0]
    ir_commands += a

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

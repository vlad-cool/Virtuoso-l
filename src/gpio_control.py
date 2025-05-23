from ast import literal_eval
from static_vars import static_vars
import subprocess
import system_info

send_pin_proc = None
send_rc5_proc = None
get_pin_proc = None
get_rc5_proc = None

def setup():
    global send_pin_proc
    global send_rc5_proc
    global get_pin_proc
    global get_rc5_proc

    send_pin_proc = subprocess.Popen("./send_pin", bufsize=0, text=True, stdin=subprocess.PIPE)
    send_rc5_proc = subprocess.Popen("./send_rc5", bufsize=0, text=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE)
    get_pin_proc = subprocess.Popen("./get_pin", bufsize=0, text=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE)
    get_rc5_proc = subprocess.Popen("./get_rc5", bufsize=0, text=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE)

    get_pin_proc.stdin.write("add_pin 7\n")
    get_pin_proc.stdin.write("add_pin 27\n")
    get_pin_proc.stdin.write("add_pin 32\n")
    get_pin_proc.stdin.write("add_pin 36\n")

    send_pin_proc.stdin.write("add_pin 5 0\n")
    send_pin_proc.stdin.write("add_pin 15 0\n")
    send_pin_proc.stdin.write("add_pin 29 0\n")
    send_pin_proc.stdin.write("add_pin 35 0\n")
    send_pin_proc.stdin.write("add_pin 31 0\n")
    send_pin_proc.stdin.write("add_pin 38 0\n")

    if system_info.input_support:
        send_pin_proc.stdin.write("add_pin 37 1\n")
    else:
        get_pin_proc.stdin.write("add_pin 37\n")

    send_pin_proc.stdin.write("setup\n")


def set(pin, value):
    send_pin_proc.stdin.write(f"set {pin} {value}\n")


def button_emu(pin, times):
    for _ in range(times):
        send_pin_proc.stdin.write(f"button {pin}\n")


def ir_emu(address, command):
    send_rc5_proc.stdin.write(f"transmit {address} {command}\n")


def read_pins():
    get_pin_proc.stdin.write("get\n")
    return literal_eval(get_pin_proc.stdout.readline())


@static_vars(toggle=-1)
def read_rc5():
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

        ir_commands.append((addr, cmd, new))
        raw_rc5 = get_rc5_proc.stdout.readline()
    return ir_commands

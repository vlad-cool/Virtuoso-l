from time import sleep
from ast import literal_eval
from static_vars import static_vars
from flask import Flask, render_template
import json
import threading
import system_info

app = Flask(__name__)

import logging
log = logging.getLogger('werkzeug')
log.setLevel(logging.ERROR)

input_pins = {}
output_pins = {}
input_rc5 = []
output_rc5 = []

lock = threading.Lock()

@app.route("/")
def index():
    global output_pins
    with lock:
        return render_template('index.html', input_pins=input_pins, output_pins=output_pins)

@app.route("/output_pins")
def get_output_pins():
    global output_pins
    with lock:
        return json.dumps(output_pins)

@app.route("/toggle_pin/<pin>")
def set_pin(pin):
    global input_pins
    with lock:
        input_pins[int(pin)] = 1 - input_pins[int(pin)]
    
    return json.dumps({"status": "ok"})

ir_commands = []

button_emulating = []

def run_emulator():
    global app
    app.run(host="0.0.0.0", port=1234)


def setup():
    global output_pins
    

    input_pins[7] = 0
    input_pins[27] = 0
    input_pins[32] = 0
    input_pins[36] = 0
    input_pins[18] = 0

    with lock:
        output_pins[5] = 0
        output_pins[15] = 0
        output_pins[29] = 0
        output_pins[35] = 0

        if system_info.input_support:
            output_pins[37] = 1
        else:
            input_pins[37] =  0

    flask_thread = threading.Thread(target=run_emulator)
    flask_thread.start()
    
    print("Setted up!")
    

def set(pin, value):
    global input_pins
    with lock:
        input_pins[pin] = value
    print(f"Setted pin {pin} to value {value}")


def button_emu(pin, times):
    for _ in range(times):
        print(f"Pressed button on pin {pin} {times} times")


def ir_emu(address, command):
    global output_rc5
    with lock:
        output_rc5.append((address, command))
    print(f"Pressed {command} button on remote with address {address}")


def read_pins():
    global input_pins
    with lock:
        ret_val = input_pins.copy()

    return ret_val


def read_rc5():
    global input_rc5
    with lock:
        ret_val = input_rc5.copy()

    return ret_val

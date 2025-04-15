#!/bin/python3
import jinja2
import copy
import json
import sys

def to_hex_color(data):
    alphabet = "0123456789abcdef"
    for key in data:
        if isinstance(data[key], list):
            data[key] = f"#{alphabet[data[key][0] // 16]}{alphabet[data[key][0] % 16]}{alphabet[data[key][1] // 16]}{alphabet[data[key][1] % 16]}{alphabet[data[key][2] // 16]}{alphabet[data[key][2] % 16]}ff"
        if isinstance(data[key], str):
            data[key] = f"\"{data[key]}\""
            data[key] = data[key].replace("#", "")
        if isinstance(data[key], dict):
            data[key] = to_hex_color(data[key])
    return data

with open(sys.argv[1]) as f:
    colors = json.load(f)
    
    for key in colors.keys():
        if isinstance(colors[key], str):
            print(f"    out property <brush> {key}: {colors[key]};")
        else:
            for key_2 in colors[key].keys():
                print(f"    out property <brush> {key}_{key_2}: {colors[key][key_2]};")

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

def resize(data):
    for key in data:
        if isinstance(data[key], int):
            data[key] = data[key] // 2
        if isinstance(data[key], dict):
            data[key] = resize(data[key])
    return data

def hex_to_rgba(color):
    return [int(color[i:i+2], 16) / 255 for i in range(1, 8, 2)]

def darker(color):
    # new_color = color.copy()
    new_color = color
    new_color = new_color[:-3] + "33\""
    return new_color

if len(sys.argv) <= 1:
    print("No input file")
    exit(-1)

with open(sys.argv[1]) as f:
    data = json.load(f)
    
if len(sys.argv) > 2:
    with open(sys.argv[2]) as f:
        gen_dict = json.load(f)
        
        for key in gen_dict.keys():
            if "y" in gen_dict[key]:
                pass
                print(gen_dict[key]["y"], end=" -> ")
                gen_dict[key]["y"] = gen_dict["background"]["height"] - gen_dict[key]["y"] - gen_dict[key]["height"]
                print(gen_dict[key]["y"])
        
        data.update(gen_dict)
        
print(data)

resized_data = resize(copy.deepcopy(data))

with open("colors.json") as f:
    colors = to_hex_color(json.load(f))

print(colors)

environment = jinja2.Environment(loader=jinja2.FileSystemLoader("."))
template = environment.get_template("template_main.kv")

with open("output.kv", "w") as f:
    f.write(template.render(data=data, resized_data=resized_data, colors=colors, darker=darker, hex_to_rgba=hex_to_rgba))

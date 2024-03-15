#!/bin/python3
import jinja2
import json
import sys

if len(sys.argv) <= 1:
    print("No input file")
    exit(-1)

with open(sys.argv[1]) as f:
    data = json.load(f)

environment = jinja2.Environment(loader=jinja2.FileSystemLoader("."))
template = environment.get_template("template_main.kv")

with open("output.kv", "w") as f:
    f.write(template.render(data=data))

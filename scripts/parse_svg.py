#!env python3
from lxml import etree
from sys import argv
import json
import re

tree = etree.parse(argv[1])
root = tree.getroot()

elements = {}

class Element:
    def __init__(self, attrib, parent):
        
        name = attrib.get("{https://penpot.app/xmlns}name", None)
        type = attrib.get("{https://penpot.app/xmlns}type", None)
        
        self.name = name
        self.type = type
        
        match type:
            case "text":
                x = attrib.get("{https://penpot.app/xmlns}x", -10)
                y = attrib.get("{https://penpot.app/xmlns}y", -10)
                width = attrib.get("{https://penpot.app/xmlns}width", -10)
                height = attrib.get("{https://penpot.app/xmlns}height", -10)
                content = attrib.get("{https://penpot.app/xmlns}content", None)
                
                
                self.x = int(float(x) + 0.5)
                self.y = int(float(y) + 0.5)
                self.width = int(float(width) + 0.5)
                self.height = int(float(height) + 0.5)
                
                s = content.replace("&quot;", "")
                
                pattern = r'"fontSize":"(\d+)"'
                match = re.search(pattern, s)
                
                self.font_size = int(match.group(1))

            case "rect":
                for child in parent:
                    if child.tag == r"{http://www.w3.org/2000/svg}g":
                        for child1 in child:
                            if child1.tag == r"{http://www.w3.org/2000/svg}rect":
                                attrib = child1.attrib
                                x = attrib.get("x", -10)
                                y = attrib.get("y", -10)
                                rx = attrib.get("rx", -10)
                                ry = attrib.get("ry", -10)
                                width = attrib.get("width", -10)
                                height = attrib.get("height", -10)

                self.x = int(float(x) + 0.5)
                self.y = int(float(y) + 0.5)
                self.rx = int(float(rx) + 0.5)
                self.ry = int(float(ry) + 0.5)
                self.width = int(float(width) + 0.5)
                self.height = int(float(height) + 0.5)

def parse_layout(parent, offset=0):
    for child in parent:
        if child.tag == r"{https://penpot.app/xmlns}shape":
            elements[child.attrib["{https://penpot.app/xmlns}name"]] = Element(
                child.attrib,
                parent
            )
        parse_layout(child, offset=offset + 4)


parse_layout(root)

layout = {}

for key, element in elements.items():
    match element.type:
        case "text":
            print(f"    {element.name}: TextProperties,")
        case "rect":
            print(f"    {element.name}: RectangleProperties,")
        
    layout[element.name] = vars(element)

print()
print("--------------------------------------------")
print()

for key, element in elements.items():
    # if element.type == "text":
    #     print(
    #         element.name,
    #         element.type,
    #         element.x,
    #         element.y,
    #         element.widht,
    #         element.height
    #     )
    # if element.type == "rect":
    #     print(
    #         element.name,
    #         element.type,
    #         element.x,
    #         element.y,
    #         element.rx,
    #         element.ry,
    #         element.widht,
    #         element.height
    #     )
    match element.type:
        case "text":
            print(
                f"""{element.name}: TextProperties{{
                    x: {element.x},
                    y: {element.y},
                    width: {element.width},
                    height: {element.height},
                    font_size: {element.font_size},
                }},"""
            )
        case "rect":
            print(
                f"""{element.name}: RectangleProperties{{
                    x: {element.x},
                    y: {element.y},
                    width: {element.width},
                    height: {element.height},
                    radius: {element.rx},
                }},"""
            )

if len(argv) > 2:
    with open(argv[2], "w") as f:
        json.dump(layout, f)

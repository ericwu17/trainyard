# This script reads some levels in the old format, and outputs json in the new format.

import json


def blank_level(name):
    res = {}
    res["name"] = name
    res["sources"] = []
    res["sinks"] = []
    return res

def read_position(position):
    x, y = position.split(",")
    x = int(x)
    y = int(y)
    return [6 - y, x]
def add_rock(curr_level, position):
    if "rocks" in curr_level.keys():
        curr_level["rocks"].append(position)
    else:
        curr_level["rocks"] = [position]


curr_city = ""
curr_level = {}
levels = []

with open("levels.txt") as f:
    for line in f.readlines():
        if line == "// END HERE\n":
            break
        if line.startswith("//") or line.strip() == "":
            continue
        
        line = line.rstrip("\n")

        if line.startswith("CITY:"):
            curr_city = line.lstrip("CITY:")
        
        elif ":" in line:
            levels.append(curr_level)
            level_name = list(line.split(":"))[0]
            curr_level = blank_level(level_name)
        elif line.startswith("+"):
            _, position, colors, direction = line.split(" ")
            position = read_position(position)
            colors = list(map(lambda x: x.capitalize(), list(colors.split(","))))
            direction = direction.capitalize()

            curr_level["sources"].append([colors, direction, position])
        elif line.startswith("o"):
            _, position, colors, directions = line.split(" ")
            position = read_position(position)
            colors = list(map(lambda x: x.capitalize(), list(colors.split(","))))
            directions = list(map(lambda x: x.capitalize(), list(directions.split(","))))
            curr_level["sinks"].append([colors, directions, position])
        elif line.startswith("* "):
            for rock in line.lstrip("* ").split(" "):
                position = read_position(rock)
                add_rock(curr_level, position)

    levels.append(curr_level)

levels = [level for level in levels if "name" in level.keys()]

print(json.dumps(levels))

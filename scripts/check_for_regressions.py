#!/usr/bin/python3

import sys
import json

filename = sys.argv[1]

with open(filename) as file:
    for line in file:
        if line.startswith('{'):
            j = json.loads(line)
            if j["reason"] != "benchmark-complete":
                continue
            if j["change"]["change"] == "Regressed":
                print(f"::error title=Regression::'{j['id']}' has regressed")

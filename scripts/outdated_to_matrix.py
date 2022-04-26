import json

f = open('outdated.json')
o = json.load(f)
print(f"::set-output name=matrix::{o['dependencies']}")
f.close()
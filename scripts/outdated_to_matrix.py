import json

f = open('outdated.json')
o = json.load(f)
print(o['dependencies'])
f.close()
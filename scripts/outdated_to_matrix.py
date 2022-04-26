import json

f = open('outdated.json')
o = json.load(f)
d = o['dependencies']

print(f"::set-output name=matrix::{[{'name': x['name'], 'project': x['project'], 'latest': x['latest']} for x in d]}")
f.close()
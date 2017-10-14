import json
import os

with open(os.path.join(os.path.dirname(__file__), "lcids.json")) as f:
    _data = json.load(f)

def get(tag: str) -> int:
    return _data.get(tag, None)

def get_hex8(tag: str) -> str:
    x = _data.get(tag, None)
    if x is not None:
        return "{:08x}".format(x)

def get_hex4(tag: str) -> str:
    x = _data.get(tag, None)
    if x is not None:
        return "{:04x}".format(x)

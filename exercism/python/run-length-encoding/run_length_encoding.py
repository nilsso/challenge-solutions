import re

def decode(string):
    return re.sub(r"(\d+)(\D)", lambda m: int(m.group(1)) * m.group(2), string)

def encode(string):
    return re.sub(r"((.)\2+)", lambda m: "{}{}".format(len(m.group(1)), m.group(2)), string)


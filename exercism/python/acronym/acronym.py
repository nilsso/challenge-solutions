#!/usr/bin/env python

import re

def abbreviate(words):
    return "".join(w for w in re.findall("(\w)\w*", words)).upper()

if __name__ == "__main__":
    from sys import argv
    print(abbreviate(argv[1]))


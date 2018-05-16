#!/usr/bin/env python
import io
import re

short_scale = [
        "",
        " thousand ",
        " million ",
        " billion ",
        " trillion "
        ]

cardinals = {
        0:"zero", 1:"one", 2:"two", 3:"three", 4:"four", 5:"five", 6:"six",
        7:"seven", 8:"eight", 9:"nine", 10:"ten", 11:"eleven", 12:"twelve",
        13:"thirteen", 14:"fourteen", 15:"fifteen", 16:"sixteen",
        17:"seventeen", 18:"eighteen", 19:"nineteen", 20:"twenty", 30:"thirty",
        40:"forty", 50:"fifty", 60:"sixty", 70:"seventy", 80:"eighty",
        90:"ninety" }

def cardinal(n):
    if n in cardinals:
        return cardinals[n]
    t, o = divmod(n, 10)
    return "{}-{}".format(cardinals[t*10], cardinals[o])

def hundred(n):
    h, t = divmod(n, 100)
    return\
            "{} hundred and {}".format(cardinal(h), cardinal(t)) if (h and t) else\
            "{} hundred".format(cardinal(h)) if h else\
            "{}".format(cardinal(t))

def say(number):
    if number < 0 or number >= 1e12:
        raise ValueError(".+")
    if number < 100:
        return cardinal(number)
    buff = io.StringIO()
    def gen(n):
        while n > 0:
            n, r = divmod(n, 1000)
            yield r
    n = list(gen(number))
    for i, v in reversed(list(enumerate(n))):
        if v:
            buff.write(hundred(v) + short_scale[i])
    english_number = buff.getvalue().strip()
    m = re.search(r"( [a-z-]+ )([a-z-]+)$", english_number)
    if m and m.group(1) in short_scale:
        english_number = re.sub("([a-z-]+)$", r"and \1", english_number)
    return english_number

if __name__ == "__main__":
    from sys import argv
    print(say(int(argv[1])))

#!/usr/bin/env python

numerals = [
        (1000, "M"),
        (900, "CM"), (500, "D"), (400, "CD"), (100, "C"),
        (90,  "XC"), (50,  "L"), (40,  "XL"), (10,  "X"),
        (9,   "IX"), (5,   "V"), (4,   "IV"), (1,   "I")]


def int_to_numeral(n):
    s = ""
    for v, r in numerals:
        while n >= v:
            n -= v
            s += r
    return s


def numeral_to_int(n):
    return "?"


if __name__ == "__main__":
    from sys import argv
    if len(argv) > 1:
        n = argv[1]
        try:
            n = int(n)
            print(int_to_numeral(n))
        except:
            print(numeral_to_int(n))

numeral = int_to_numeral

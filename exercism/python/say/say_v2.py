import re

short_scale = [ " thousand ", " million ", " billion " ]

cardinals = {
        1:"one", 2:"two", 3:"three", 4:"four", 5:"five", 6:"six", 7:"seven",
        8:"eight", 9:"nine", 10:"ten", 11:"eleven", 12:"twelve", 13:"thirteen",
        14:"fourteen", 15:"fifteen", 16:"sixteen", 17:"seventeen",
        18:"eighteen", 19:"nineteen", 20:"twenty", 30:"thirty", 40:"forty",
        50:"fifty", 60:"sixty", 70:"seventy", 80:"eighty", 90:"ninety" }

def cardinal(n):
    if n in cardinals:
        return cardinals[n]
    t, o = divmod(n, 10)
    return "{}-{}".format(cardinals[t*10], cardinals[o])

def cardinal_group(n):
    h, t = divmod(n, 100)
    return \
            cardinal(h) + " hundred and " + cardinal(t) if h and t else\
            cardinal(h) + " hundred" if h else\
            cardinal(t)

def num_to_english_gen(n):
    n, r = divmod(n, 1000)
    if r:
        yield ("and " if r < 100 else "") + cardinal_group(r)
    i = 0
    while n:
        n, r = divmod(n, 1000)
        if r:
            yield cardinal_group(r) + short_scale[i]
        i += 1

def say(number):
    if number < 0 or number >= 1e12:
        raise ValueError(".+")
    if number == 0:
        return "zero"
    if number < 100:
        return cardinal(number)
    return "".join(reversed(list(num_to_english_gen(number)))).strip()

if __name__ == "__main__":
    from sys import argv
    print(say(int(argv[1])))

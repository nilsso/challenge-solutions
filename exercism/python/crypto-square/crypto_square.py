from math import ceil, sqrt
import re


def encode(plain_text):
    plain_text = re.sub("\W", "", plain_text.lower())
    n = sqrt(len(plain_text))
    c = ceil(n)
    plain_text = plain_text.ljust(c * round(n), " ")
    return " ".join(plain_text[i::c] for i in range(c))

from math import ceil, sqrt
from itertools import chain
from functools import lru_cache


P, A, D = "perfect", "abundant", "deficient"


@lru_cache(maxsize=None)
def factors(n):
    f = set([1])
    for d in range(2, ceil(sqrt(n+1))):
        if n % d == 0:
            f.add(d)
            f.add(n // d)
    return f


def is_perfect(n):
    return n == sum(factors(n))


def classify(n):
    if n < 1:
        raise ValueError("Invalid number")
    elif n == 1:
        return D
    ratio = sum(factors(n)) / n
    return P if ratio == 1 else A if ratio > 1 else D

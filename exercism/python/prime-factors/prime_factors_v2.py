from math import floor, sqrt
from functools import lru_cache


@lru_cache(maxsize=None)
def prime_factor_helper(n):
    for d in range(2, floor(sqrt(n))+1):
        if n % d == 0:
            return d
    return n


@lru_cache(maxsize=None)
def prime_factors(n):
    factors = []
    while n > 1:
        d = prime_factor_helper(n)
        factors.append(d)
        n //= d
    return factors

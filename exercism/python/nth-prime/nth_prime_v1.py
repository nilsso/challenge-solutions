from math import floor, sqrt
from functools import lru_cache


@lru_cache(maxsize=None)
def is_prime(n):
    for d in range(2, floor(sqrt(n))+1):
        if n % d == 0:
            return False
    return True


@lru_cache(maxsize=None)
def nth_prime(n):
    if n <= 0:
        raise ValueError("Term must be positive")
    i, count, prime = 2, 0, None
    while count < n:
        if is_prime(i):
        # if len(prime_factors(i)) == 1:
            prime = i
            count += 1
        i += 1
    return prime

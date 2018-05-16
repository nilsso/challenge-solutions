from math import sqrt


def prime_factors(n):
    factors = []
    d = 2
    h = int(sqrt(n)+1)
    while n > 1:
        while n % d != 0:
            d += 1
            if d == h:
                d = n
        factors.append(d)
        n //= d
    return factors

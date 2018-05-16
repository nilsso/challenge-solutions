def prime_factors(n):
    factors = []
    while n > 1:
        d = 2
        while n % d != 0:
            d += 1
        factors.append(d)
        n //= d
    return factors

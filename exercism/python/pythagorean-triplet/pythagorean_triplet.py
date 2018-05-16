from math import gcd, sqrt, floor


def coprime(m, n):
    return gcd(m, n) == 1


def coprime_factor_generator(mn):
    for n in range(1, floor(sqrt(mn)) + 1):
        m = mn/n
        if m.is_integer():
            m = int(m)
            if coprime(m, n):
                yield (m, n)


def triplet(a, b, c):
    return tuple(sorted((a, b, c)))


def primitive_triplets(a):
    if a % 4 != 0:
        raise ValueError("Argument not divisible by 4")
    return set(
            triplet(a, m*m - n*n, m*m + n*n)
            for m, n in coprime_factor_generator(a // 2))


def triplets_in_range(start, end):
    triplets = set()
    for a in range(start, end):
        for b in range(a+1, end):
            c = sqrt(a*a + b*b)
            if c <= end and c.is_integer():
                triplets.add(triplet(a, b, int(c)))
    return triplets


def is_triplet(triplet):
    a, b, c = sorted(triplet)
    return a*a + b*b == c*c

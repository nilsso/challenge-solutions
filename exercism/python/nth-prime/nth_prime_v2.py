def sieve(limit):
    marker = [0, 0] + [1] * (limit + 1)
    primes = []
    for p in range(limit + 1):
        if not marker[p]:
            continue
        primes.append(p)
        for i in range(p * 2, limit + 1, p):
            marker[i] = 0
    return primes


n = 1000000
primes = sieve(n)


def nth_prime(n):
    if n <= 0:
        raise ValueError("Term must be positive")
    return primes[n-1]

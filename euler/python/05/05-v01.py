# Number 5
# Prompt: What is the smallest positive number that is evenly divisible by all
# of the numbers from 1 to 20?


from math import ceil, floor, sqrt, log
from functools import reduce
from operator import mul


def sieve(lim):
    return sorted(
            set(range(2, lim+1)) -
            set(n for i in range(2, ceil(sqrt(lim+1)))
                for n in range(i*2, lim+1, i)))


def greatestPower(n, lim):
    return n**floor(log(lim, n))


# Now with prime factorization
def smallestMultiple(lim):
    return reduce(mul, (map(lambda p: greatestPower(p, lim), sieve(lim))))


if __name__ == "__main__":
    from sys import argv
    print(smallestMultiple(int(argv[1]) if len(argv) > 1 else 20))

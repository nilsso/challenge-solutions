# Number 6
# Promt: Find the difference between the sum of the squares of the first one
# hundred natural numbers and the square of the sum.


def sumOfInts(lim):
    return sum(range(lim+1))


def sumOfSquares(lim):
    return sum(map(lambda x: x**2, range(lim+1)))


def e06(lim):
    return sumOfInts(lim)**2 - sumOfSquares(lim)


if __name__ == "__main__":
    from sys import argv
    lim = int(argv[1]) if len(argv) > 1 else 100
    print(e06(lim))

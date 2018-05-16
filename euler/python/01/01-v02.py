# Number 1
# Prompt: Find the sum of all the multiples of 3 or 5 below 1000.


from math import ceil


# Sum of multiples
# @param m Number to take multiples of
# @param lim Limit of largest multiple
def som(m, lim):
    return m * sum(range(ceil(lim / m)))


if __name__ == "__main__":
    from sys import argv
    n = int(argv[1]) if len(argv) > 1 else 1000
    print(som(3, n) + som(5, n) - som(15, n))

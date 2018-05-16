from math import ceil, sqrt

def sieve(limit):
    return sorted(
            set(range(2, limit+1)) -
            set(n for i in range(2, ceil(sqrt(limit+1)))
                for n in range(i*2, limit+1, i)))

if __name__ == "__main__":
    from sys import argv
    n = int(argv[1])
    print("n: {}\np: {}".format(n, sieve(n)))

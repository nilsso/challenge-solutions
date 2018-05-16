# Number 2, v3
# Prompt: By considering the terms in the Fibonacci sequence whose values do
# not exceed four million, find the sum of the even-valued terms.


def evenFib(lim):
    a, b = 1, 1
    while b < lim:
        yield a + b
        a, b = a + 2*b, 2*a + 3*b


if __name__ == "__main__":
    from sys import argv
    for n in evenFib(int(argv[1]) if len(argv) > 1 else 4000000):
        print(n)

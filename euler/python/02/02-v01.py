# Number 2, v1
# Prompt: By considering the terms in the Fibonacci sequence whose values do
# not exceed four million, find the sum of the even-valued terms.


def fib(lim):
    a, b = 0, 1
    while b < lim:
        yield b
        a, b = b, a+b


if __name__ == "__main__":
    from sys import argv
    fibs = fib(int(argv[1]) if len(argv) > 1 else 4000000)
    print(sum(filter(lambda x: x % 2 == 0, fibs)))

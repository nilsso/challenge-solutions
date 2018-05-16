# Number 2, v2
# Prompt: By considering the terms in the Fibonacci sequence whose values do
# not exceed four million, find the sum of the even-valued terms.


from math import sqrt


phi = (1+sqrt(5))/2
evenPhi = 2*phi+1  # = phi**3


def evenFib(lim):
    n = 2
    while n < lim:
        yield n
        n = round(n * evenPhi)


if __name__ == "__main__":
    from sys import argv
    print(sum(evenFib(int(argv[1]) if len(argv) > 1 else 4000000)))

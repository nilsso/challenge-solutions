# Number 3
# Prompt: What is the largest prime factor of the number 600851475143?


from math import sqrt


def primeFactors(n):
    pf, i = [], 2
    while i <= sqrt(n):
        while n % i == 0:
            pf.append(i)
            n = n // i
        i += 1
    if n is not 1:
        pf.append(n)
    return pf


if __name__ == "__main__":
    from sys import argv
    print(max(primeFactors(int(argv[1]) if len(argv) > 1 else 600851475143)))

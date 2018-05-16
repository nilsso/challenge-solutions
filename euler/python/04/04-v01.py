# Number 4
# Promt: Find the largest palindrome made from the product of two 3-digit
# numbers.


from itertools import product


def isPalindrome(s):
    return s == s[::-1]


def palindromeProducts(lim):
    for a, b in product(range(1, lim), range(1, lim)):
        n = a * b
        if isPalindrome(str(n)):
            yield n


if __name__ == "__main__":
    from sys import argv
    print(max(set(palindromeProducts(int(argv[1]) if len(argv) > 1 else 1000))))

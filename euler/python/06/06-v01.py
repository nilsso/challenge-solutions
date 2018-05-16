# Number 6
# Promt: Find the difference between the sum of the squares of the first
# one-hundred natural numbers and the square of the sum of the first
# one-hundred natural numbers.


def e06(n):
    return (3*n**4 + 2*n**3 - 3*n**2 - 2 * n) // 12


if __name__ == "__main__":
    from sys import argv
    print(e06(int(argv[1]) if len(argv) > 1 else 100))

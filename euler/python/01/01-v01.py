# Number 1
# Prompt: Find the sum of all the multiples of 3 or 5 below 1000.


if __name__ == "__main__":
    from sys import argv
    n = int(argv[1]) if len(argv) > 1 else 1000
    print(sum({i for i in range(3, n, 3)} | {i for i in range(5, n, 5)}))

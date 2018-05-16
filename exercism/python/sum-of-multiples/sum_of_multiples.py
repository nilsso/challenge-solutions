def sum_of_multiples(limit, multiples):
    return sum(set(n for m in multiples for n in range(m, limit, m)))

if __name__ == "__main__":
    from sys import argv
    l = int(argv[1])
    m = list(map(int, argv[2:]))
    print("l: {}\nm: {}\ns: {}".format(l, m, sum_of_multiples(l, m)))

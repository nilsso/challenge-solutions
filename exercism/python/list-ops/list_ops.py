def length(xs):
    i = 0
    for _ in xs:
        i += 1
    return i


def reverse(xs):
    return xs[::-1]


# letting n = len(xs), foldl works as:
# f(f(f(f(acc, xs[0]), xs[1]), ...), xs[n-1])
def foldl(function, xs, acc):
    for n in xs:
        print(acc, n)
        acc = function(acc, n)
    return acc


# letting n = len(xs), foldr works as:
# f(xs[0], f(xs[1], f(..., f(xs[n-1], acc))))
def foldr(function, xs, acc):
    for n in reverse(xs):
        acc = function(n, acc)
    return acc


def append(xs, ys):
    return xs + ys


def concat(lists):
    return foldl(append, lists, [])


def filter_clone(function, xs):
    return [n for n in xs if function(n)]


def map_clone(function, xs):
    return [function(n) for n in xs]

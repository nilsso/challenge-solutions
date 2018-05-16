from itertools import cycle


def fence_pattern(rails):
    r = list(range(rails))
    return cycle(r + r[-2:0:-1])


def encoder(l, it):
    return sorted(l, key=lambda _: next(it))


def encode(msg, rails):
    p = fence_pattern(rails)
    return "".join(encoder(msg, p))


def decode(msg, rails):
    p = iter(encoder(range(len(msg)), fence_pattern(rails)))
    return "".join(encoder(msg, p))

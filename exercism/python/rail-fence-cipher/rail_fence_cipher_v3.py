from itertools import cycle


def fence_pattern(rails, length):
    r = list(range(rails))
    p = cycle(r + r[-2:0:-1])
    return sorted(range(length), key=lambda _: next(p))


def encode(message, rails):
    result = [""] * len(message)
    for i, j in enumerate(fence_pattern(rails, len(message))):
        result[i] = message[j]
    return "".join(result)


def decode(message, rails):
    result = [""] * len(message)
    for i, j in enumerate(fence_pattern(rails, len(message))):
        result[j] = message[i]
    return "".join(result)

from itertools import compress


def fence_pattern(rails, message_size):
    pattern = [[0]*message_size for i in range(rails)]
    div = 2*(rails-1)
    for i in range(message_size):
        j = abs(int((div / 2) + (i % div) - div))
        pattern[j][i] = 1
    return pattern[::-1]


def encode(msg, rails):
    pattern = fence_pattern(rails, len(msg))
    return "".join("".join(compress(msg, rail)) for rail in pattern)


def decode(msg, rails):
    n = len(msg)
    pattern = fence_pattern(rails, n)
    banks = []
    for rail in pattern:
        banks.append(msg[:rail.count(1)])
        msg = msg[rail.count(1):]
    s = ""
    for i in range(n):
        for j, rail in enumerate(pattern):
            if rail[i]:
                s += banks[j][0]
                banks[j] = banks[j][1:]
    return s

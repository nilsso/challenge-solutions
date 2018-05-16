from string import ascii_uppercase as LETTERS, digits as DIGITS
from itertools import product
import secrets


def nameBank(num_letters, num_digits):
    L = map("".join, product(LETTERS, repeat=num_letters))
    D = map("".join, product(DIGITS, repeat=num_digits))
    return list(map("".join, product(L, D)))


name_bank = nameBank(2, 3)


class Robot(object):
    def __init__(self):
        self.reset()

    def reset(self):
        i = secrets.randbelow(len(name_bank))
        self.name = name_bank.pop(i)


if __name__ == "__main__":
    from sys import argv
    print([Robot().name for i in range(int(argv[1]) if len(argv) > 1 else 5)])

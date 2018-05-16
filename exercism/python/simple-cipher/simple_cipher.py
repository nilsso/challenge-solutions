from string import ascii_lowercase as lc
import secrets
import re


a = ord("a")  # 97


def generate_key():
    return "".join(secrets.choice(lc) for i in range(100))


def get_offset(key, i):
    return ord(key[i % len(key)]) - a


def shift_chr(c, n):
    return chr((ord(c) - a + n) % 26 + a)


class Cipher(object):
    def __init__(self, key=None):
        if key and re.search("[^a-z]", key):
            raise ValueError("Invalid cipher key!")
        self.key = key or generate_key()

    def encode(self, text):
        return "".join(
                shift_chr(c, get_offset(self.key, i))
                for i, c in enumerate(re.sub("[^a-z]", "", text.lower())))

    def decode(self, text):
        return "".join(
                shift_chr(c, -get_offset(self.key, i))
                for i, c in enumerate(text))


class Caesar(Cipher):
    def __init__(self):
        super().__init__(key="d")

import re

# 1. a vowel (that isn't u)
# 2. u if it's proceeded by q
# 3. x or y if followed by a consonant
p = re.compile("[aeio]|(?<!q)u|[xy](?![aeiou])")


def translate_word(word):
    s = p.search(word).start()
    return word[s:]+word[:s]+"ay"


def translate(text):
    return (" ".join(map(translate_word, text.lower().split())))


if __name__ == "__main__":
    from sys import argv
    print(translate(argv[1]))

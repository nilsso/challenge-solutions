import re
from string import ascii_lowercase

transtable = str.maketrans(ascii_lowercase, ascii_lowercase[::-1])

def encode(text, size=5):
    text = re.sub("[\W]+", "", text).lower().translate(transtable)
    return " ".join( text[i:i+size] for i in range(0, len(text), size) )

def decode(text):
    return text.translate(transtable).replace(" ", "")

if __name__ == "__main__":
    from sys import argv
    a = encode(argv[1], int(argv[2]))
    print("encoded: \"{}\"".format(a))
    print("decoded: \"{}\"".format(decode(a)))


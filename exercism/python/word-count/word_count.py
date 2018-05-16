import re
from collections import Counter

# Method 2
def word_count(phrase):
    words = re.findall(r"([a-z0-9]+(?:'[a-z0-9]+)*)", phrase.lower())
    return dict(Counter(words))

# Method 1 (fastest)
# def word_count(phrase):
    # words = re.findall(r"([a-z0-9]+(?:'[a-z0-9]+)*)", phrase.lower())
    # return { w: words.count(w) for w in set(words) }

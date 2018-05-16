import re
def is_isogram(sentence):
    sentence = re.sub("[^a-z]", "", sentence.lower())
    characters = set(sentence)
    for c in characters:
        if sentence.count(c) > 1:
            return False
    return True

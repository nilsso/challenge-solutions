import re
def hey(phrase):
    phrase = phrase.strip()
    return (
            "Whoa, chill out!" if phrase.isupper() else
            "Sure." if phrase.endswith("?") else
            "Fine. Be that way!" if not phrase else
            "Whatever.")

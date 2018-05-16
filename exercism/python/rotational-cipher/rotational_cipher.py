from string import ascii_uppercase as uc, ascii_lowercase as lc
def rotate(text, key):
    return text.translate(str.maketrans(uc+lc,
            uc[key:] + uc[:key] + lc[key:] + lc[:key]))

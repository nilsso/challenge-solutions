import re


cleaner = re.compile("[^({\[\]})]")
pattern = re.compile("\[\]|\(\)|{}")


def check_brackets(input_string):
    def helper(s):
        m = pattern.search(s)
        return s if not m else helper(s[:m.start()] + s[m.end():])
    return len(helper(cleaner.sub("", input_string))) == 0

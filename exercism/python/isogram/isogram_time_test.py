import re
from timeit import timeit

s = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."

def method1(s):
    s = s.lower()
    for c in s:
        if c.isalpha() and s.count(c) > 1:
            return True
    return False

def method2(s):
    s = re.sub("[^\w]", "", s.lower())
    return len(s) == len(set(s))

def method3(s):
    p = re.compile("[^\w]")
    s = p.sub("", s.lower())
    return len(s) == len(set(s))

def method4(s):
    return not list(filter(
        lambda p: p[1].isalpha() and p[1] in s[p[0]+1:],
        enumerate(s.lower())))

methods = [
        method1,
        method2,
        method3,
        method4
        ]

for m in methods:
    print(timeit(lambda:(m(s)), number=10000))


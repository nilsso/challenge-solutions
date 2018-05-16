import re
from itertools import combinations


vertexP = re.compile('\+')


def isSegmentH(ascii_diagram, y, l, r):
    return all(c in "+-" for c in ascii_diagram[y][l:r+1])


def isSegmentV(ascii_diagram, x, t, b):
    return all(r[x] in "+|" for r in ascii_diagram[t:b+1])


def isRect(ascii_diagram, t, l, b, r):
    return (all(isSegmentH(ascii_diagram, row, l, r) for row in [t, b]) and
            all(isSegmentV(ascii_diagram, col, t, b) for col in [l, r]))


def count(ascii_diagram):
    vertexes = [
            (m.start(), y)
            for y, r in enumerate(ascii_diagram)
            for m in vertexP.finditer(r)]
    return sum(
            1 for a, b in combinations(vertexes, 2)
            if b[0] > a[0] and b[1] > a[1]
            if isRect(ascii_diagram, a[1], a[0], b[1], b[0]))

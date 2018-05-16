import re

NORTH, EAST, SOUTH, WEST = (0, 1), (1, 0), (0, -1), (-1, 0)

# Rotation matrices
a = [1,0,-1,0]
b = [0,1,0,-1]
c = [0,-1,0,1]
d = [1,0,-1,0]

class Robot(object):
    def __init__(self, bearing=NORTH, x=0, y=0):
        self._bearing = bearing
        self._x = x
        self._y = y

    @property
    def bearing(self):
        return self._bearing

    @property
    def coordinates(self):
        return self._x, self._y

    def turn_right(self, n=1):
        x, y = self._bearing
        n %= 4
        self._bearing = (a[n]*x + b[n]*y, c[n]*x + d[n]*y)

    def turn_left(self, n=1):
        self.turn_right(n=-n)

    def advance(self, n=1):
        x, y = self._bearing
        self._x += n*x
        self._y += n*y

    commands = {
            "R": turn_right,
            "L": turn_left,
            "A": advance }

    def simulate(self, cmd):
        for n, c in re.findall(r"((.)\2*)", cmd):
            self.commands[c](self, len(n))



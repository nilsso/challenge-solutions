import re
import numpy as np

NORTH, EAST, SOUTH, WEST = range(4)

directions = [
        np.array((0,  1)), np.array((1,  0)),
        np.array((0, -1)), np.array((-1, 0)) ]

class Robot(object):
    def __init__(self, bearing=NORTH, x=0, y=0):
        self._bearing = bearing
        self._coordinates = np.array((x, y))

    @property
    def bearing(self):
        return self._bearing

    @property
    def coordinates(self):
        return tuple(self._coordinates)

    def turn_right(self, n=1):
        self._bearing = (self._bearing + n)%4

    def turn_left(self, n=1):
        self.turn_right(-n)

    def advance(self, n=1):
        self._coordinates += n*directions[self._bearing]

    commands = {
            "R": turn_right,
            "L": turn_left,
            "A": advance }

    def simulate(self, cmd):
        # Featuring modified run length encoding pattern
        for n, c in re.findall(r"((.)\2*)", cmd):
            self.commands[c](self, len(n))



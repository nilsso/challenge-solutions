from collections import defaultdict
from sortedcontainers import SortedSet


class School(object):

    def __init__(self, name):
        self._name = name
        self._roster = defaultdict(SortedSet)

    def add(self, name, grade):
        self._roster[grade].add(name)

    def grade(self, grade):
        return tuple(self._roster[grade])

    def sort(self):
        return [(i, tuple(v)) for i, v in self._roster.items()]

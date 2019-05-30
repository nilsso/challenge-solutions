class Node(object):
    def __init__(self, value):
        pass

    def value(self):
        pass

    def next(self):
        pass

    def __next__(self):
        return next(self)


class LinkedList(object):
    def __init__(self, values=[]):
        pass

    def __len__(self):
        pass

    def head(self):
        pass

    def push(self, value):
        pass

    def pop(self):
        pass

    def reversed(self):
        pass


class EmptyListException(Exception):
    pass

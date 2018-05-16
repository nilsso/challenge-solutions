class Matrix(object):
    def __init__(self, s):
        self.rows = [list(map(int, l.split())) for l in s.splitlines()]
        self.columns = [list(col) for col in zip(*self.rows)]

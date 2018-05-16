from math import ceil


class Luhn(object):
    def __init__(self, id):
        self.id = id.replace(" ", "")

    def is_valid(self):
        try:
            if not int(self.id):
                return bool(len(self.id)-1)
        except ValueError:
            return False

        def helper(p):
            l, r = p[0], p[1]*2
            return l+(r if r < 10 else r-9)

        a = list(map(int, self.id.rjust(ceil(len(self.id)/2)*2, '0')))
        b = zip(*[iter(a[::-1])]*2)
        c = sum(map(helper, b))
        return c % 10 == 0

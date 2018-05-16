class Luhn(object):
    def __init__(self, id):
        self.id = id.replace(" ", "")

    def is_valid(self):
        try:
            if not int(self.id):
                return bool(len(self.id)-1)
        except ValueError:
            return False

        def helper(c):
            n = int(c)*2
            return n if n < 10 else n-9

        a = sum(map(int, self.id[-1::-2]))
        b = sum(map(helper, self.id[-2::-2]))
        return (a+b) % 10 == 0

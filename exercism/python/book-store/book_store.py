price = 800
discount = [1, 1-0.05, 1-0.10, 1-0.20, 1-0.25]


def group_price(group):
    n = len(group)
    return int(n * price * (1 - discount[n-1]))


class CountReducer:
    def __init__(self, data):
        self.data = [n for n in data if n[1] > 0]
        self.i = 0

    def __iter__(self):
        return self

    def __next__(self):
        if not self.data:
            raise StopIteration
        self.i %= len(self.data)
        n = self.data[self.i][0]
        self.data[self.i][1] -= 1
        if self.data[self.i][1] <= 0:
            self.data.pop(self.i)
        else:
            self.i += 1
        return n

    def reset(self):
        self.i = 0


def max_size_groups(cart, max_size, reset):
    '''
    :param list(int) cart Cart of items.
    :param int max_size Max size for groups.
    :param bool reset If true resets index of item to take to zero whenever a
        group is full.
    '''
    counts = [[n, cart.count(n)] for n in set(cart)]
    groups = [[]]
    it = CountReducer(counts.copy())
    for n in it:
        if len(groups[-1]) >= max_size:
            groups.append([])
            if reset:
                it.reset()
        if n not in groups[-1]:
            groups[-1].append(n)
        else:
            groups.append([n])
    return groups


def all_groups(cart):
    for reset in [0, 1]:
        for max_size in range(1, min(len(cart)+1, 6)):
            yield(max_size_groups(cart, max_size, reset))


def calculate_total(cart):
    if cart:
        return min(sum(group_price(g) for g in gs) for gs in all_groups(cart))
    return 0

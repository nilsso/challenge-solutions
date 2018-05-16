# Don't have to identify the hands
# Just have to collect which hands win
# Hands that win: straight, flushes, highest pairs and best pairs/cards
from collections import Counter

HC, OP, TP, TK, ST, FL, FH, FK, SF = range(9)

valueMap = {
        '1': 1, '2': 2, '3': 3, '4': 4, '5': 5, '6': 6, '7': 7,
        '8': 8, '9': 9, '10': 10, 'J': 11, 'Q': 12, 'K': 13, 'A': 14}


class Hand(object):
    def __init__(self, cards):
        self.cards = cards
        self.values = sorted(valueMap[c[:-1]] for c in self.cards)
        self.groups = [[v]*self.values.count(v) for v in set(self.values)]
        self.rank = Hand.rank(self)

    @staticmethod
    def isStraight(hand):
        for i in range(1, len(hand.values)):
            if hand.values[i]-1 != hand.values[i-1]:
                return False
        return True

    @staticmethod
    def isFlush(hand):
        return len(set(c[-1] for c in hand.cards)) == 1

    @staticmethod
    def rank(hand):
        c = Counter(hand.values)
        v = c.values()
        s, f = Hand.isStraight(hand), Hand.isFlush(hand)
        if s and f:
            return SF
        if len(v) == 2:
            if 4 in v:
                return FK
            else:
                return FH
        if s:
            return ST
        if f:
            return FL
        if len(v) == 3:
            if 3 in v:
                return TK
            else:
                return TP
        if len(v) == 5:
            return HC

    def __lt__(self, other):
        return self.rank < other.rank

    def __eq__(self, other):
        if isinstance(other, Hand):
            other = other.cards
        return [self.cards] == other

    def __repr__(self):
        return repr(self.cards)


def poker(hands):
    return max(Hand(hand) for hand in hands)

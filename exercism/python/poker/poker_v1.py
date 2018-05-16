H, P, P2, K3, S, F, FH, K4, SF = range(9)

valueMap = {
        '1': 1, '2': 2, '3': 3, '4': 4, '5': 5, '6': 6, '7': 7,
        '8': 8, '9': 9, '10': 10, 'J': 11, 'Q': 12, 'K': 13, 'A': 14}


def cardValue(card):
    return valueMap[card[:-1]]


class Hand(object):
    @staticmethod
    def values(cards):
        return sorted([cardValue(c) for c in cards], reverse=True)

    @staticmethod
    def groups(values, valueSet):
        return sorted(
                sorted([[i]*values.count(i) for i in valueSet], reverse=True),
                key=len, reverse=True)

    @staticmethod
    def rank(hand):
        n = len(hand.valueSet)
        if hand.isStraight() and hand.isFlush():
            return SF  # Straight flush
        if n == 2:
            if len(max(hand.groups, key=len)) == 4:
                return K4  # Four of a kind
            return FH  # Full house
        if hand.isFlush():
            return F  # Flush
        if hand.isStraight():
            return S  # Straight
        if n == 3:
            if len(max(hand.groups, key=len)) == 3:
                return K3  # Three of a kind
            return P2  # Two pair
        if n == 4:
            return P  # Pair
        if n == 5:
            return H  # High card
        else:  # Five of a kind, or unknown
            raise ValueError

    def isStraight(self):
        for i in range(1, len(self.values)):
            if self.values[i-1] != self.values[i]+1:
                return False
        return True

    def isFlush(self):
        return len(set(c[-1] for c in self.cards)) == 1

    def __init__(self, cards):
        self.cards = sorted(cards, key=lambda c: cardValue(c))
        self.values = Hand.values(self.cards)
        self.valueSet = set(self.values)
        self.groups = Hand.groups(self.values, self.valueSet)
        self.rank = Hand.rank(self)

    def compare(self, other):
        # Check rank
        if self.rank < other.rank:
            return -1
        elif self.rank > other.rank:
            return 1
        # Check tie
        if self.groups < other.groups:
            return -1
        elif self.groups > other.groups:
            return 1
        else:
            return 0


def poker(hands):
    winningIndexes = [0]
    winningHand = Hand(hands[0])
    for i, cards in list(enumerate(hands))[1:]:
        hand = Hand(cards)
        c = hand.compare(winningHand)
        if c == 1:
            winningIndexes = [i]
            winningHand = hand
        elif c == 0:
            winningIndexes.append(i)
    return [hands[i] for i in winningIndexes]

def makeSingles(n):
    return lambda dice: n*dice.count(n)

YACHT = lambda dice: 50 if len(set(dice)) is 1 else 0
ONES = makeSingles(1)
TWOS = makeSingles(2)
THREES = makeSingles(3)
FOURS = makeSingles(4)
FIVES = makeSingles(5)
SIXES = makeSingles(6)
FULL_HOUSE = lambda dice: sum(dice) if len(set(dice)) is 2 and dice.count(dice[0]) in (2,3) else 0
FOUR_OF_A_KIND = lambda dice: 4*dice[2] if len(set(dice)) <= 2 and dice.count(dice[2]) >= 4 else 0
LITTLE_STRAIGHT = lambda dice: 30 if dice == [1,2,3,4,5] else 0
BIG_STRAIGHT = lambda dice: 30 if dice == [2,3,4,5,6] else 0
CHOICE = sum

def score(dice, category):
    dice = sorted(dice)
    return category(dice)


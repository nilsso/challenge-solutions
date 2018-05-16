from itertools import permutations


def rightOf(lhs, rhs):
    return lhs - rhs == 1


def beside(lhs, rhs):
    return abs(lhs - rhs) == 1


def solution():
    houses = first, second, third, fourth, fifth = range(5)
    p = list(permutations(houses))
    return ('It is the {} who drinks the water.\n'
            'The {} keeps the zebra.').format(*next([{
                english: 'Englishman',
                ukraine: 'Ukrainian',
                japan: 'Japanese',
                norway: 'Norwegian',
                spain: 'Spaniard'}
                [i] for i in (water, zebra)]
                for (yellow, blue, ivory, green, red) in p
                if rightOf(green, ivory)
                for (english, ukraine, japan, norway, spain) in p
                if english is red
                if norway is first
                if beside(norway, blue)
                for (milk, orange, coffee, water, tea) in p
                if coffee is green
                if milk is third
                if ukraine is tea
                for (kool, chesterField, oldGold, luckyStrike, parliament) in p
                if kool is yellow
                if luckyStrike is orange
                if japan is parliament
                for (zebra, dog, snail, horse, fox) in p
                if spain is dog
                if oldGold is snail
                if beside(chesterField, fox)
                if beside(kool, horse)))

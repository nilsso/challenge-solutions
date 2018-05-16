allergens = [ "eggs", "peanuts", "shellfish", "strawberries",
        "tomatoes", "chocolate", "pollen", "cats" ]

class Allergies(object):

    def __init__(self, score):
        self.score = score

    def is_allergic_to(self, item):
        return bool(self.score & 2**allergens.index(item))

    @property
    def lst(self):
        return [ a for a in allergens if self.is_allergic_to(a) ]


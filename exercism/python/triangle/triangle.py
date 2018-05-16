class TriangleError(Exception):
    pass


def triangle_property_decorator(cls):
    def get_side_factory(i):
        return lambda self: self.sides[i]

    for i, side in enumerate(['a', 'b', 'c']):
        setattr(cls, side, property(get_side_factory(i)))

    return cls


@triangle_property_decorator
class Triangle(object):
    kinds = ['equilateral', 'isosceles', 'scalene']

    def __init__(self, a, b, c):
        self.sides = sorted([a, b, c])

        if any(side <= 0 for side in self.sides):
            raise TriangleError('Contains negative or zero length sides')
        if (self.a + self.b <= self.c):
            raise TriangleError('Impossible side lengths')

    def kind(self):
        return Triangle.kinds[len(set(self.sides))-1]

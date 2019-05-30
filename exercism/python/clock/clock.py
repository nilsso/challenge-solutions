class Clock(object):
    def __init__(self, hour, minute):
        self.h = (hour + minute // 60) % 24
        self.m = minute % 60

    def __repr__(self):
        return "%02d:%02d" % (self.h, self.m)

    def __eq__(self, other):
        return self.h is other.h and self.m is other.m

    def __add__(self, minutes):
        self.__init__(self.h, self.m + minutes)
        return self

    def __sub__(self, minutes):
        self.__add__(-minutes)
        return self

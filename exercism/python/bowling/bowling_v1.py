class Frame(list):
    def __init__(self, pins):
        self.append(pins)

    def isComplete(self):
        return len(self) == 2 or self[0] == 10

    def isSpare(self):
        return len(self) == 2 and sum(self) == 10

    def isStrike(self):
        return len(self) == 1 and self[0] == 10

    def isStrikeOrSpare(self):
        return self.isStrike() or self.isSpare()


class BowlingGame(list):
    @property
    def frame(self):
        return self[-1]

    @property
    def last(self):
        return self[9] if len(self) >= 10 else None

    def isComplete(self):
        return (self.last is not None and
                self.last.isComplete()
                and (not self.last.isStrikeOrSpare()
                    or (self.last.isSpare() and len(self) == 11)
                    or (self.last.isStrike() and
                        len(self) == 12 or
                        len(self) == 11 and len(self[10]) == 2)))

    def scoreFrame(self, i):
        score = sum(self[i])
        if self[i].isSpare():
            score += self[i+1][0]
        elif self[i].isStrike():
            if len(self[i+1]) == 2:
                score += sum(self[i+1])
            else:
                score += self[i+1][0]
                score += self[i+2][0]
        return score

    def roll(self, pins):
        if pins < 0 or pins > 10:
            raise ValueError('Number of pins out of bounds')
        if self.isComplete():
            raise IndexError('Cannot roll after complete game')
        if not self or self.frame.isComplete():
            # New frame
            self.append(Frame(pins))
        else:
            # Finish frame
            if sum(self.frame + [pins]) > 10:
                raise ValueError('Number of pins in frame exceeds 10')
            self.frame.append(pins)

    def score(self):
        if not self.isComplete():
            raise IndexError('Failed to score incomplete game')
        return sum(self.scoreFrame(i) for i in range(10))

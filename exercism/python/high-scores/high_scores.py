class HighScores(object):
    def __init__(self, scores):
        self.scores = scores
        self._latest = scores[-1]
        self._best = max(scores)
        self._top = list(reversed(sorted(scores)[-3:]))

    def latest(self):
        return self._latest

    def personal_best(self):
        return self._best

    def personal_top(self):
        return self._top

    def report(self):
        return "Your latest score was %d. That's %syour personal best!" % (
                self._latest,
                "" if self._latest is self._best else
                "%d short of " % (self._best - self._latest))

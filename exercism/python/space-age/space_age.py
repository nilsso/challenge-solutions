class SpaceAge(object):
    def __init__(self, seconds):
        self.seconds = seconds


earth_year = 365.25*24*60*60
# Populate attributes
for name, seconds in [
        ("earth",   earth_year),
        ("mercury", earth_year*0.2408467),
        ("venus",   earth_year*0.61519726),
        ("mars",    earth_year*1.8808158),
        ("jupiter", earth_year*11.862615),
        ("saturn",  earth_year*29.447498),
        ("uranus",  earth_year*84.016846),
        ("neptune", earth_year*164.79132)]:
    setattr(SpaceAge, "on_"+name,
            lambda self, seconds=seconds: round(self.seconds/seconds, 2))


SpaceAge(10000000).on_earth()

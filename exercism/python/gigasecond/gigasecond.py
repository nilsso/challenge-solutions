from datetime import timedelta

# All the same:
# 1000000000
# 10**9
# 1E9

def add_gigasecond(birth_date):
    return birth_date + timedelta(seconds=1E9)

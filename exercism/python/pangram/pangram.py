## Is pangram
# Determines if a string is a pangram. A pangram is a
# sentence using every letter of the alphabet at least once.
# Case insensitive.
import re
def is_pangram(s):
    return 26 == len(set(re.sub("[^a-z]", "", s.lower())))

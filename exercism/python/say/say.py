import io

short_scale = [ " thousand ", " million ", " billion " ]

"""
Initial list of English cardinal numbers
"""
cardinals = {
        0:"zero",
        1:"one", 2:"two", 3:"three", 4:"four", 5:"five", 6:"six", 7:"seven",
        8:"eight", 9:"nine", 10:"ten", 11:"eleven", 12:"twelve", 13:"thirteen",
        14:"fourteen", 15:"fifteen", 16:"sixteen", 17:"seventeen",
        18:"eighteen", 19:"nineteen", 20:"twenty", 30:"thirty", 40:"forty",
        50:"fifty", 60:"sixty", 70:"seventy", 80:"eighty", 90:"ninety" }

"""
Finish populating the list
"""
for i in range(1, 100):
    if not i in cardinals:
        t, o = divmod(i, 10)
        cardinals[i] = "{}-{}".format(cardinals[t*10], cardinals[o])

"""
Convert a period to english cardinal number form
"""
def cardinal_period(p):
    h, t = divmod(p, 100)
    return \
            cardinals[h] + " hundred and " + cardinals[t] if h and t else\
            cardinals[h] + " hundred" if h else\
            cardinals[t] 

"""
ExercismIO Python #23 (say)
"""
def say(number):
    if number < 0 or number >= 1e12:
        raise ValueError(".+")
    if number == 0:
        return "zero"
    if number < 100:
        return cardinals[number]
    """
    Populate a list of periods
    """
    periods = []
    while number:
        number, period = divmod(number, 1000)
        periods.append(period)
    """
    Write cardinal periods to buffer and return contents
    """
    buffer = io.StringIO()
    for i,p in reversed(list(enumerate(periods[1:]))):
        if p:
            buffer.write(cardinal_period(p))
            buffer.write(short_scale[i])
    if periods[0]:
        buffer.write("and " if periods[0] < 100 else "")
        buffer.write(cardinal_period(periods[0]))
    return buffer.getvalue().strip()

if __name__ == "__main__":
    from sys import argv
    print(say(int(argv[1])))

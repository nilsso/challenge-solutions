def is_armstrong(number):
    s = str(number)
    n = len(s)
    return number == sum((ord(i)-48)**n for i in s)

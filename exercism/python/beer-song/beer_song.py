from io import StringIO


a = '{0} bottle{1} of beer on the wall, {0} bottle{1} of beer.\n'
b = 'Take {} down and pass it around, '
c = 'Go to the store and buy some more, '
d = '{} bottle{} of beer on the wall.\n'


def verse(n):
    s = StringIO()
    s.write(a.format(n or 'no more', 's' if n-1 else '').capitalize())
    s.write(b.format('one' if n-1 else 'it') if n else c)
    s.write(d.format((n-1) or 'no more' if n else 99, 's' if (n-2) else ''))
    return s.getvalue()


def song(n, end=0):
    if n > end:
        return verse(n) + '\n' + song(n-1, end)
    return verse(end) + '\n'


if __name__ == "__main__":
    from sys import argv
    choice = int(argv[1])
    n = int(argv[2])
    print(verse(n) if choice else song(n))

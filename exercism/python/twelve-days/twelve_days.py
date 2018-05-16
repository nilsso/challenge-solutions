intro = 'On the {} day of Christmas my true love gave to me, '


ords = [
        'first', 'second', 'third', 'fourth', 'fifth', 'sixth',
        'seventh', 'eighth', 'ninth', 'tenth', 'eleventh', 'twelfth']


verse_text = [
        'a Partridge in a Pear Tree',
        'two Turtle Doves',
        'three French Hens',
        'four Calling Birds',
        'five Gold Rings',
        'six Geese-a-Laying',
        'seven Swans-a-Swimming',
        'eight Maids-a-Milking',
        'nine Ladies Dancing',
        'ten Lords-a-Leaping',
        'eleven Pipers Piping',
        'twelve Drummers Drumming']


def verse(n):
    a = intro.format(ords[n-1])
    b = ', '.join(verse_text[i] for i in reversed(range(1, n)))
    c = (', and ' if (n-1) else '') + verse_text[0] + '.\n'
    return a + b + c


def verses(start, end):
    return '\n'.join(verse(i) for i in range(start, end+1)) + '\n'


def sing():
    return verses(1, 12)


if __name__ == "__main__":
    from sys import argv
    print(verses(int(argv[1]), int(argv[2])))

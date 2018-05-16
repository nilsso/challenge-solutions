def verify(isbn):
    try:
        i = iter((i+1)*n for i, n in enumerate(
            [int(c) for c in isbn.replace('-', '')[:-1]] +
            [10 if isbn[-1] is 'X' else int(isbn[-1])]))
        return sum(i) % 11 == 0
    except (IndexError, ValueError):
        return False

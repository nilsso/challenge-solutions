from bidict import bidict


events = bidict([
    ('wink', 1), ('double blink', 2), ('close your eyes', 4), ('jump', 8)])


def is_valid_number(number):
    if isinstance(number, str):
        return all(c in "01" for c in number)
    return 0 < number < 32


def is_valid_code(secret_code):
    return all(e in events for e in secret_code)


def handshake(number):
    if not is_valid_number(number):
        return []
    if isinstance(number, str):
        number = int(number, 2)
    sequence = [e for e, i in events.items() if number & i]
    return sequence[::1 if (not number & 16) else -1]


def code(secret_code):
    if not is_valid_code(secret_code):
        return '0'
    sequence = [events[e] for e in secret_code]
    r = (sequence != sorted(sequence))
    return '{:b}'.format(sum(sequence) + (0 if not r else 16))

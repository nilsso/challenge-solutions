from itertools import zip_longest


def transpose(in_str):
    lines = in_str.splitlines()
    return '\n'.join(
            ''.join(
                c or ' ' if any(line[i:]) else ''
                for i, c in enumerate(line))
            for line in zip_longest(*lines))

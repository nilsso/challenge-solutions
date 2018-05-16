def transpose(matrix):
    return zip(*matrix)


def occurences(group, value):
    for i, v in enumerate(group):
        if v == value:
            yield i


def saddle_points(matrix):
    if len(set(map(len, matrix))) > 1:
        raise ValueError("Irregular matrix")
    col_max = set(
            (x, y)
            for x, col in enumerate(matrix)
            for y in occurences(col, max(col)))
    row_min = set(
            (x, y)
            for y, row in enumerate(transpose(matrix))
            for x in occurences(row, min(row)))
    return col_max & row_min

valid_chars = " *12345678"
tt = str.maketrans(' 1234567', '12345678')


def rows_to_board(row_strings):
    return [list(r) for r in row_strings]


def board_to_rows(board):
    return [''.join(r) for r in board]


def is_valid_board(row_strings):
    w = len(row_strings[0])
    for row in row_strings:
        if w != len(row):
            return False
        for c in row:
            if c not in valid_chars:
                return False
    return True


def mines(board):
    for y, row in enumerate(board):
        for x, c in enumerate(row):
            if board[y][x] == '*':
                yield x, y


def neighbors(board, x, y):
    w, h = len(board[0]), len(board)
    for y2 in range(max(y-1, 0), min(y+2, h)):
        for x2 in range(max(x-1, 0), min(x+2, w)):
            if not (x2 == x and y2 == y):
                yield x2, y2, board[y2][x2]


def board(row_strings):
    if not row_strings:
        return []
    if not is_valid_board(row_strings):
        raise ValueError("Invalid board")
    board = rows_to_board(row_strings)
    for x, y in mines(board):
        for nx, ny, c in neighbors(board, x, y):
            board[ny][nx] = c.translate(tt)
    return board_to_rows(board)

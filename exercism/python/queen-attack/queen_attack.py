def validate_position(w_pos, b_pos):
    if w_pos == b_pos:
        raise ValueError("Positions overlapping")
    for v in [w_pos[0], w_pos[1], b_pos[0], b_pos[1]]:
        if not (0 <= v <= 7):
            raise ValueError("Invalid position")


def board(w_pos, b_pos):
    validate_position(w_pos, b_pos)
    board = [['_'] * 8 for _ in range(8)]
    board[w_pos[0]][w_pos[1]] = 'W'
    board[b_pos[0]][b_pos[1]] = 'B'
    return [''.join(row) for row in board]


def can_attack(w_pos, b_pos):
    validate_position(w_pos, b_pos)
    dy = b_pos[0] - w_pos[0]
    dx = b_pos[1] - w_pos[1]
    return dy == 0 or dx == 0 or abs(dy // dx) == 1

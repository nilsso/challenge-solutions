segmented_digits = {
        '     |  |   ': '1', ' _  _||_    ': '2',
        ' _  _| _|   ': '3', '   |_|  |   ': '4',
        ' _ |_  _|   ': '5', ' _ |_ |_|   ': '6',
        ' _   |  |   ': '7', ' _ |_||_|   ': '8',
        ' _ |_| _|   ': '9', ' _ | ||_|   ': '0'}


def is_valid_grid(input_grid):
    if not input_grid:
        return False  # Empty
    if len(input_grid) % 4 != 0:
        return False  # Number of strings not divisible by 4
    if any(len(line) % 3 != 0 for line in input_grid):
        return False  # String lengths not all divisible by 3
    for i in range(0, len(input_grid), 4):
        if len(set(map(len, input_grid[i:i+4]))) != 1:
            return False  # Uneven string lengths in a 4 line group
    return True


def convert(input_grid):
    if not is_valid_grid(input_grid):
        raise ValueError("Invalid input grid")
    return (
            # Join each line with a comma
            ','.join(
                # Join each character in a line
                ''.join(
                    # Convert segmented digit to character
                    segmented_digits.get(
                        # Join digit segments
                        ''.join(r[j:j+3] for r in input_grid[i:i+4]), '?')
                    for j in range(0, len(input_grid[i]), 3))
                for i in range(0, len(input_grid), 4)))

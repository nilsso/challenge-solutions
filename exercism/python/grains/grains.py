def on_square(integer_number):
    if not (1 <= integer_number <= 64):
        raise ValueError("Invalid range!")
    return 2**(integer_number-1)


def total_after(integer_number):
    if not (1 <= integer_number <= 64):
        raise ValueError("Invalid range!")
    return sum(on_square(i) for i in range(1, integer_number+1))

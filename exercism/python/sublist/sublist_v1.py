SUBLIST, SUPERLIST, EQUAL, UNEQUAL = 0, 1, 2, 3


def check_sublist(lhs, rhs):
    """ Naive search """
    n = len(lhs)
    if not n:
        return True
    for i in range(len(rhs)-n+1):
        if rhs[i:i+n] == lhs:
            return True
    return False


def check_lists(lhs, rhs):
    if lhs == rhs:
        return EQUAL
    elif len(rhs) > len(lhs):
        return SUBLIST if check_sublist(lhs, rhs) else UNEQUAL
    else:
        return SUPERLIST if check_sublist(rhs, lhs) else UNEQUAL

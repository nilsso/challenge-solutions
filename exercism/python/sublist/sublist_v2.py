SUBLIST, SUPERLIST, EQUAL, UNEQUAL = 0, 1, 2, 3


def failure_function(pattern):
    """
    Knuth-Morris-Pratt failure function.

    :param pattern: Search pattern string
    :return: Failure function table for the pattern string
    :type pattern: str
    :rtype: list(int)
    """
    n = len(pattern)
    T = [-1] + [None] * (n - 1)
    i, j = 1, T[0]  # i:position in pattern, j:pointer
    while i < n:
        if pattern[i] == pattern[j+1]:
            T[i] = j + 1
        elif j == -1:
            T[i] = -1
        else:
            j = T[j]
        i += 1
    return T


def kmp_search(pattern, text):
    """
    Knuth-Morris-Pratt search.

    :param pattern: Search pattern string
    :param text: String in which the pattern is being searched for
    :return: Start and end positions in text containing the pattern
    :type pattern: str
    :type text: str
    :rtype: tuple(int, int)
    """
    T = failure_function(pattern)
    m, n = len(text), len(pattern)
    i, j = 0, 0  # i:position in text, j:position in pattern
    while i < m:
        if text[i] == pattern[j]:
            i += 1
            j += 1
            if j == n:
                return (i - n, i - 1)
        elif j > 0:
            j = T[j - 1] + 1
        else:
            i += 1


def check_lists_helper(lhs, rhs):
    return kmp_search(lhs, rhs) if (lhs and rhs) else True if rhs else False


def check_lists(lhs, rhs):
    if len(lhs) == len(rhs):
        return EQUAL if lhs == rhs else UNEQUAL
    elif len(rhs) > len(lhs):
        return SUBLIST if check_lists_helper(lhs, rhs) else UNEQUAL
    else:
        return SUPERLIST if check_lists_helper(rhs, lhs) else UNEQUAL

def flatten_helper(iterable):
    for i in iterable:
        if hasattr(i, "__iter__") and not isinstance(i, str):
            yield from flatten_helper(i)
        elif i is not None:
            yield i


def flatten(iterable):
    return list(flatten_helper(iterable))

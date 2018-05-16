m = {'(': ')', '[': ']', '{': '}'}


def check_brackets(input_string):
    stack = []
    # for c in input_string:
    for c in [c for c in input_string if c in '([{}])']:
        if c in m:
            stack.append(c)
        # elif c in m.values():
        else:
            if len(stack) == 0 or m[stack.pop()] != c:
                return False
    return len(stack) == 0

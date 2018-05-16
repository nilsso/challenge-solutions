import re
import operator

ER_INVALID_QUESTION = 'Invalid question syntax'
ER_MISSING_OPERATOR = 'Missing operator before "{}"'
ER_MISSING_OPERAND = 'Missing operand after "{}"'
ER_INVALID_OPERATOR = 'Invalid operator "{}"'
ER_INVALID_OPERAND = 'Invalid operand "{}"'

pat_a = re.compile(r'What is (-?\d+)([^?]+)\?')
pat_b = re.compile(r'\s?((?:[a-z]+\s?)*)\s([-\d]*)')

ops = {
        'plus': operator.add,
        'minus': operator.sub,
        'multiplied by': operator.mul,
        'divided by': operator.truediv}


def calculate(question):
    m = pat_a.search(question)
    if not m:
        raise ValueError(ER_INVALID_QUESTION)
    if not m[1].lstrip('-').isnumeric():
        raise ValueError(ER_INVALID_OPERAND.format(m[1]))
    stack = [int(m[1])]
    for op_token, rhs_token in pat_b.findall(m[2]):
        if not op_token:
            raise ValueError(ER_MISSING_OPERATOR.format(rhs_token))
        if not rhs_token:
            raise ValueError(ER_MISSING_OPERAND.format(op_token))
        if op_token not in ops:
            raise ValueError(ER_INVALID_OPERATOR.format(op_token))
        if not rhs_token.lstrip('-').isnumeric():
            raise ValueError(ER_INVALID_OPERAND.format(rhs_token))
        op, lhs, rhs = ops[op_token], stack.pop(), int(rhs_token)
        stack.append(op(lhs, rhs))
    return stack.pop()

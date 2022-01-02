import pyparsing as pp
lines = list(filter(None, open('input.txt').read().split('\n')))

expressions = [[c for c in line if c != ' '] for line in lines]


def eval_expression(arr, index=0):
    value = None
    operator = None
    while True:
        new_value = None
        if index >= len(arr) or arr[index] == ')':
            return value, index
        if arr[index] in ('+', '*'):
            operator = arr[index]
        elif arr[index] == '(':
            new_value, index = eval_expression(arr, index + 1)
        else:
            new_value = int(arr[index])

        if new_value:
            if not value:
                value = new_value
            else:
                value = value * new_value if operator == '*' else value + new_value
                operator = None
        index += 1


# for exp in expressions:
#     print(eval_expression(exp))

print(sum(map(lambda x: eval_expression(x)[0], expressions)))


def create_parse_action(op):
    def parse_action(tokens):
        vals = [int(t) for t in tokens[0]]
        ret = 0
        if op == '+':
            ret = sum(vals)
        else:
            value = 1
            for token in vals:
                value *= token
            ret = value
        return ret

    return parse_action


expr = pp.infixNotation(pp.Word(pp.nums), [
    (pp.Suppress('+'), 2, pp.opAssoc.LEFT, create_parse_action('+')),
    (pp.Suppress('*'), 2, pp.opAssoc.LEFT, create_parse_action('*')),
])

total = 0
for line in lines:
    parsed = expr.parseString(line)
    print(parsed)
    total += parsed[0]

print(total)
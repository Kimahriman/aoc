import re
lines = open('input.txt').read().split('\n')

iterator = iter(lines)

rules = {}
for line in iterator:
    if not line:
        break
    number = int(line.split(':')[0])
    rule = line.split(': ')[1]
    match = re.match(r'"(\w)"', rule)
    if match:
        rules[number] = match[1]
    else:
        rules[number] = [[int(s) for s in sequence.split(' ')]
                         for sequence in rule.split(' | ')]

messages = list(filter(None, iterator))


def check_rule(message, index, rule_num):
    if index >= len(message):
        return
    rule = rules[rule_num]
    # print('Checking', message, index, rule)
    if isinstance(rule, str):
        if message[index] == rule:
            # print('Yielding const', index + 1)
            yield index + 1
    else:
        for ruleset in rule:
            # print('Checking ruleset', ruleset)
            new_indices = [index]
            valid = True
            for num in ruleset:
                new_indices = [
                    match for new_index in new_indices
                    for match in check_rule(message, new_index, num)
                ]
                # print(message, index, offsets)
                if len(new_indices) == 0:
                    # print('No valid paths found')
                    valid = False
                    break
                # print('Found new indices', new_indices)

            if valid:
                # print('Found valid ruleset', new_indices)
                yield from new_indices


count = 0
for message in messages:
    indices = list(check_rule(message, 0, 0))
    print(message, indices)
    if any((index == len(message) for index in indices)):
        count += 1
        print('Found match')

print(count)

rules[8] = [[42], [42, 8]]
rules[11] = [[42, 31], [42, 11, 31]]

count = 0
for message in messages:
    indices = list(check_rule(message, 0, 0))
    print(message, indices)
    if any((index == len(message) for index in indices)):
        count += 1
        print('Found match')

print(count)

import re

data = open('input.txt').read().split('\n')

forward = {}
backward = {}
for line in data:
    if not line:
        continue
    match = re.match(r'^(\w+ \w+) bags contain (.*)\.$', line)
    parent = match[1]
    children = []
    for child in match[2].split(', '):
        if child == 'no other bags':
            forward[child] = []
            continue
        child_match = re.match(r'(\d+) (\w+ \w+) bags?', child)
        children.append((child_match[2], int(child_match[1])))

    forward[parent] = children
    for child, _ in children:
        backward.setdefault(child, []).append(parent)


def find_bags(current):
    parents = backward.get(current, [])
    yield from parents
    for parent in parents:
        yield from find_bags(parent)


bags = set(find_bags('shiny gold'))
print(len(bags))


def inner_bag_count(current):
    sum = 0
    for child, count in forward.get(current, []):
        sum += count
        sum += inner_bag_count(child) * count

    return sum


count = inner_bag_count('shiny gold')
print(count)
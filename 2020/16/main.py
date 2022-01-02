import itertools
import re

data = open('input.txt').read().splitlines()

lines = iter(data)
reqs = []
for line in lines:
    if not line:
        break

    match = re.match(r'(.*): (\d+)-(\d+) or (\d+)-(\d+)', line)
    reqs.append((match[1], (int(match[2]), int(match[3])), (int(match[4]),
                                                            int(match[5]))))

assert next(lines).strip() == 'your ticket:'

my_ticket = [int(x) for x in next(lines).strip().split(',')]

assert next(lines).strip() == ''
assert next(lines).strip() == 'nearby tickets:'

nearby_tickets = []
for line in lines:
    line = line.strip()
    if line:
        nearby_tickets.append([int(x) for x in line.split(',')])


def valid(value, req):
    return (value >= req[1][0]
            and value <= req[1][1]) or (value >= req[2][0]
                                        and value <= req[2][1])


error_rate = 0
valid_tickets = [my_ticket]
for ticket in nearby_tickets:
    is_valid = True
    for value in ticket:
        if not any((valid(value, req) for req in reqs)):
            error_rate += value
            is_valid = False

    if is_valid:
        valid_tickets.append(ticket)

print(error_rate)

valid_cols = []
for i in range(len(reqs)):
    valid_reqs = []
    for j in range(len(reqs)):
        if all((valid(ticket[j], reqs[i]) for ticket in valid_tickets)):
            valid_reqs.append(j)

    valid_cols.append(valid_reqs)

print(valid_cols)

changed = True
while changed:
    changed = False
    for i, col in enumerate(valid_cols):
        if len(col) == 1:
            for j, c in enumerate(valid_cols):
                if i != j and col[0] in c:
                    changed = True
                    c.remove(col[0])

print(valid_cols)
valid_cols = [c[0] for c in valid_cols]
print(valid_cols)

total = 1
for i, req in enumerate(reqs):
    if 'departure' in req[0]:
        total *= my_ticket[valid_cols[i]]

print(total)

from typing import DefaultDict


data = open('input.txt').read().splitlines()

positions = DefaultDict(lambda: 0)
lines = 0

for line in data:
    lines += 1
    for i, c in enumerate(line):
        positions[i] += int(c)

lines /= 2

gamma = ''.join(('1' if positions[i] > lines else '0' for i in range(len(positions))))
epsilon = ''.join(('0' if positions[i] > lines else '1' for i in range(len(positions))))

gamma_num = int(gamma, 2)
epsilon_num = int(epsilon, 2)

print(gamma_num * epsilon_num)

remaining = {i for i in range(len(data))}
index = 0
while len(remaining) > 1:
    pos = set()
    neg = set()
    for r in remaining:
        # print(r, index)
        if data[r][index] == '1':
            pos.add(r)
        elif data[r][index] == '0':
            neg.add(r)
    
    if len(pos) >= len(neg):
        remaining = pos
    else:
        remaining = neg
    index += 1

ogr = int(data[remaining.pop()], 2)

remaining = {i for i in range(len(data))}
index = 0
while len(remaining) > 1:
    pos = set()
    neg = set()
    for r in remaining:
        # print(r, index)
        if data[r][index] == '1':
            pos.add(r)
        elif data[r][index] == '0':
            neg.add(r)
    
    if len(pos) < len(neg):
        remaining = pos
    else:
        remaining = neg
    index += 1

co2 = int(data[remaining.pop()], 2)

print(ogr * co2)

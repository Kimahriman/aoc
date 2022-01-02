from typing import DefaultDict

data = open('input.txt').read().splitlines()

lines = [[[int(val)  for val in pair.split(',')] for pair in line.split(' -> ')] for line in data]
grid = DefaultDict(lambda: 0)

for line in lines:
    if line[0][0] == line[1][0]:
        step = 1 if line[0][1] < line[1][1] else -1
        for y in range(line[0][1], line[1][1] + step, step):
            grid[line[0][0], y] += 1
    elif line[0][1] == line[1][1]:
        step = 1 if line[0][0] < line[1][0] else -1
        for x in range(line[0][0], line[1][0] + step, step):
            grid[x, line[0][1]] += 1
    else:
        x_step = 1 if line[0][0] < line[1][0] else -1
        y_step = 1 if line[0][1] < line[1][1] else -1
        for i in range(abs(line[0][0] - line[1][0]) + 1):
            grid[line[0][0] + i * x_step, line[0][1] + i * y_step] += 1

total = 0
for g in grid.values():
    if g > 1:
        total += 1

print(total)
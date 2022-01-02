data = open('input.txt').read().splitlines()

hor = 0
depth = 0

for line in data:
    dir, num = line.split(' ')
    if dir == 'down':
        depth += int(num)
    elif dir == 'up':
        depth -= int(num)
    elif dir == 'forward':
        hor += int(num)

print(hor * depth)

aim = 0
hor = 0
depth = 0

for line in data:
    dir, num = line.split(' ')
    if dir == 'down':
        aim += int(num)
    elif dir == 'up':
        aim -= int(num)
    elif dir == 'forward':
        hor += int(num)
        depth += aim * int(num)

print(hor * depth)
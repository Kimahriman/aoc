data = list(filter(None, open('input.txt').read().split('\n')))

x = 0
y = 0
facing = 90

for line in data:
    action = line[0]
    val = int(line[1:])

    if action == 'N' or (action == 'F' and facing == 0):
        y += val
    elif action == 'S' or (action == 'F' and facing == 180):
        y -= val
    elif action == 'E' or (action == 'F' and facing == 90):
        x += val
    elif action == 'W' or (action == 'F' and facing == 270):
        x -= val
    elif action == 'R':
        facing += val
        facing %= 360
    elif action == 'L':
        facing -= val
        facing %= 360

mdist = abs(x) + abs(y)
print(mdist)

x = 0
y = 0
wayx = 10
wayy = 1
for line in data:
    action = line[0]
    val = int(line[1:])

    if action == 'N':
        wayy += val
    elif action == 'S':
        wayy -= val
    elif action == 'E':
        wayx += val
    elif action == 'W':
        wayx -= val
    elif (action == 'R' and val == 90) or (action == 'L' and val == 270):
        wayx, wayy = wayy, -wayx
    elif (action == 'L' and val == 90) or (action == 'R' and val == 270):
        wayx, wayy = -wayy, wayx
    elif (action == 'R' or action == 'L') and val == 180:
        wayx, wayy = -wayx, -wayy
    elif action == 'F':
        x += wayx * val
        y += wayy * val
    else:
        raise Exception(f'Unknown command {line}')

print(abs(x) + abs(y))
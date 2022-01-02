data = list(filter(None, open('input.txt').read().split('\n')))

acc = 0
visited = set()
line = 0
changed = 0
while True:
    if line in visited:
        print('Tried changing line', changed)
        acc = 0
        line = 0
        visited = set()
        changed += 1
        continue
    if line >= len(data):
        break

    visited.add(line)

    op, val = data[line].split(' ')

    if (op == 'jmp' and line != changed) or (op == 'nop' and line == changed):
        line += int(val)
    else:
        if op == 'acc':
            acc += int(val)
        line += 1

print(acc, changed)

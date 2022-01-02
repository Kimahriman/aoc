data = list(filter(None, open('input.txt').read().split('\n')))

earliest = int(data[0])
ids = [int(d) for d in data[1].split(',') if d != 'x']

earliest_id = ids[0]
lowest = int(earliest / ids[0] + 1) * ids[0] - earliest

for _id in ids[1:]:
    new_wait = int(earliest / _id + 1) * _id - earliest
    if new_wait < lowest:
        print('Setting lowest to', new_wait)
        lowest = new_wait
        earliest_id = _id

print(earliest_id, lowest, earliest_id * lowest)

ids = [(int(d), i) for i, d in enumerate(data[1].split(',')) if d != 'x']
# ids = [ids[0]] + sorted(ids[1:], key=lambda x: x[0])
print(ids)

slope = ids[0][0]
value = ids[0][0]
first = None
for i in range(1, len(ids)):
    print("Working on", ids[i])
    while True:
        if (value + ids[i][1]) % ids[i][0] == 0:
            if first:
                slope = value - first
                value = first
                first = None
                break
            else:
                if i == len(ids) - 1:
                    break
                first = value
        value += slope
    print("Found the next step", value, slope)

print("Ending value", value)

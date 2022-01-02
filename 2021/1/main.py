data = [int(d) for d in open('input.txt').read().splitlines()]

prev = data[0]
inc = 0
dec = 0

for x in data[1:]:
    if x > prev:
        inc += 1
    elif x < prev:
        dec += 1
    prev = x

print(inc)

prev = data[0] + data[1] + data[2]
window_size = 3

inc = 0
dec = 0

for i in range(1, len(data) - 2):
    s = sum(data[i:i+window_size])
    if s > prev:
        inc += 1
    elif s < prev:
        dec += 1
    prev = s

print(inc)
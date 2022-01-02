data = open('input.txt').read().split('\n')

max_id = 0
seat_ids = set()

for line in data:
    row = 0
    factor = 64
    for c in line[:7]:
        if c == 'B':
            row += factor
        factor /= 2

    seat = 0
    factor = 4
    for c in line[7:]:
        if c == 'R':
            seat += factor
        factor /= 2

    seat_id = int(row * 8 + seat)
    seat_ids.add(seat_id)
    print(row, seat, seat_id)
    if seat_id > max_id:
        max_id = seat_id

print(max_id)

for i in range(max_id):
    if i not in seat_ids:
        print(i)
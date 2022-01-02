data = open('input.txt').read().splitlines()

positions = [int(d) for d in data[0].split(',')]

min_pos = 0
max_pos = max(positions)

min_fuel = None
for test_pos in range(min_pos, max_pos):
    fuel = sum((abs(p - test_pos) for p in positions))

    if min_fuel is None or fuel < min_fuel:
        min_fuel = fuel

print(min_fuel)

min_fuel = None
for test_pos in range(min_pos, max_pos):
    fuel = sum((sum(range(1, abs(p - test_pos) + 1)) for p in positions))

    if min_fuel is None or fuel < min_fuel:
        min_fuel = fuel

print(min_fuel)
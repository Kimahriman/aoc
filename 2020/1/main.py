data = [int(d) for d in open('input.txt').read().splitlines()]

for i, x in enumerate(data):
    for j, y in enumerate(data):
        if i != j and x + y == 2020:
            print(x * y)

for i, x in enumerate(data):
    for j, y in enumerate(data):
        for k, z in enumerate(data):
            if i != j and i != k and j != k and x + y + z == 2020:
                print(x * y * z)
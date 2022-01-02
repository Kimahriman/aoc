import re

lines = list(filter(None, open('input.txt').read().split('\n')))

dir_map = {
    'e': (1, 0, 0),
    'se': (1, 0, 1),
    'sw': (0, 0, 1),
    'w': (0, 1, 1),
    'nw': (0, 1, 0),
    'ne': (1, 1, 0)
}

tiles = {}
for line in lines:
    coord = (0, 0, 0)
    for step in re.finditer(r'nw|ne|sw|se|e|w', line):
        offset = dir_map[step[0]]
        coord = coord[0] + offset[0], coord[1] + offset[1], coord[2] + offset[2]

    min_val = min(coord)
    coord = coord[0] - min_val, coord[1] - min_val, coord[2] - min_val
    tiles[coord] = not tiles.get(coord, False)

count = 0
for tile in tiles.values():
    if tile:
        count += 1

print(count)


def get_max():
    return max((x for i, j, k in tiles
                for x in (i, j, k) if tiles.get((i, j, k), False))) + 1


max_dist = get_max()


def print_tiles():
    for coord in sorted(tiles):
        print(coord, tiles[coord])


print_tiles()


def normalize(i, j, k):
    min_val = min(i, j, k)
    return i - min_val, j - min_val, k - min_val


def update_tile(i, j, k):
    count = 0
    for _i, _j, _k in dir_map.values():
        if tiles.get(normalize(i + _i, j + _j, k + _k), False):
            count += 1

    color = tiles.get((i, j, k), False)
    if color and (count == 0 or count > 2):
        return False
    if not color and count == 2:
        return True
    return color


for _ in range(100):
    new_tiles = {}
    for x in range(max_dist + 1):
        for y in range(max_dist + 1):
            for i, j, k in ((x, y, 0), (x, 0, y), (0, x, y)):
                new_tiles[i, j, k] = update_tile(i, j, k)

            if x != y:
                for i, j, k in ((y, x, 0), (y, 0, x), (0, y, x)):
                    new_tiles[i, j, k] = update_tile(i, j, k)

    tiles = new_tiles
    max_dist = get_max()
    count = 0
    for tile in tiles.values():
        if tile:
            count += 1
    print(count)
    # print_tiles()

count = 0
for tile in tiles.values():
    if tile:
        count += 1

print(count)
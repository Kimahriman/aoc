import re

tile_strings = open('input.txt').read().strip().split('\n\n')

pattern_map = {}
tiles = {}
tile_patterns = {}

for t in tile_strings:
    id_line = t.split('\n')[0]
    tile_id = re.match(r'Tile (\d+):', id_line)[1]
    # print(tile_id)

    tile_lines = t.split('\n')[1:]
    tile_patterns[tile_id] = tile_lines

    top = tile_lines[0]
    bottom = tile_lines[-1][::-1]
    left = ''.join(reversed([l[0] for l in tile_lines]))
    right = ''.join([l[-1] for l in tile_lines])

    for pattern in (top, right, bottom, left):
        reverse_pattern = ''.join(reversed(pattern))
        pattern_map.setdefault(pattern, set()).add(tile_id)
        pattern_map.setdefault(reverse_pattern, set()).add(tile_id)
        tiles.setdefault(tile_id, []).append(pattern)

for pattern, ids in pattern_map.items():
    if len(ids) > 2:
        print(ids)

total = 1
corners = []
edges = []
for tile, patterns in tiles.items():
    count = 0
    for pattern in patterns:
        reverse_pattern = ''.join(reversed(pattern))
        if len(pattern_map[pattern]) > 1 or len(
                pattern_map[reverse_pattern]) > 1:
            count += 1

    if count < 3:
        print(tile)
        total *= int(tile)
        corners.append(tile)
    if count == 3:
        edges.append(tile)

print(len(tiles), len(edges), total)

oriented = {}
for i in range(4):
    first = pattern_map[tiles[corners[0]][i]]
    second = pattern_map[tiles[corners[0]][(i + 1) % 4]]
    if len(first) > 1 and len(second) > 1:
        print('Orienting corning', first, second, i + 3)
        oriented[0, 0] = (corners[0], (i + 3) % 4)
        # print(tiles[corners[0]][(i + 3) % 4])
        break

for t in tiles[corners[0]]:
    print(t)

size = 12


def transpose_edges(edges):
    return [edges[3][::-1], edges[2][::-1], edges[1][::-1], edges[0][::-1]]


for y in range(size):
    for x in range(size):
        if x == 0 and y == 0:
            continue

        tile = None
        if x > 0:
            left_tile_id, left_tile_orientation = oriented[y, x - 1]
            # print('Attempting', x, y, left_tile_id, left_tile_orientation)
            left_tile_edges = tiles[left_tile_id]
            # print('Left edges', left_tile_edges)
            if left_tile_orientation < 0:
                left_tile_edges = transpose_edges(left_tile_edges)
            # print('Left edges', left_tile_edges)
            right_edge = left_tile_edges[(abs(left_tile_orientation) + 1) % 4]
            # print('Right edge', right_edge)
            tile = pattern_map[right_edge] - {left_tile_id}
            assert len(tile) == 1, (tile, left_tile_id,
                                    pattern_map[right_edge])
            tile = tile.pop()
            orientation = None
            # for t in tiles[tile]:
            # print(t)
            for i, edge in enumerate(tiles[tile]):
                # print('Testing', edge[::-1])
                if edge[::-1] == right_edge:
                    orientation = (i + 1) % 4
                    break

            if orientation is None:
                for i, edge in enumerate(transpose_edges(tiles[tile])):
                    # print('Testing', edge[::-1])
                    if edge[::-1] == right_edge:
                        orientation = -(i + 1)
                        break

            assert orientation is not None
            # print('Orienting', tile, x, y, orientation)
            oriented[y, x] = (tile, orientation)
        else:
            top_tile_id, top_tile_orientation = oriented[y - 1, x]
            # print('Attempting', x, y, left_tile_id, left_tile_orientation)
            top_tile_edges = tiles[top_tile_id]
            # print('Left edges', left_tile_edges)
            if top_tile_orientation < 0:
                top_tile_edges = transpose_edges(top_tile_edges)
            # print('Left edges', left_tile_edges)
            bottom_edge = top_tile_edges[(abs(top_tile_orientation) + 2) % 4]
            # print('Right edge', right_edge)
            tile = pattern_map[bottom_edge] - {top_tile_id}
            assert len(tile) == 1, (tile, top_tile_id,
                                    pattern_map[bottom_edge])
            tile = tile.pop()
            orientation = None
            # for t in tiles[tile]:
            # print(t)
            for i, edge in enumerate(tiles[tile]):
                # print('Testing', edge[::-1])
                if edge[::-1] == bottom_edge:
                    orientation = i
                    break

            if orientation is None:
                for i, edge in enumerate(transpose_edges(tiles[tile])):
                    # print('Testing', edge[::-1])
                    if edge[::-1] == bottom_edge:
                        orientation = -4 if i == 0 else -i  # need to handle -0 == -4
                        break

            assert orientation is not None
            # print('Orienting', tile, x, y, orientation)
            oriented[y, x] = (tile, orientation)

print(oriented.items())
# for (x, y), (tile, ore) in oriented.items():
#     label = 'middle'
#     if tile in corners:
#         label = 'corner'
#     elif tile in edges:
#         label = 'edge'
#     print(x, y, label)


def rotate_tile(tile_content):
    return list(zip(*tile_content[::-1]))


def transpose_tile(tile_content):
    new_tile = {}
    for i, line in enumerate(tile_content):
        for j, c in enumerate(line):
            new_tile[j, i] = c

    return [
        ''.join(new_tile[i, j] for j in range(len(tile_content[0])))
        for i in range(len(tile_content))
    ]


def strip_edges(tile_content):
    width = len(tile_content[0])
    height = len(tile_content)
    return [
        ''.join(tile_content[i][j] for j in range(1, width - 1))
        for i in range(1, height - 1)
    ]


stiched = {}
for i in range(size):
    for j in range(size):
        tile, orientation = oriented[i, j]
        tile_pattern = tile_patterns[tile]
        if orientation < 0:
            tile_pattern = transpose_tile(tile_pattern)

        for _ in range(4 - abs(orientation)):
            tile_pattern = rotate_tile(tile_pattern)

        tile_pattern = strip_edges(tile_pattern)
        for k, line in enumerate(tile_pattern):
            index = i * len(tile_pattern) + k
            if index not in stiched:
                stiched[index] = ''
            stiched[index] += ''.join(line)

stiched = [stiched[i] for i in range(len(stiched))]
for line in stiched:
    print(line)

monster = [(0, 18), (1, 0), (1, 5), (1, 6), (1, 11), (1, 12), (1, 17), (1, 18),
           (1, 19), (2, 1), (2, 4), (2, 7), (2, 10), (2, 13), (2, 16)]

max_count = 0


def check_for_monster(row, col):
    global max_count
    count = 0
    for y, x in monster:
        if stiched[row + y][col + x] != '#':
            pass
        else:
            count += 1
    if count > max_count:
        max_count = count
        print('Found new max count', count)
    return count == len(monster)


monster_count = 0
for rot in range(4):
    for row in range(len(stiched) - 3):
        for col in range(len(stiched[row]) - 20):
            if check_for_monster(row, col):
                print('Found monster at orientation', rot)
                monster_count += 1

    stiched = rotate_tile(stiched)

stiched = transpose_tile(stiched)

for rot in range(4):
    for row in range(len(stiched) - 3):
        for col in range(len(stiched[row]) - 20):
            if check_for_monster(row, col):
                print('Found monster at transposed orientation', rot)

    stiched = rotate_tile(stiched)

total = len([val for line in stiched for val in line if val == '#'])
total -= (monster_count * len(monster))
print(total)
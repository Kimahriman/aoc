import itertools
lines = list(filter(None, open('input.txt').read().split('\n')))

grid = {}
height = 0, len(lines)
width = 0, len(lines[0])
depth = 0, 1
fourth = 0, 1

for row, line in enumerate(lines):
    for col, c in enumerate(line):
        grid[row, col, 0, 0] = c == '#'


def check_spot(g, x, y, z, w):
    count = 0
    for offset in itertools.product(*((-1, 0, 1), ) * 4):
        if offset == (0, 0, 0, 0):
            continue
        if g.get((y + offset[1], x + offset[0], z + offset[2], w + offset[3]),
                 False):
            count += 1
    return count


for _ in range(6):
    height = height[0] - 1, height[1] + 1
    width = width[0] - 1, width[1] + 1
    depth = depth[0] - 1, depth[1] + 1
    fourth = fourth[0] - 1, fourth[1] + 1
    new_grid = {}

    for x in range(*width):
        for y in range(*height):
            for z in range(*depth):
                for w in range(*fourth):
                    count = check_spot(grid, x, y, z, w)
                    active = grid.get((y, x, z, w), False)
                    if active and (count < 2 or count > 3):
                        active = False
                    elif not active and count == 3:
                        active = True
                    new_grid[y, x, z, w] = active

    grid = new_grid

print(len(list(filter(None, grid.values()))))

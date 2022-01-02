data = list(filter(None, open('input.txt').read().split('\n')))

width = len(data[0])
height = len(data)

grid = {}
offsets = ((-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1,
                                                                          1))


def print_grid(g):
    for row in range(height):
        for col in range(width):
            if (row, col) in grid:
                print('#' if grid[row, col] else 'L', end='')
            else:
                print('.', end='')
        print()


for row, line in enumerate(data):
    for col, c in enumerate(line):
        if c == 'L':
            grid[row, col] = False

print_grid(grid)


def occupied(grid, row, col, slope):
    row += slope[0]
    col += slope[1]
    while row >= 0 and col >= 0 and row < height and col < width:
        if (row, col) in grid:
            return grid[row, col]

        row += slope[0]
        col += slope[1]
    return False


changed = True
while changed:
    changed = False

    new_grid = {}
    for row in range(height):
        for col in range(width):
            if (row, col) in grid:
                occ = len(
                    list(
                        filter(None, (occupied(grid, row, col, slope)
                                      for slope in offsets))))
                # print(grid[row, col], occupied)
                if grid[row, col] and occ >= 5:
                    new_grid[row, col] = False
                    changed = True
                elif not grid[row, col] and occ == 0:
                    new_grid[row, col] = True
                    changed = True
                else:
                    new_grid[row, col] = grid[row, col]

    grid = new_grid
    # print()
    # print_grid(grid)
    # input()

count = len(list(filter(None, grid.values())))
print(count)
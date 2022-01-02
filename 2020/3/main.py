data = list(filter(None, open('input.txt').read().split('\n')))

width = len(data[0])
print(data[0])
height = len(data)
grid = {}

for row, line in enumerate(data):
    for col, c in enumerate(line):
        grid[row, col] = c == '#'

row = 1
col = 3
trees = 0
while row < height:
    print(row, col, grid[row, col])
    if grid[row, col]:
        trees += 1

    row += 1
    col += 3
    if col >= width:
        col = col % width

print(trees)

slopes = ((1, 1), (3, 1), (5, 1), (7, 1), (1, 2))
total = 1
for right, down in slopes:
    row = down
    col = right
    trees = 0
    while row < height:
        if grid[row, col % width]:
            trees += 1

        row += down
        col += right

    total *= trees

print(total)
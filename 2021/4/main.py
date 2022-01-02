import re

data = open('input.txt').read().splitlines()

orig_numbers = data[0].split(',')
numbers = orig_numbers
data = data[2:]

boards = [[re.split(r'\s+', l.strip()) for l in data[d:d+5]] for d in range(0, len(data), 6)]

called_numbers = set()

def check_board(board):
    for c in range(len(board)):
        count = 0
        for r in range(len(board)):
            if board[c][r] in called_numbers:
                count += 1
        if count == 5:
            return True
    
    for r in range(len(board)):
        count = 0
        for c in range(len(board)):
            if board[c][r] in called_numbers:
                count += 1
        if count == 5:
            return True
    
    return False

found = False
while not found:
    number = numbers[0]
    called_numbers.add(number)
    numbers = numbers[1:]


    for board in boards:
        if check_board(board):
            total = sum((int(cell) for row in board for cell in row if cell not in called_numbers))
            print(total * int(number))
            found = True
            break

found = False
numbers = orig_numbers
called_numbers.clear()
while not found:
    number = numbers[0]
    called_numbers.add(number)
    numbers = numbers[1:]

    new_boards = [board for board in boards if not check_board(board)]
    if len(new_boards) == 0:
        total = sum((int(cell) for row in boards[0] for cell in row if cell not in called_numbers))
        print(total, number, total * int(number))
        break
    boards = new_boards
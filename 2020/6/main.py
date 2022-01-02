data = open('input.txt').read().split('\n')

total = 0
answers = set()
for line in data:
    if line:
        for c in line:
            answers.add(c)
    else:
        total += len(answers)
        answers = set()

print(total)

total = 0
first = True
answers = set()
for line in data:
    if line:
        if first:
            answers = set(line)
            first = False
        else:
            answers &= set(line)
    else:
        total += len(answers)
        answers = set()
        first = True

print(total)
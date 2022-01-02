import re

with open('input.txt') as f:
    valid = 0
    valid2 = 0
    for line in f:
        match = re.match(r'(\d+)-(\d+) (\w): (\w+)', line)
        low, high, letter, password = int(match[1]), int(
            match[2]), match[3], match[4]
        count = 0
        for l in password:
            if l == letter:
                count += 1

        if count >= low and count <= high:
            valid += 1

        if (password[low - 1] == letter or password[high - 1]
                == letter) and password[low - 1] != password[high - 1]:
            valid2 += 1

    print(valid, valid2)

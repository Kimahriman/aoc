import re


def height(x):
    match = re.match(r'(\d+)(in|cm)', x)
    if not match:
        return False

    if match[2] == 'in':
        return 59 <= int(match[1]) <= 76
    else:
        return 150 <= int(match[1]) <= 193


fields = {
    'byr': lambda x: len(x) == 4 and 1920 <= int(x) <= 2002,
    'iyr': lambda x: len(x) == 4 and 2010 <= int(x) <= 2020,
    'eyr': lambda x: len(x) == 4 and 2020 <= int(x) <= 2030,
    'hgt': height,
    'hcl': lambda x: bool(re.match(r'^#[0-9a-z]{6}$', x)),
    'ecl': lambda x: x in ('amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'),
    'pid': lambda x: bool(re.match(r'^\d{9}$', x))
}

present = 0
valid = 0
present_fields = set()
valid_fields = set()
valid_values = set()
for line in open('input.txt').read().split('\n'):
    if line:
        for val in line.split(' '):
            field = val.split(':')[0]
            value = val.split(':')[1]
            present_fields.add(field)
            if fields.get(field, lambda x: True)(value):
                valid_fields.add(field)
                valid_values.add(val)

    else:
        if all((f in present_fields for f in fields)):
            present += 1
        if all((f in valid_fields for f in fields)):
            print("VALID\n")
            print(valid_values)
            valid += 1

        present_fields = set()
        valid_fields = set()
        valid_values = set()

print(present, valid)
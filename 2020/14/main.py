import re

data = open('input.txt').read().splitlines()

one_mask = 0
zero_mask = 0
mem = {}
for line in data:
    mask_line = re.match(r'mask = (.*)', line)
    if mask_line:
        one_mask = int(mask_line[1].replace('X', '0'), 2)
        zero_mask = int(mask_line[1].replace('X', '1'), 2)
        pass
    else:
        mem_line = re.match(r'mem\[(\d+)\] = (\d+)', line)
        value = int(mem_line[2])
        value &= zero_mask
        value |= one_mask
        mem[mem_line[1]] = value

print(sum(mem.values()))


def get_addresses(address, mask, pos=0):
    if pos >= len(mask):
        yield int(address, 2)
    elif mask[pos] == '1' and address[pos] != '1':
        yield from get_addresses(address[:pos] + '1' + address[pos + 1:], mask,
                                 pos + 1)
    elif mask[pos] == 'X':
        yield from get_addresses(address[:pos] + '0' + address[pos + 1:], mask,
                                 pos + 1)
        yield from get_addresses(address[:pos] + '1' + address[pos + 1:], mask,
                                 pos + 1)
    else:
        yield from get_addresses(address, mask, pos + 1)


mask = None
mem = {}
for line in data:
    mask_line = re.match(r'mask = (.*)', line)
    if mask_line:
        mask = mask_line[1]
        pass
    else:
        mem_line = re.match(r'mem\[(\d+)\] = (\d+)', line)
        address = f'{int(mem_line[1]):036b}'
        value = int(mem_line[2])
        for a in get_addresses(address, mask):
            mem[a] = value

print(sum(mem.values()))

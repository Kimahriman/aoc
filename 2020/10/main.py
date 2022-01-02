import itertools

nums = list(
    map(lambda x: int(x), filter(None,
                                 open('input.txt').read().split('\n'))))

nums = sorted(nums)
nums = [0] + nums + [nums[-1] + 3]

ones = 0
threes = 1

if nums[0] == 1:
    ones += 1
else:
    threes += 1

for i in range(len(nums) - 1):
    if nums[i + 1] - nums[i] == 1:
        ones += 1
    else:
        threes += 1

print(ones * threes)

cache = {}


def validate(segment):
    for i in range(len(segment) - 1):
        if segment[i + 1] - segment[i] > 3:
            return False

    return True


def count_valid(segment):
    # print('Counting valid for', segment)
    # if len(segment) in cache:
    #     return cache[len(segment)]

    count = 0
    for i in range(len(segment) - 1):
        for combo in itertools.combinations(segment[1:-1], i):
            if validate([segment[0]] + list(combo) + [segment[-1]]):
                # print('Valid', [segment[0]] + list(combo) + [segment[-1]])
                count += 1

    cache[len(segment)] = count
    # print('Found', count)
    return count


end_index = 0
index = 0
total = 1
while index < len(nums) - 1:
    end_index = index
    while end_index < len(nums) - 1 and nums[end_index +
                                             1] - nums[end_index] == 1:
        end_index += 1

    if end_index - index >= 2:
        total *= count_valid(nums[index:end_index + 1])
        index = end_index + 1
    else:
        index += 1

print(total)
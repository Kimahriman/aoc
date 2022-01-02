nums = list(
    map(lambda x: int(x), filter(None,
                                 open('input.txt').read().split('\n'))))

index = 0
weak = 0
for i, num in enumerate(nums):
    if i < 25:
        continue

    prev = nums[i - 25:i]
    found = False
    for x in prev:
        for y in prev:
            if x + y == num and x != y:
                found = True
                break
        if found:
            break

    if not found:
        print(num)
        weak = num
        break

index = 0
size = 2
while True:
    total = sum(nums[index:index + size])
    if total < weak:
        size += 1
    if total == weak:
        break
    if total > weak:
        size = 2
        index += 1

print(min(*nums[index:index + size]) + max(*nums[index:index + size]))

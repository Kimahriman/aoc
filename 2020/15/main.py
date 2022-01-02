numbers = [1, 0, 16, 5, 17, 4]
# numbers = [0, 3, 6]

mem = {}

index = 0
for num in numbers[:-1]:
    mem[num] = index
    index += 1

last_num = numbers[-1]
while index < 30000000 - 1:
    if last_num in mem:
        new_last_num = index - mem[last_num]
    else:
        new_last_num = 0
    # print("new num", new_last_num)
    mem[last_num] = index
    last_num = new_last_num
    index += 1

print(last_num)

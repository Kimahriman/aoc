data = open('input.txt').read().splitlines()

all_fish = [int(d) for d in data[0].split(',')]

fish_map_first = {}

for i in range(9):
    days = 128
    fish = [i]
    while days > 0:
        new_fish = []
        for j in range(len(fish)):
            if fish[j] == 0:
                fish[j] = 6
                new_fish.append(8)
            else:
                fish[j] -= 1
        
        fish = fish + new_fish
        days -= 1
    
    fish_map_first[i] = fish

fish_map_second = {}

for i in range(7):
    total = 0
    for f in fish_map_first[i]:
        total += len(fish_map_first[f])
    fish_map_second[i] = total

    
print(fish_map_second)
print(sum((fish_map_second[x] for x in all_fish)))

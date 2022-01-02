import re

allergen_map = {}
ingredients = []

for line in open('input.txt'):
    ings = re.match(r'^(.*) \(', line)[1].split(' ')
    ingredients.append(ings)
    allergens = re.match(r'.*\(contains (.*)\)', line)
    # print(allergens, line)
    if allergens:
        for allergen in allergens[1].split(', '):
            # print('Found allergen', allergen, ings)
            if allergen not in allergen_map:
                allergen_map[allergen] = set(ings)
            else:
                allergen_map[allergen] &= set(ings)

print(allergen_map)

while True:
    changed = False
    for allergen in allergen_map:
        if len(allergen_map[allergen]) == 1:
            ing = list(allergen_map[allergen])[0]
            for allergen2 in allergen_map:
                if allergen != allergen2 and ing in allergen_map[allergen2]:
                    changed = True
                    allergen_map[allergen2].remove(ing)

    if not changed:
        break

print(allergen_map)

count = 0
for ingredient_list in ingredients:
    for ingredient in ingredient_list:
        if not any(
            (ingredient in all_ing for all_ing in allergen_map.values())):
            count += 1

print(count)

print(','.join(
    (allergen_map[allergen].pop() for allergen in sorted(allergen_map))))

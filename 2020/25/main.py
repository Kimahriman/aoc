pub1 = 1717001
pub2 = 523731

subject = 7
mod = 20201227

card_loop = 0
card_value = 1
while card_value != pub1:
    card_value = (card_value * subject) % mod
    card_loop += 1

door_loop = 0
door_value = 1
while door_value != pub2:
    door_value = (door_value * subject) % mod
    door_loop += 1

print(card_loop, door_loop)

key1 = 1
for _ in range(card_loop):
    key1 = (key1 * pub2) % mod

key2 = 1
for _ in range(door_loop):
    key2 = (key2 * pub1) % mod

print(key1, key2)
cups = [int(c) for c in '653427918']
cups += list(range(10, 1000001))
size = len(cups)


class Node:
    def __init__(self, value, _next=None):
        self.value = value
        self.next = _next


current = Node(cups[0], None)
reverse_map = {cups[0]: current}
prev = current
for cup in cups[1:]:
    prev.next = Node(cup, None)
    prev = prev.next
    reverse_map[prev.value] = prev

prev.next = current


def collect(size):
    node = current
    for _ in range(size):
        yield node.value
        node = node.next


def move():
    global current

    pickup = []
    node = current
    for _ in range(3):
        pickup.append(node.next)
        node.next = node.next.next

    # print('Picked up', [p.value for p in pickup], current.value)

    dest = (current.value - 2) % size + 1
    while any((dest == p.value for p in pickup)):
        dest = (dest - 2) % size + 1

    dest_node = reverse_map[dest]
    for p in pickup:
        old_next = dest_node.next
        dest_node.next = p
        dest_node = dest_node.next
        dest_node.next = old_next

    current = current.next
    # print(list(collect(size)))
    node = current


#     global cups
#     pickup_index = cups.index(current) + 1
#     pickup = []
#     for _ in range(3):
#         if pickup_index >= len(cups):
#             pickup_index = 0
#         pickup.append(cups.pop(pickup_index))

#     dest = (current - 2) % size + 1
#     while dest not in cups:
#         dest = (dest - 2) % size + 1

#     dest_index = cups.index(dest)
#     cups = cups[:dest_index + 1] + pickup + cups[dest_index + 1:]
#     current = cups[(cups.index(current) + 1) % len(cups)]
#     # print(cups)
#     # print()

for i in range(10000000):
    move()
    if i % 100000 == 0:
        print('Finished', i)

cup_one = reverse_map[1]
print(cup_one.next.value * cup_one.next.next.value)

# print(list(collect(size)))

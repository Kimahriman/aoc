lines = open('input.txt').read().split('\n')

iterator = iter(lines)

player1 = []
player2 = []

next(iterator)
for line in iterator:
    if not line:
        break

    player1.append(int(line))

next(iterator)
for line in iterator:
    if not line:
        break

    player2.append(int(line))

print(player1)
print(player2)


def play_game(deck1, deck2):
    seen_hands = set()
    while len(deck1) > 0 and len(deck2) > 0:
        # print(deck1)
        # print(deck2)
        hand = (tuple(deck1), tuple(deck2))
        if hand in seen_hands:
            return True
        seen_hands.add(hand)
        deck1_card = deck1.pop(0)
        deck2_card = deck2.pop(0)

        winner = None
        if deck1_card <= len(deck1) and deck2_card <= len(deck2):
            winner = play_game(deck1[:deck1_card], deck2[:deck2_card])
        else:
            winner = deck1_card > deck2_card

        if winner:
            deck1.append(deck1_card)
            deck1.append(deck2_card)
        else:
            deck2.append(deck2_card)
            deck2.append(deck1_card)

    return len(deck1) > 0


play_game(player1, player2)

score = 0
for i, card in enumerate(reversed(player1 if player1 else player2)):
    score += (i + 1) * card

print(score)
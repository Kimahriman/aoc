const fs = require('fs')

const data = fs.readFileSync('input.txt').toString()

const [_, numPlayers, numMarbles] = data.match(/(\d+) players; last marble is worth (\d+) points/)
console.log(numPlayers, numMarbles)

const players = {}
let current = { value: 0 }
current.next = current
current.prev = current

function remove(node) {
	node.prev.next = node.next
	node.next.prev = node.prev
}

for (let i=1; i <= numMarbles; i++) {
	if (i % 23 === 0) {
		const player = i % numPlayers
		players[player] = (players[player] || 0) + i
		for (let j=0; j < 7; j++) {
			current = current.prev
		}
		remove(current)
		players[player] += current.value
		current = current.next
	} else {
		current = current.next
		const newNode = { value: i, prev: current, next: current.next }
		current.next = newNode
		newNode.next.prev = newNode
		current = newNode
	}
}

const maxScore = Object.values(players).reduce((acc, p) => p > acc ? p : acc, 0)

console.log(maxScore)

// Part 2

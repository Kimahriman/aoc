const fs = require('fs')

const data = fs.readFileSync('input.txt').toString()

const sum = data.trim().split('\n')
	.map(d => parseInt(d))
	.reduce((acc, d) => acc + d, 0)

console.log('Sum', sum)

// Part 2

const seenFreqs = new Set()

const changes = data.trim().split('\n').map(d => parseInt(d))

let index = 0
let freq = 0

while (true) {
	freq += changes[index]
	if (seenFreqs.has(freq)) {
		console.log("First repeated is", freq)
		break
	}

	seenFreqs.add(freq)

	index++
	index %= changes.length
}


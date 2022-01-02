const fs = require('fs')

const data = fs.readFileSync('input.txt').toString()

const res = data.split('\n')
	.reduce((acc, d) => {
		const counts = {}
		d.split('').forEach(c => {
			counts[c] = (counts[c] || 0) + 1
		})
		const vals = Object.values(counts)
		if (vals.some(v => v === 2)) {
			acc.twos += 1
		}
		if (vals.some(v => v === 3)) {
			acc.threes += 1
		}
		return acc
	}, { twos: 0, threes: 0 })

console.log(res.twos, res.threes, res.twos * res.threes)

// Part 2

const idMap = {}

data.trim().split('\n').forEach(d => {
	for (let i=0; i < d.length; i++) {
		const sub = d.slice(0, i) + d.slice(i+1)
		if (!idMap[i]) {
			idMap[i] = {}
		}
		if (idMap[i][sub]) {
			console.log("Found match!", sub, d, idMap[i][sub])
		}
		idMap[i][sub] = d
	}
})
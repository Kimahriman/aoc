const fs = require('fs')

const data = fs.readFileSync('input.txt').toString()

const [sampleData, program] = data.split('\n\n\n')

const sampleRegex = /Before: \[(\d+), (\d+), (\d+), (\d+)\]\n(\d+) (\d+) (\d+) (\d+)\nAfter:  \[(\d+), (\d+), (\d+), (\d+)\]/g

const sampleOps = []
let match
while (match = sampleRegex.exec(sampleData)) {
	const startReg = match.slice(1, 5).map(d => parseInt(d))
	const op = match.slice(5, 9).map(d => parseInt(d))
	const endReg = match.slice(9, 13).map(d => parseInt(d))

	sampleOps.push([startReg, op, endReg])
}

const replace = (arr, i, val) => {
	const copy = [...arr]
	copy[i] = val
	return copy
}

const ops = [
	(regs, a, b, c) => replace(regs, c, regs[a] + regs[b]),
	(regs, a, b, c) => replace(regs, c, regs[a] + b),
	(regs, a, b, c) => replace(regs, c, regs[a] * regs[b]),
	(regs, a, b, c) => replace(regs, c, regs[a] * b),
	(regs, a, b, c) => replace(regs, c, regs[a] & regs[b]),
	(regs, a, b, c) => replace(regs, c, regs[a] & b),
	(regs, a, b, c) => replace(regs, c, regs[a] | regs[b]),
	(regs, a, b, c) => replace(regs, c, regs[a] | b),
	(regs, a, b, c) => replace(regs, c, regs[a]),
	(regs, a, b, c) => replace(regs, c, a),
	(regs, a, b, c) => replace(regs, c, a > regs[b] ? 1 : 0),
	(regs, a, b, c) => replace(regs, c, regs[a] > b ? 1 : 0),
	(regs, a, b, c) => replace(regs, c, regs[a] > regs[b] ? 1 : 0),
	(regs, a, b, c) => replace(regs, c, a === regs[b] ? 1 : 0),
	(regs, a, b, c) => replace(regs, c, regs[a] === b ? 1 : 0),
	(regs, a, b, c) => replace(regs, c, regs[a] === regs[b] ? 1 : 0)
]

function equal(arr1, arr2) {
	for (let i=0; i < arr1.length; i++) {
		if (arr1[i] !== arr2[i]) {
			return false
		}
	}
	return true
}

function valid(sample, op) {
	const [opNum, a, b, c] = sample[1]
	const res = op(sample[0], a, b, c)
	return equal(sample[2], res)
}

const sampleCount = sampleOps.reduce((acc, sample) => {
	const opCount = ops.reduce((a, o) => valid(sample, o) ? a + 1 : a, 0)
	return opCount >= 3 ? acc + 1 : acc
}, 0)

console.log(sampleCount)

// Part 2

function intersection(set1, set2) {
	if (!set1) {
		return set2
	} else if (!set2) {
		return set1
	}
	const inter = new Set()
	set1.forEach(val => {
		if (set2.has(val)) {
			inter.add(val)
		}
	})
	return inter
}

let opNumTracker = sampleOps.reduce((acc, sample) => {
	const possible = ops.reduce((acc2, op, i) => valid(sample, op) ? acc2.add(i) : acc2, new Set())
	// if (sample[1][0] === 0) {
	// 	console.log(sample[0], sample[1], sample[2], acc[sample[1][0]], possible)
	// 	console.log(intersection(acc[sample[1][0]], possible))
	// }
	acc[sample[1][0]] = intersection(acc[sample[1][0]], possible)
	return acc
}, [])

const found = new Set(opNumTracker.filter(x => x.size === 1).map(x => [...x][0]))

console.log("Found", found)

while (opNumTracker.some(x => x.size > 1)) {
	opNumTracker.forEach(x => {
		if (x.size === 1) {
			return
		}

		found.forEach(f => {
			x.delete(f)
		})

		if (x.size === 1) {
			found.add([...x][0])
		}
	})

	// console.log(opMap)
	// console.log(opNumTracker.map((k, i) => [i, k]))
}

opMap = opNumTracker.map(x => [...x][0])

console.log(opMap)

const instructions = program.trim().split('\n')
	.map(line => line.match(/(\d+) (\d+) (\d+) (\d+)/))
	.map(parsed => parsed.slice(1, 5).map(x => parseInt(x)))

let regs = [0, 0, 0, 0]

instructions.forEach(inst => {
	const [opCode, a, b, c] = inst
	const op = ops[opMap[opCode]]
	regs = op(regs, a, b, c)
})

console.log(regs)

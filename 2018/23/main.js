const fs = require('fs')
const TinyQueue = require('tinyqueue')

const data = fs.readFileSync('input.txt').toString()

const bots = data.trim().split('\n')
	.map(line => line.match(/pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)/))
	.map(p => p.slice(1, 5).map(d => parseInt(d)))
	.map(([x, y, z, r]) => ({ pos: [x, y, z], r}))

const strongest = bots.reduce((acc, b) => (!acc || b.r > acc.r) ? b : acc, null)



// Part 2

function distance(a, b) {
	return Math.abs(a[0] - b[0]) + Math.abs(a[1] - b[1]) + Math.abs(a[2] - b[2])
}

function valid(a, b, buffer=0) {
	// console.log("Valid", a, b, distance(a.pos, b), buffer, a.r)
	return distance(a.pos, b) - buffer <= a.r
}

const inRange = bots.filter(b => valid(strongest, b.pos))

console.log("In range", inRange.length)

function insert(queue, box) {
	const index = queue.findIndex(v => {
		// return box.bots === v.bots
		// 	? box.dist === v.dist
		// 		? box.r < v.r
		// 		: box.dist < v.dist
		// 	: box.bots > v.bots
		return box.bots >= v.bots
	})
	if (index === -1) {
		queue.push(box)
	} else {
		queue.splice(index, 0, box)
	}
}

function getOcta(pos, r) {
	const inRange = bots.filter(b => valid(b, pos, r)).length
	return {
		pos,
		r,
		bots: inRange,
		dist: Math.max(distance(pos, origin) - r, 0)
	}
}

const origin = [0, 0, 0]
const steps = [
	[-1, 0, 0],
	[0, -1, 0],
	[0, 0, -1],
	[1, 0, 0],
	[0, 1, 0],
	[0, 0, 1]
]

function split(pos, r) {
	if (r > 2) {
		const newRadius = Math.ceil(2 * r / 3)
		return steps.map(step => {
			const newPos = pos.map((v, i) => v + (step[i] * Math.floor(r / 3)))
			return getOcta(newPos, newRadius)
		})
	} else {
		const newOctas = []
		for (let i=-r; i <= r; i++) {
			for (let j=-(r-Math.abs(i)); j <= r-Math.abs(i); j++) {
				for (let k=-(r-Math.abs(i)-Math.abs(j)); k <= r-Math.abs(i)-Math.abs(j); k++) {
					newOctas.push(getOcta([pos[0] + i, pos[1] + j, pos[2] + k], 0))
				}
			}
		}
		return newOctas
	}
}


const startingRadius = bots.reduce((acc, b) => {
	const dist = distance(b.pos, origin)
	return dist > acc ? dist : acc
}, 0)
console.log("Starting radius", startingRadius)
// const queue = [getOcta(origin, startingRadius)]
const queue = new TinyQueue([getOcta(origin, startingRadius)], (a, b) => b.bots - a.bots)
console.log(queue)
let count = 0

while (true) {
	count++
	const current = queue.pop()
	if (count % 10000 === 0) {
		console.log("Gone through", count, "In queue", queue.length, current.r, current.bots)
	}
	// console.log("Current", current.bots, current.r)
	if (current.r === 0) {
		console.log("Found a zero radius with dist", current.dist, "bots", current.bots)
		break
	}
	split(current.pos, current.r).forEach(x => {
		// console.log("Inserting", x)
		// insert(queue, x)
		queue.push(x)
	})
}

const fs = require('fs')
const keypress = require('keypress')

const data = fs.readFileSync('input.txt').toString()

let maxY = 0

const grid = data.trim().split('\n')
	.reduce((acc, line) => {
		const parsed = line.match(/(x|y)=(\d+), (x|y)=(\d+)\.\.(\d+)/)
		let [_, var1, val1, var2, range1, range2] = parsed

		val1 = parseInt(val1)
		range1 = parseInt(range1)
		range2 = parseInt(range2)
		if (var1 === 'x' && range2 > maxY) {
			maxY = range2
		} else if (var1 === 'y' && val1 > maxY) {
			maxY = val1
		}

		for (let i=range1; i <= range2; i++) {
			if (var1 === 'x') {
				acc[[val1, i]] = 'clay'
			} else {
				acc[[i, val1]] = 'clay'
			}
		}
		return acc
	}, {})

function print(x, y, write) {
	const minX = Math.min(...Object.keys(grid).map(c => parseInt(c.split(',')[0])))
	const maxX = Math.max(...Object.keys(grid).map(c => parseInt(c.split(',')[0])))
	console.log("min/max", minX, maxX)
	for (let j=y-20; j <= y+20; j++) {
		let s = ''
		for (let i=x-60; i <= x+60; i++) {
			const cell = grid[[i, j]]
			s += (i === x && j === y) ? 'x' : !cell ? '.' : cell === 'clay' ? '#' : cell === 'water' ? '|' : '~'
		}
		console.log(s)
	}

	if (write) {
		let data = ''
		for (let j=0; j <= maxY; j++) {
			for (let i=minX; i <= maxX; i++) {
				const cell = grid[[i, j]]
				data += (i === x && j === y) ? 'x' : !cell ? '.' : cell === 'clay' ? '#' : cell === 'water' ? '|' : '~'
			}
			data += '\n'
		}
		fs.writeFileSync('grid.txt', data)
}
}

function fillStill(x, y) {
	// console.log("Filling", x, y)
	const startX = x, startY = y
	while (grid[[x, y]] !== 'clay') {
		grid[[x, y]] = 'still'
		x--
	}
	// console.log("Done filling to left")
	x = startX + 1
	while (grid[[x, y]] !== 'clay') {
		grid[[x, y]] = 'still'
		x++
	}
	// print(startX, startY)
	// console.log("Done filling")
}

const junctions = {}

function flow(x, y, top, dir) {
	// if (Math.abs(x - 519) <= 10 && Math.abs(y - 73) <= 7) {
	// console.log("Flowing to", x, y, dir)
		// print(x, y)
	// }
	

	let below = grid[[x, y+1]]

	while ((!below || below === 'water') && y < maxY) {
		dir = 0
		y++
		grid[[x, y]] = 'water'
		below = grid[[x, y+1]]
	}

	if (y >= maxY) {
		// console.log("Returning passed bottom")
		return
	}

	if (!dir) {
		if (below === 'clay' || below === 'still') {
			const left = grid[[x-1, y]]
			const right = grid[[x+1, y]]

			if (left === 'water' && right === 'water') {
				return
			}
			const leftEnd = flow(x, y, null, -1)
			const rightEnd = flow(x, y, null, 1)

			if (junctions[[x, y]]) {
				console.log("Already seen junction", x, y, top)
			}
			junctions[[x, y]] = true

			if (!leftEnd || !rightEnd) {
				// Fell past max y
				// console.log("Returning sideways infinite flow")
				return
			}

			if (leftEnd.y === y && rightEnd.y === y) {
				// console.log("Left", leftEnd, "Right", rightEnd)
				fillStill(x, y)
				if (top && (top.x !== x || top.y !== y)) {
					return flow(top.x, top.y, top)
				}
			}
			// console.log("Returning combo of flows sideways", x, y)
			return leftEnd.y > rightEnd.y ? leftEnd : rightEnd
		}
	} else {
		let adjacent = grid[[x+dir, y]]
		// console.log("Adjacent", adjacent)
		while (below === 'clay' || below === 'still') {
			if (adjacent === 'clay' || adjacent === 'still') {
				return { x, y }
			}
			x += dir
			grid[[x, y]] = 'water'
			adjacent = grid[[x+dir, y]]
			below = grid[[x, y+1]]
		}
		return flow(x, y, { x, y })
	}
	throw "Shouldn't hit here"
}

const spout = [500, 0]

let oldWater = 0
// print()

// while (true) {
console.log("Flowing", oldWater)
flow(spout[0], spout[1], { x: spout[0], y: spout[1] })


const newWater = Object.values(grid).reduce((acc, cell) => ['water', 'still'].includes(cell) ? acc + 1 : acc, 0)
// if (newWater === oldWater) {
// 	break
// }
console.log(newWater)

const stillWater = Object.values(grid).reduce((acc, cell) => cell === 'still' ? acc + 1 : acc, 0)
console.log("Water left", stillWater)
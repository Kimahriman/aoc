const fs = require('fs')

const data = fs.readFileSync('input.txt').toString()

const coords = data.trim().split('\n')
	.filter(d => d)
	.map(d => d.split(', ').map(x => parseInt(x)))

const size = [Math.max(...coords.map(c => c[0])), Math.max(...coords.map(c => c[1]))]

const infinite = new Set()
let queue = coords.map((c, i) => ({ id: i, dist: 0, x: c[0], y: c[1] }))
let grid = queue.reduce((acc, c) => {
	acc[[c.x, c.y]] = c
	return acc
}, {})

while (queue.length > 0) {
	const coord = queue.shift()

	for (let i=coord.x-1; i <= coord.x+1; i++) {
		for (let j=coord.y-1; j <= coord.y+1; j++) {
			if ((i === coord.x && j === coord.y) || (i !== coord.x && j !== coord.y) || coord.id == -1) {
				continue
			}

			const current = grid[[i, j]]
			if (!current) {
				if (i < 0 || i > size[0] || j < 0 || j > size[1]) {
					infinite.add(coord.id)
					continue
				}

				grid[[i, j]] = { id: coord.id, dist: coord.dist + 1, x: i, y: j }
				queue.push(grid[[i, j]])
			} else {
				if (current.id !== coord.id && current.dist === coord.dist + 1) {
					grid[[i, j]].id = -1
				}
			}
		}
	}
}

const idCounts = Object.values(grid).reduce((acc, coord) => {
	if (coord.id === -1 || infinite.has(coord.id)) {
		return acc
	}
	acc[coord.id] = (acc[coord.id] || 0) + 1
	return acc
})

console.log(Math.max(...Object.values(idCounts)))

// Part 2
queue = coords.map((c, i) => ({ id: i, dist: 0, x: c[0], y: c[1] }))
grid = queue.reduce((acc, c) => {
	acc[[c.x, c.y]] = { [c.id]: 0 }
	return acc
}, {})

while (queue.length > 0) {
	const coord = queue.shift()

	for (let i=coord.x-1; i <= coord.x+1; i++) {
		for (let j=coord.y-1; j <= coord.y+1; j++) {
			if ((i === coord.x && j === coord.y) || (i !== coord.x && j !== coord.y) || coord.id == -1) {
				continue
			}

			if (i < 0 || i > size[0] || j < 0 || j > size[1]) {
				continue
			}

			let current = grid[[i, j]]
			if (!current) {
				current = grid[[i, j]] = {}
			}

			if (current[coord.id] !== undefined) {
				continue
			}

			current[coord.id] = coord.dist + 1
			queue.push({ id: coord.id, dist: coord.dist + 1, x: i, y: j })
		}
	}
}

const maxSum = 10000
const withinRange = Object.values(grid).reduce((acc, coord) => {
	return Object.values(coord).reduce((acc, dist) => acc + dist, 0) < maxSum ? acc + 1 : acc
}, 0)

console.log(`Within ${maxSum}`, withinRange)

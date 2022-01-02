const fs = require('fs')

const data = fs.readFileSync('input.txt').toString()

const parents = []
const grid = {}
let current = [0, 0]

grid[[0, 0]] = { door: false, dist: 0 }

data.replace(/[$^]/, '').trim().split('').forEach(c => {
	const [x, y] = current
	const currentCell = grid[[x, y]]
	let room
	let door
	switch (c) {
		case 'N':
			door = [x, y+1]
			room = [x, y+2]
			break
		case 'E':
			door = [x+1, y]
			room = [x+2, y]
			break
		case 'S':
			door = [x, y-1]
			room = [x, y-2]
			break
		case 'W':
			door = [x-1, y]
			room = [x-2, y]
			break
		case '(':
			parents.push(current)
			break
		case ')':
			current = parents.pop()
			break
		case '|':
			current = parents[parents.length - 1]
			break
	}

	if (room && door) {
		grid[door] = { door: true }
		if (!grid[room]) {
			grid[room] = { door: false, dist: currentCell.dist + 1 }
		} else if (grid[room].dist > currentCell.dist + 1) {
			grid[room].dist = currentCell.dist + 1
		}
		current = room
	}
})

// console.log(grid)
console.log("Parents", parents)
console.log("Furthest", Object.values(grid).reduce((acc, c) => (!c.door && c.dist > acc) ? c.dist : acc, 0))
console.log("Over 1000 away", Object.values(grid).filter(c => !c.door && c.dist >= 1000).length)
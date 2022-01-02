const fs = require('fs')

const data = fs.readFileSync('input.txt').toString()

let grid = data.trim().split('\n')
	.map(line => line.split(''))

const rounds = 1000000000

function getAdjacent(x, y) {
	return [
		...(grid[y-1] || []).slice(Math.max(x-1, 0), x+2),
		grid[y][x-1],
		grid[y][x+1],
		...(grid[y+1] || []).slice(Math.max(x-1, 0), x+2)
	].filter(x => x)
}

function count(list, type) {
	return list.filter(a => a === type).length
}

const cache = {}

for (let r=0; r < rounds; r++) {
	const hash = grid.reduce((acc, row) => acc.concat(row), []).join('')
	if (cache[hash]) {
		console.log("Found repeat between", cache[hash], r)
		r += Math.floor((rounds - r) / (r - cache[hash])) * (r - cache[hash])
	}
	cache[hash] = r
	grid = grid.map((row, j) => {
		return row.map((cell, i) => {
			const adjacent = getAdjacent(i, j)
			if (cell === '.' && count(adjacent, '|') >= 3) {
				return '|'
			}
			if (cell === '|' && count(adjacent, '#') >= 3) {
				return '#'
			}
			if (cell === '#' && (count(adjacent, '#') === 0 || count(adjacent, '|') === 0)) {
				return '.'
			}
			return cell
		})
	})
}

const trees = count(grid.reduce((acc, row) => acc.concat(row), []), '|')
const lumberyards = count(grid.reduce((acc, row) => acc.concat(row), []), '#')

console.log(trees, lumberyards, trees * lumberyards)

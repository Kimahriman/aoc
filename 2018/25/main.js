const fs = require('fs')

const data = fs.readFileSync('input.txt').toString()

const points = data.trim().split('\n')
	.map(line => line.split(',').map(d => parseInt(d)))

const grid = {}
const constellations = {}
let nextId = 1

function getConnections(pos) {
	const [x, y, z, w] = pos
	const connections = []
	for (let i=x-3; i <= x+3; i++) {
		const yd = 3 - Math.abs(x - i)
		for (let j=y-yd; j <= y+yd; j++) {
			const zd = 3 - Math.abs(x - i) - Math.abs(y - j)
			for (let k=z-zd; k <= z+zd; k++) {
				const wd = 3 - Math.abs(x - i) - Math.abs(y - j) - Math.abs(z - k)
				for (let l=w-wd; l <= w+wd; l++) {
					// console.log("Man dist", i, yd, zd, wd, pos, [i, j, k, l], pos.reduce((acc, d, index) => acc + Math.abs(d - [i, j, k, l][index]), 0))
					if (grid[[i, j, k, l]]) {
						connections.push([i, j, k, l])
					}
				}
			}
		}
	}
	return connections
}

points.forEach(p => {
	// console.log("Doing point", p)
	const connections = getConnections(p)
	if (connections.length === 0) {
		grid[p] = nextId
		constellations[nextId] = [p]
		nextId++
	} else {
		const constId = grid[connections[0]]
		// console.log("Setting others to", constId)
		grid[p] = constId
		constellations[constId].push(p)
		connections.forEach(c => {
			if (grid[c] === constId) {
				return
			}
			const oldC = grid[c]
			// console.log(c, oldC, grid[c], constellations[grid[c]], constId, constellations[constId])
			constellations[oldC].forEach(pos => {
				// console.log("Updating existing constellation loc", pos)
				grid[pos] = constId
				constellations[constId].push(pos)
			})
			constellations[oldC] = null
		})
	}
})

console.log("Constellations:", Object.values(constellations).filter(c => c).length)
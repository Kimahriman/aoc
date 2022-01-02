const fs = require('fs')

const data = fs.readFileSync('input.txt').toString()

const re = /#(\d+) @ (\d+),(\d+): (\d+)x(\d+)/
const parsed = data.split('\n')
	.filter(d => d)
	.map(d => d.match(re))
	.map(d => ({
		id: parseInt(d[1]),
		left: parseInt(d[2]),
		top: parseInt(d[3]),
		width: parseInt(d[4]),
		height: parseInt(d[5])
	}))

const fabric = {}
parsed.forEach(({ left, top, width, height }) => {
	for (let i=left; i < left + width; i++) {
		for (let j=top; j < top + height; j++) {
			fabric[[i, j]] = (fabric[[i, j]] || 0) + 1
		}
	}
})

const overlapped = Object.values(fabric).filter(d => d > 1).length

console.log(overlapped)

// Part 2

parsed.forEach(({ id, left, top, width, height }) => {
	for (let i=left; i < left + width; i++) {
		for (let j=top; j < top + height; j++) {
			if (fabric[[i, j]] !== 1) {
				return
			}
		}
	}
	console.log("Found non overlapping section!", id)
})

const fs = require('fs')
const keypress = require('keypress')

const data = fs.readFileSync('input.txt').toString()

const nodes = data.trim().split('\n')
	.map(d => d.match(/position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>/))
	.map(d => ({ pos: { x: parseInt(d[1]), y: parseInt(d[2]) }, vel: { x: parseInt(d[3]), y: parseInt(d[4]) } }))

let time = 0

function tick() {
	time++
	nodes.forEach(node => {
		node.pos.x += node.vel.x
		node.pos.y += node.vel.y
	})
}

function untick() {
	time--
	nodes.forEach(node => {
		node.pos.x -= node.vel.x,
		node.pos.y -= node.vel.y
	})
}

function grid(scale=1) {
	const scaledNodes = nodes.map(node => ({
		x: Math.floor(node.pos.x / scale),
		y: Math.floor(node.pos.y / scale)
	}))
	const map = scaledNodes.reduce((acc, node) => {
		acc[[node.x, node.y]] = true
		return acc
	}, {})
	const minX = scaledNodes.reduce((acc, node) => node.x < acc ? node.x : acc, scaledNodes[0].x)
	const maxX = scaledNodes.reduce((acc, node) => node.x > acc ? node.x : acc, scaledNodes[0].x)
	const minY = scaledNodes.reduce((acc, node) => node.y < acc ? node.y : acc, scaledNodes[0].y)
	const maxY = scaledNodes.reduce((acc, node) => node.y > acc ? node.y : acc, scaledNodes[0].y)

	return {
		map,
		minX,
		maxX,
		minY,
		maxY
	}
}

function print(grid) {
	console.log('==========================================')
	for (let j=grid.minY; j <= grid.maxY; j++) {
		let s = ''
		for (let i=grid.minX; i <= grid.maxX; i++) {
			s += grid.map[[i, j]] ? '#' : '.'
		}
		console.log(s)
	}
	console.log(time)
}

for (let i=0; i < 10905; i++) {
	tick()
}

keypress(process.stdin)

process.stdin.on('keypress', (ch, key) => {
	if (key && key.ctrl && key.name == 'c') {
		process.stdin.pause()
	} else if (key.name === 'left') {
		untick()
		print(grid(1))
	} else if (key.name === 'right') {
		tick()
		print(grid(1))
	}

})

process.stdin.setRawMode(true)

// const g = grid()
print(grid(1))

// Part 2

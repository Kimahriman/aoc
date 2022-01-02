const depth = 11739
// const depth = 510
const target = [11, 718]
// const target = [10, 10]

const erosion = {} 

for (let i=0; i <= target[0]; i++) {
	for (let j=0; j <= target[1]; j++) {
		let geologic
		if (i === 0 && j === 0) {
			geologic = 0
		} else if (i === 0) {
			geologic = j * 48271
		} else if (j === 0) {
			geologic = i * 16807
		} else if (i === target[0] && j === target[1]) {
			geologic = 0
		} else {
			geologic = erosion[[i-1, j]] * erosion[[i, j-1]]
		}
		erosion[[i, j]] = (geologic + depth) % 20183
	}
}

for (let j=0; j <= target[1]; j++) {
	let s = ''
	for (let i=0; i <= target[0]; i++) {
		switch (erosion[[i, j]] % 3) {
			case 0:
				s += '.'
				break
			case 1:
				s += '='
				break
			case 2:
				s += '|'
				break
		}
	}
	console.log(s)
}

console.log(Object.values(erosion).reduce((acc, e) => acc + (e % 3), 0))

// Part 2

const grid = {}

const equipMap = {
	rocky: ['torch', 'climbing'],
	wet: ['climbing', 'nothing'],
	narrow: ['torch', 'nothing']
}

function getErosion(x, y) {
	let e
	if (erosion[[x, y]]) {
		return erosion[[x, y]]
	} else {
		let geologic
		if (x === 0 && y === 0) {
			geologic = 0
		} else if (x === 0) {
			geologic = y * 48271
		} else if (y === 0) {
			geologic = x * 16807
		} else if (x === target[0] && y === target[1]) {
			geologic = 0
		} else {
			geologic = getErosion(x-1, y) * getErosion(x, y-1)
		}
		e = (geologic + depth) % 20183
		erosion[[x, y]] = e
	}
	return e
}

function getType(x, y) {
	return ['rocky', 'wet', 'narrow'][getErosion(x, y) % 3]
}

function valid(type, equip) {
	return equipMap[type].includes(equip)
}

function change(currentType, equiped) {
	return equipMap[currentType].find(e => e !== equiped)
}

function move(currentType, equiped, minutes, nextType) {
	// console.log("Is valid", nextType, equiped, valid(nextType, equiped))
	if (valid(nextType, equiped)) {
		return { minutes: minutes + 1, equiped }
	} else {
		return { minutes: minutes + 8, equiped: change(currentType, equiped) }
	}
}

const startType = getType(0, 0)
let queue = [
	{ x: 0, y: 0, type: startType, equiped: 'torch', minutes: 0 }
]
grid[[0, 0]] = {
	torch: 0
}

const steps = [[0, 1], [1, 0], [0, -1], [-1, 0]]

let max = 0
while (true) {
	queue = queue.sort((a, b) => b.minutes - a.minutes)
	const current = queue.pop()
	// console.log(current.x, current.y, current.minutes, current.equiped, current.type)
	if (current.minutes > max) {
		max = current.minutes
		console.log(max)
	}
	if (current.x === target[0] && current.y === target[1]) {
		console.log("Hit target in", current.minutes, "minutes with", current.equiped)
		if (current.equiped !== 'torch') {
			queue.push({ ...current, equiped: 'torch', minutes: current.minutes + 7})
		} else {
			break
		}
	}
	steps.forEach(([dx, dy]) => {
		const [x, y] = [current.x + dx, current.y + dy]
		if (x < 0 || y < 0) {
			return
		}
		const nextType = getType(x, y)
		const { minutes, equiped } = move(current.type, current.equiped, current.minutes, nextType)
		const existing = grid[[x, y]]
		if (existing && equiped in existing) {
			if (minutes < existing[equiped]) {
				existing[equiped] = minutes
				queue.push({ x, y, type: nextType, equiped, minutes })
			}
		} else {
			if (!existing) {
				grid[[x, y]] = {}
			}
			grid[[x, y]][equiped] = minutes
			queue.push({ x, y, type: nextType, equiped, minutes })
		}
	})
}
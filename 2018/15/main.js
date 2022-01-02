const fs = require('fs')

function simulate(attack) {
	const data = fs.readFileSync('input.txt').toString()

	let goblins = []
	let elves = []
	const grid = data.trim().split('\n')
		.map((row, j) => {
			return row.split('').map((c, i) => {
				switch (c) {
					case '#':
						return { wall: true }
					case '.':
						return { wall: false }
					case 'G':
						const g = { type: 'goblin', x: i, y: j, health: 200, attack: 3 }
						goblins.push(g)
						return { wall: false, unit: g }
					case 'E':
						const e = { type: 'elf', x: i, y: j, health: 200, attack }
						elves.push(e)
						return { wall: false, unit: e }
				}
			})
		})

	const elfCount = elves.length

	function print(g) {
		g.forEach(row => {
			console.log(row.map(c => c.wall ? '#' : !c.unit ? '.' : c.unit.type === 'goblin' ? 'G' : 'E').join(''))
		})
	}

	function getTargets(type, x, y) {
		const adjacent = [
			grid[y-1] && grid[y-1][x],
			grid[y][x+1],
			grid[y+1] && grid[y+1][x],
			grid[y][x-1]
		]
		
		return adjacent
			.filter(a => a && a.unit && a.unit.type === type)
			.sort((a, b) => a.unit.health !== b.unit.health 
				? a.unit.health - b.unit.health
				: a.unit.y !== b.unit.y
					? a.unit.y - b.unit.y
					: a.unit.x - b.unit.x)
	}

	function findNearest(type, x, y, steps, seen) {
		const queue = [{ x, y, steps }]
		let found = null
		const options = []

		while (queue.length > 0) {
			const option = queue.shift()
			const cell = grid[option.y][option.x]
			if (seen[[option.x, option.y]] || !cell || cell.unit || cell.wall || (found && option.steps > found)) {
				continue
			}

			seen[[option.x, option.y]] = true

			const targets = getTargets(type, option.x, option.y)
			if (targets.length > 0) {
				found = option.steps
				options.push(option)
			} else {
				[
					[option.x, option.y-1, grid[option.y-1] && grid[option.y-1][x]],
					[option.x+1, option.y, grid[option.y][option.x+1]],
					[option.x, option.y+1, grid[option.y+1] && grid[option.y+1][option.x]],
					[option.x-1, option.y, grid[option.y][option.x-1]]
				].forEach(([x, y, a]) => {
					// console.log("Adding", x, y, steps + 1)
					queue.push({ x, y, steps: option.steps + 1})
				})
			}
		}

		// console.log("Found these options", options)

		return options.sort((a, b) => a.y !== b.y ? a.y - b.y : a.x - b.x)[0]
	}

	function step() {
		const units = [...goblins, ...elves].sort((a, b) => a.y === b.y ? a.x - b.x : a.y - b.y)

		units.forEach(unit => {
			if (grid[unit.y][unit.x].unit !== unit) {
				// this was killed off
				return
			}

			const targetType = unit.type === 'goblin' ? 'elf' : 'goblin'
			let targets = getTargets(targetType, unit.x, unit.y)

			if (!targets.length) {
				const seen = {}
				seen[[unit.x, unit.y]] = true
				const possibleSteps = [
					[0, -1, findNearest(targetType, unit.x, unit.y-1, 1, { ...seen })],
					[-1, 0, findNearest(targetType, unit.x-1, unit.y, 1, { ...seen })],
					[1, 0, findNearest(targetType, unit.x+1, unit.y, 1, { ...seen })],
					[0, 1, findNearest(targetType, unit.x, unit.y+1, 1, { ...seen })]
				].filter(([x, y, s]) => s).sort(([x1, y1, a], [x2, y2, b]) => a.steps !== b.steps ? a.steps - b.steps : a.y !== b.y ? a.y - b.y : a.x - b.x)

				if (possibleSteps.length > 0) {
					const [x, y, step] = possibleSteps[0]
					// console.log("Moving toward", step, "from", unit.x, unit.y)
					grid[unit.y][unit.x].unit = null
					unit.x += x
					unit.y += y
					grid[unit.y][unit.x].unit = unit
					targets = getTargets(targetType, unit.x, unit.y)
				}
			} else {
				// console.log("Target found", unit.x, unit.y)
			}

			if (targets.length) {
				const target = targets[0]
				// console.log("Attacking target", target.unit.health, unit.attack)
				target.unit.health -= unit.attack
				if (target.unit.health <= 0) {
					// console.log(target.unit.type, "killed")
					goblins = goblins.filter(x => x !== target.unit)
					elves = elves.filter(x => x !== target.unit)
					grid[target.unit.y][target.unit.x].unit = null
				}
			}
		})
		// console.log("===========================")
		// print(grid)
	}

	let rounds = 0
	while (goblins.length > 0 && elves.length > 0) {
		step()
		rounds++
	}

	// print(grid)
	console.log("Took", rounds)
	if (elves.length > 0) {
		console.log("Elves win with remaining health of", elves.reduce((acc, e) => acc + e.health, 0))
	} else if (goblins.length > 0) {
		console.log("Goblins win with remaining health of", goblins.reduce((acc, e) => acc + e.health, 0))
	}
	console.log(elfCount - elves.length, "elves died")
	return elves.length === elfCount
}

let attack = 4
while (!simulate(attack)) {
	console.log("Failed for attack", attack)
	attack++
}

console.log("Attack is", attack)

// Part 2


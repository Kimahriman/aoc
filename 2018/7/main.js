const fs = require('fs')

const data = fs.readFileSync('input.txt').toString()

const regex = /Step ([A-Z]) must be finished before step ([A-Z]) can begin./

const steps = data.trim().split('\n')
	.map(d => d.match(regex))
	.reduce((acc, d) => {
		if (!acc[d[1]]) {
			acc[d[1]] = new Set()
		}
		if (!acc[d[2]]) {
			acc[d[2]] = new Set()
		}
		acc[d[2]].add(d[1])
		return acc
	}, {})

let finished = []

function isSubset(set1, set2) {
	for (var elem of set1) {
		if (!set2.has(elem)) {
			return false
		}
	}
	return true
}

const p1Steps = { ...steps }
while (Object.keys(p1Steps).length > 0) {
	const next = Object.entries(p1Steps).reduce((acc, [step, reqs]) => {
		return isSubset(reqs, new Set(finished)) && (!acc || step < acc) ? step : acc
	}, '')

	finished.push(next)
	delete p1Steps[next]
}

console.log(finished.join(''))

// Part 2

const maxWorkers = 5
let workers = []
finished = new Set()

let time = -1
while (Object.keys(steps).length > 0 || workers.length > 0) {
	time++

	workers.forEach(w => {
		w.timeLeft--
	})

	workers.filter(w => w.timeLeft == 0).forEach(w => {
		console.log("Finished", w.id)
		finished.add(w.id)
	})

	workers = workers.filter(w => w.timeLeft > 0)
	while (workers.length < maxWorkers) {
		const next = Object.entries(steps).reduce((acc, [step, reqs]) => {
			return isSubset(reqs, finished) && (!acc || step < acc) ? step : acc
		}, '')
		// console.log("Trying to get new thing to work on", next)

		if (next) {
			delete steps[next]
			workers.push({ id: next, timeLeft: 60 + next.charCodeAt(0) - 'A'.charCodeAt(0) + 1 })
			console.log("Starting", next, 60 + next.charCodeAt(0) - 'A'.charCodeAt(0) + 1)
		} else {
			break
		}
	}
}

console.log(time)

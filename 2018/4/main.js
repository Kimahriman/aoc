const fs = require('fs')

const data = fs.readFileSync('input.txt').toString()

const re = /\[\d{4}-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.+)/
const parsed = data.trim().split('\n')
	.map(d => d.match(re))
	.map(p => ({
		month: parseInt(p[1]),
		day: parseInt(p[2]),
		hour: parseInt(p[3]),
		minute: parseInt(p[4]),
		message: p[5]
	}))

const sorted = parsed.sort((a, b) => a.month !== b.month ? a.month - b.month
	: a.day !== b.day ? a.day - b.day
		: a.hour !== b.hour ? a.hour - b.hour
			: a.minute - b.minute
)

const guards = {}
let currentGuard
let fellAsleepAt
sorted.forEach(m => {
	const match = m.message.match(/Guard #(\d+) begins shift/)
	if (match) {
		currentGuard = match[1]
		if (!guards[currentGuard]) {
			guards[currentGuard] = []
		}
	} else if (m.message.match(/falls asleep/)) {
		fellAsleepAt = m.minute
	} else if (m.message.match(/wakes up/)) {
		guards[currentGuard].push([...Array(m.minute - fellAsleepAt)].map((d, i) => i + fellAsleepAt))
	}
})

const guardAsleepMinutes = Object.keys(guards).reduce((acc, id) => {
	acc[id] = guards[id].reduce((total, day) => total + day.length, 0)
	return acc
}, {})

const longestAsleepGuard = Object.keys(guardAsleepMinutes).reduce((acc, id) => {
	if (!acc || guardAsleepMinutes[id] > guardAsleepMinutes[acc]) {
		return id
	}
	return acc
}, null)

console.log("Longest asleep", longestAsleepGuard)

const guardMinutes = guards[longestAsleepGuard].reduce((acc, minutes) => {
	minutes.forEach(m => acc[m] = (acc[m] || 0) + 1)
	return acc
}, {})

const bestMinute = Object.keys(guardMinutes).reduce((acc, minute) => guardMinutes[minute] > guardMinutes[acc] ? minute : acc)

console.log("Best minute is", bestMinute)

// Part 2

const guardsAsleepByMinute = Object.keys(guards).reduce((acc, id) => {
	guards[id].forEach(day => {
		day.forEach(minute => {
			if (!acc[minute]) {
				acc[minute] = []
			}
			acc[minute].push(id)
		})
	})
	return acc
}, {})

const { id, minute } = Object.keys(guardsAsleepByMinute).reduce((acc, minute) => {
	const daysPerGuard = guardsAsleepByMinute[minute].reduce((a, id) => {
		a[id] = (a[id] || 0) + 1
		return a
	}, {})

	const maxGuard = Object.keys(daysPerGuard).reduce((a, id) => {
		if (daysPerGuard[id] > a.total) {
			a = { id, total: daysPerGuard[id] }
		}
		return a
	}, { id: null, total: 0 })

	if (maxGuard.total > acc.total) {
		acc = { id: maxGuard.id, total: maxGuard.total, minute }
	}
	return acc
}, { id: null, total: 0, minute: 0 })

console.log('Max minute', id, minute, id * minute)



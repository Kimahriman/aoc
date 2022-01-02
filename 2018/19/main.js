const fs = require('fs')

const data = fs.readFileSync('input.txt').toString().trim().split('\n')

const ipReg = parseInt(data.splice(0, 1)[0].match(/#ip (\d+)/)[1])
console.log("IP Register", ipReg)

const instructions = data.map(line => line.match(/(\w+) (\d+) (\d+) (\d+)/)).map(p => [p[1], ...p.slice(2, 5).map(d => parseInt(d))])
console.log(instructions)

const ops = {
	addr: (regs, a, b, c) => replace(regs, c, regs[a] + regs[b]),
	addi: (regs, a, b, c) => replace(regs, c, regs[a] + b),
	mulr: (regs, a, b, c) => replace(regs, c, regs[a] * regs[b]),
	muli: (regs, a, b, c) => replace(regs, c, regs[a] * b),
	banr: (regs, a, b, c) => replace(regs, c, regs[a] & regs[b]),
	bani: (regs, a, b, c) => replace(regs, c, regs[a] & b),
	borr: (regs, a, b, c) => replace(regs, c, regs[a] | regs[b]),
	bori: (regs, a, b, c) => replace(regs, c, regs[a] | b),
	setr: (regs, a, b, c) => replace(regs, c, regs[a]),
	seti: (regs, a, b, c) => replace(regs, c, a),
	gtir: (regs, a, b, c) => replace(regs, c, a > regs[b] ? 1 : 0),
	gtri: (regs, a, b, c) => replace(regs, c, regs[a] > b ? 1 : 0),
	gtrr: (regs, a, b, c) => replace(regs, c, regs[a] > regs[b] ? 1 : 0),
	eqir: (regs, a, b, c) => replace(regs, c, a === regs[b] ? 1 : 0),
	eqri: (regs, a, b, c) => replace(regs, c, regs[a] === b ? 1 : 0),
	eqrr: (regs, a, b, c) => replace(regs, c, regs[a] === regs[b] ? 1 : 0)
}

const replace = (arr, i, val) => {
	const copy = [...arr]
	copy[i] = val
	return copy
}

function equal(arr1, arr2) {
	for (let i=0; i < arr1.length; i++) {
		if (arr1[i] !== arr2[i]) {
			return false
		}
	}
	return true
}

function valid(sample, op) {
	const [opNum, a, b, c] = sample[1]
	const res = op(sample[0], a, b, c)
	return equal(sample[2], res)
}

// const instructions = program.trim().split('\n')
// 	.map(line => line.match(/(\d+) (\d+) (\d+) (\d+)/))
// 	.map(parsed => parsed.slice(1, 5).map(x => parseInt(x)))

let regs = [1, 0, 0, 0, 0, 0]

while (instructions[regs[ipReg]]) {
	if ([1, 2].includes(regs[ipReg])) {
		console.log("At instruction", regs[ipReg], regs)
	}
	const [op, a, b, c] = instructions[regs[ipReg]]
	regs = ops[op](regs, a, b, c)
	regs[ipReg]++
}

console.log(regs)

// instructions.forEach(inst => {
// 	const [opCode, a, b, c] = inst
// 	const op = ops[opMap[opCode]]
// 	regs = op(regs, a, b, c)
// })

// console.log(regs)

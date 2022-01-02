const fs = require('fs')

const data = fs.readFileSync('input.txt').toString().trim().split('\n')

const ipReg = parseInt(data.splice(0, 1)[0].match(/#ip (\d+)/)[1])
console.log("IP Register", ipReg)

const instructions = data.map(line => line.match(/(\w+) (\d+) (\d+) (\d+)/)).map(p => [p[1], ...p.slice(2, 5).map(d => parseInt(d))])
// console.log(instructions)

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

// const instructions = program.trim().split('\n')
// 	.map(line => line.match(/(\d+) (\d+) (\d+) (\d+)/))
// 	.map(parsed => parsed.slice(1, 5).map(x => parseInt(x)))

// let start = 0
// const limit = 1000000
// while (true) {
    // let count = 0
    let regs = [0, 0, 0, 0, 0, 0]
    const matches = []
    const set = new Set()
    // let lowest = 1000/0000000
    while (instructions[regs[ipReg]]) {
        if (regs[ipReg] === 28) {
            // console.log("Final check", regs)
            // if (regs[4] < lowest) {
            //     console.log("Found new lowest", regs[4])
            //     lowest = regs[4]
            // }
            if (set.has(regs[4])) {
                console.log("Found repeat")
                break
            }
            matches.push(regs[4])
            set.add(regs[4])
        }
        const [op, a, b, c] = instructions[regs[ipReg]]
        regs = ops[op](regs, a, b, c)
        regs[ipReg]++
        // count++
    }

console.log(matches)
console.log(matches[matches.length -1])

    // if (count < limit) {
        // console.log("Terminated at", start, "After", count)
    // }
    // start++
// }


// instructions.forEach(inst => {
// 	const [opCode, a, b, c] = inst
// 	const op = ops[opMap[opCode]]
// 	regs = op(regs, a, b, c)
// })

// console.log(regs)

const fs = require('fs')

const data = fs.readFileSync('input.txt').toString()
    .trim()
    .split('\n')

let state = data.splice(0, 2)[0]
    .match(/initial state: ([.#]+)/)[1]
    .split('')
    .map(c => c === '#')
    .reduce((acc, c, i) => {
        acc[i] = c
        return acc
    }, {})

console.log(state)

function index(condition) {
    return condition.reduce((acc, c, i) => c ? acc + (1 << i) : acc, 0)
}

const rules = data
    .map(rule => rule.match(/([.#]{5}) => ([.#])/))
    .map(([_, condition, result]) => [condition.split('').map(c => c === '#'), result === '#'])
    .reduce((acc, rule) => {
        acc[index(rule[0])] = rule[1]
        return acc
    }, {})

console.log(rules)

function iterate() {
    const min = Math.min(...Object.keys(state).map(k => parseInt(k)))
    const max = Math.max(...Object.keys(state).map(k => parseInt(k)))
    // console.log("Iterating", min, max)

    const newState = {}
    for (let i=min-2; i < max + 2; i++) {
        const condition = [state[i-2], state[i-1], state[i], state[i+1], state[i+2]]
        const result = !!rules[index(condition)]
        if (!result && (i < min || i > max)) {
            continue
        }
        newState[i] = result
    }

    state = newState
}

function print() {
    // console.log(state)
    let line = ''
    const min = Math.min(...Object.keys(state).map(k => parseInt(k)))
    const max = Math.max(...Object.keys(state).map(k => parseInt(k)))

    for (let i=min; i <= max; i++) {
        line += state[i] ? '#' : '.'
    }
    console.log(line)
}
const cache = {}

// print()
for (let i=1; i < 50000000000; i++) {
    iterate()
    if (i % 10000 === 0) {
        console.log(i)
    }

    let min = Math.min(...Object.keys(state).map(k => parseInt(k)))
    let max = Math.max(...Object.keys(state).map(k => parseInt(k)))

    while (true) {
        if (state[min]) {
            break
        }
        min++
    }

    while (true) {
        if (state[max]) {
            break
        }
        max--
    }

    let hash = ''
    for (let j=min; j <= max; j++) {
        hash += state[j] ? '#' : '.'
    }

    if (cache[hash]) {
        console.log("Found repeat!", min, i, ...cache[hash], hash)
        min = (50000000000 - i) + min
        console.log(min, hash.split('').reduce((acc, c, i) => c === '#' ? acc + i + min : acc, 0))
        break
    }

    cache[hash] = [min, i]
}

const total = Object.entries(state).reduce((acc, [num, res]) => res ? acc + parseInt(num) : acc, 0)

console.log(total)

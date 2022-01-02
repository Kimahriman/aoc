const fs = require('fs')

const data = fs.readFileSync('input.txt').toString().trim()

const alphabet = 'abcdefghijklmnopqrstuvwxyz'

const regexString = alphabet.split('').reduce((acc, letter) => {
    const letterRegex = `(${letter}${letter.toUpperCase()})|(${letter.toUpperCase()}${letter})`
    return `${acc}${acc ? '|' : ''}${letterRegex}`
}, '')
const regex = new RegExp(regexString, 'g')

console.log(data.length)

function react(s) {
    let oldLength
    do {
        oldLength = s.length
        s = s.replace(regex, '')

    } while (oldLength > s.length)
    return s
}

const reducedLength = react(data).length

console.log(reducedLength)

// Part 2

const modifiedReducedLength = alphabet.split('').reduce((acc, letter) => {
    const letterRegex = `(${letter})|(${letter.toUpperCase()})`
    const modified = data.replace(new RegExp(letterRegex, 'g'), '')
    const length = react(modified).length
    return length < acc ? length : acc
}, reducedLength)

console.log(modifiedReducedLength)
const fs = require('fs')

const data = fs.readFileSync('input.txt').toString()

const forward = cart => {
    switch (cart.dir) {
        case 0:
            cart.pos.y -= 1
            break
        case 1:
            cart.pos.x += 1
            break
        case 2:
            cart.pos.y += 1
            break
        case 3:
            cart.pos.x -= 1
            break
    }
}

const turn = turnDir => cart => {
    if (cart.dir % 2 === 0) {
        cart.dir += turnDir ? 1 : -1
    } else {
        cart.dir += turnDir ? -1 : 1
    }
    cart.dir = (cart.dir + 4) % 4
    forward(cart)
}

const inter = cart => {
    cart.dir += cart.turn - 1
    cart.dir = (cart.dir + 4) % 4
    cart.turn = (cart.turn + 1) % 3
    forward(cart)
}

const { grid, carts } = data.split('\n')
    .reduce((acc, row, j) => {
        row.split('').forEach((c, i) => {
            let cell
            switch (c) {
                case '|':
                case '-':
                    cell = { move: forward, c }
                    break
                case '/':
                    cell = { move: turn(true), c }
                    break
                case '\\':
                    cell = { move: turn(false), c }
                    break
                case '^':
                case '>':
                case 'v':
                case '<':
                    let cart = { pos: { x: i, y: j }, dir: '^>v<'.indexOf(c), turn: 0 }
                    cell = { move: forward, cart, c: '|-|-'.charAt(cart.dir) }
                    acc.carts.push(cart)
                    break
                case '+':
                    cell = { move: inter, c }
            }
            acc.grid[[i, j]] = cell
        })
        return acc
    }, { grid: {}, carts: [] })

function print() {
    // console.log(carts)
    const cartMap = carts.reduce((acc, c) => {
        acc[[c.pos.x, c.pos.y]] = c
        return acc
    }, {})

    for (j=0; j < 150; j++) {
        let s = ''
        for (i=0; i < 150; i++) {
            const cell = grid[[i, j]]
            if (cell) {
                const cart = cartMap[[i, j]]
                if (cart) {
                    s += '^>v<'.charAt(cart.dir)
                } else {
                    s += cell.c
                }
            } else {
                s += ' '
            }
        }
        console.log(s)
    }
}

// print()
while (carts.length > 1) {
    carts.sort((a, b) => a.pos.y === b.pos.y ? a.pos.x - b.pos.x : a.pos.y - b.pos.y)

    const toRemove = []

    carts.forEach(c => {
        if (toRemove.includes(c)) {
            return
        }
        const oldPos = grid[[c.pos.x, c.pos.y]]
        oldPos.cart = null
        oldPos.move(c)
        const newPos = grid[[c.pos.x, c.pos.y]]
        // console.log("New pos", c.pos.x, c.pos.y)
        if (newPos.cart) {
            toRemove.push(newPos.cart)
            toRemove.push(c)
            newPos.cart = null
            // print()
            console.log("Crash at", c.pos.x, c.pos.y, toRemove, toRemove.map(c => carts.includes(c)))
        } else {
            newPos.cart = c
        }
    })

    toRemove.forEach(c => {
        carts.splice(carts.indexOf(c), 1)
    })
    // print()
}

// print()
console.log("Final cart at", carts[0])

// console.log('Sum', sum)

// Part 2

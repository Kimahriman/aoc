const serial = 18

const grid = {}

for (let i=1; i <= 300; i++) {
    for (let j=1; j <= 300; j++) {
        let power = (i + 10) * j
        power += serial
        power *= (i + 10)
        power = Math.floor(power / 100) % 10
        power -= 5
        grid[[i, j]] = power
    }
}

let max = [-Infinity, 0, 0, 0]

const powerCache = {}

for (let size=1; size <= 300; size++) {
    console.log("Searching size", size)
    for (let i=1; i <= 301 - size; i++) {
        for (let j=1; j <= 301 - size; j++) {
            let totalPower = powerCache[[i, j, size-1]] || 0
            delete powerCache[[i, j, size-1]]

            for (let k=0; k < size; k++) {
                totalPower += grid[[i + k, j + size - 1]]
                totalPower += grid[[i + size - 1, j + k]]
            }
            totalPower -= grid[[i + size - 1, j + size - 1]] // double counted

            powerCache[[i, j, size]] = totalPower

            if (totalPower > max[0]) {
                max = [totalPower, i, j, size]
            }
        }
    }
}

console.log(max)
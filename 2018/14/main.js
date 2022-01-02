const trainingCount = 51589
const searchString = trainingCount.toString()
console.log("Looking for", searchString)
// const trainingCount = 9

let recipes = [3, 7]
let elf1 = 0
let elf2 = 1

function iterate() {
	const combined = recipes[elf1] + recipes[elf2]
	if (combined >= 10) {
		recipes.push(1)
	}
	recipes.push(combined % 10)

	elf1 = (elf1 + recipes[elf1] + 1) % recipes.length
	elf2 = (elf2 + recipes[elf2] + 1) % recipes.length
}

// while (recipes.length < trainingCount + 10) {
// 	iterate()
// }

// console.log(recipes.slice(trainingCount, trainingCount + 10).join(''))
// console.log(recipes.slice(0, 25).join(' '))

// Part 2

while (true) {
	iterate()

	if (recipes.slice(recipes.length - searchString.length - 1, recipes.length - 1).join('') === searchString) {
		console.log("Found at", recipes.length - searchString.length - 1)
		break
	} else if (recipes.slice(recipes.length - searchString.length, recipes.length).join('') === searchString) {
		console.log("Found at", recipes.length - searchString.length)
		break
	}
}

console.log(recipes.join('').indexOf(trainingCount.toString()))
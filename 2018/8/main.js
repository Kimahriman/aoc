const fs = require('fs')

const data = fs.readFileSync('input.txt').toString()

const vals = data.trim().split(' ')
	.map(d => parseInt(d))

function sumMetadata(index) {
	let sum = 0
	const numChildren = vals[index]
	const numMetadata = vals[index+1]

	index += 2

	for (let i=0; i < numChildren; i++) {
		[nextIndex, s] = sumMetadata(index)
		index = nextIndex
		sum += s
	}

	for (let j=0; j < numMetadata; j++) {
		sum += vals[index]
		index++
	}

	return [index, sum]
}

console.log(sumMetadata(0))

// Part 2

function getNode(index) {
	const numChildren = vals[index]
	const numMetadata = vals[index+1]
	console.log("Creating node", index, numChildren, numMetadata)
	let start = index
	const children = []
	const metadata = []

	index += 2

	for (let i=0; i < numChildren; i++) {
		console.log("Creating child", i, numChildren, start, "->", index)
		const [nextIndex, node] = getNode(index)
		index = nextIndex
		console.log("Adding child", node.start)
		children.push(node)
	}

	for (let j=0; j < numMetadata; j++) {
		metadata.push(vals[index])
		index++
	}

	console.log("Returning node", start)
	return [index, { start, children, metadata }]
}

function getValue(node) {
	// console.log("Getting value", node.start)
	if (node.children.length === 0) {
		return node.metadata.reduce((sum, m) => sum + m, 0)
	}

	return node.metadata.reduce((sum, m) => {
		// console.log(m, node.children.map(c => c.start))
		childIndex = m - 1
		if (childIndex >= 0 && childIndex < node.children.length) {
			return sum + getValue(node.children[childIndex])
		}
		return sum
	}, 0)
}

console.log("Root value", getValue(getNode(0)[1]))

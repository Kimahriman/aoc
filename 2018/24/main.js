const fs = require('fs')

const data = fs.readFileSync('input.txt').toString()

const [immune, infection] = data.split('\n\n')

function parseGroup(group) {
	return group.trim().split('\n').slice(1)
		.map(line => {
			const main = line.match(/(\d+) units each with (\d+) hit points.*with an attack that does (\d+) (\w+) damage at initiative (\d+)/)
			let [units, hp, attack, type, initiative] = main.slice(1)
			units = parseInt(units)
			hp = parseInt(hp)
			attack = parseInt(attack)
			initiative = parseInt(initiative)

			let immunities = line.match(/immune to (\w+(?:, \w+)*)(?:;|\))/)
			immunities = immunities ? immunities[1].split(', ') : []

			let weaknesses = line.match(/weak to (\w+(?:, \w+)*)(?:;|\))/)
			weaknesses = weaknesses ? weaknesses[1].split(', ') : []

			return {
				units,
				hp,
				attack,
				type,
				initiative,
				immunities,
				weaknesses
			}
		})
}

function attackPower(g1, g2) {
	const weak = g2.weaknesses.includes(g1.type) ? 2 : 1
	const immune = g2.immunities.includes(g1.type) ? 0 : 1
	return g1.units * g1.attack * weak * immune
}

function getTargets(attacker, defender) {
	const targeted = new Set()
	return attacker
		.map(g => {
			const target = defender.reduce((acc, ig) => {
				const ap = attackPower(g, ig)
				if (ap <= 0 || targeted.has(ig)) {
					return acc
				}
				if (!acc) {
					return ig
				}
				const currentAp = attackPower(g, acc)
				if (ap > currentAp) {
					return ig
				}
				if (ap === currentAp && ig.ep > acc.ep) {
					return ig
				}
				if (ap === currentAp && ig.ep === acc.ep && ig.initiative > acc.initiative) {
					return ig
				}
				return acc
			}, null)
			targeted.add(target)
			return target
		})
}

function sort(a, b) {
	return a.ep === b.ep ? b.initiative - a.initiative : b.ep - a.ep
}

const originalImmuneGroups = parseGroup(immune)
const originalInfectionGroups = parseGroup(infection)

function print(g1, g2) {
	g1.forEach(g => {
		console.log(g.units, g.hp, g.attack, g.type, g.immunities.join(', '), g.weaknesses.join(', '))
	})
	g2.forEach(g => {
		console.log(g.units, g.hp, g.attack, g.type, g.immunities.join(', '), g.weaknesses.join(', '))
	})
}

let boost = 0
while (true) {
	if ([35].includes(boost)) {
		boost++
		continue
	}
	console.log("Starting with boost", boost)
	let immuneGroups = originalImmuneGroups.map(g => ({ ...g, attack: g.attack + boost }))
	let infectionGroups = originalInfectionGroups.map(g => ({ ...g }))
	while (immuneGroups.length > 0 && infectionGroups.length > 0) {
		immuneGroups.forEach(g => {
			g.ep = g.units * g.attack
		})
		infectionGroups.forEach(g => {
			g.ep = g.units * g.attack
		})
		immuneGroups = immuneGroups.sort(sort)
		infectionGroups = infectionGroups.sort(sort)
		// boost >= 35 && console.log("Getting targets")
		const immuneTargets = getTargets(immuneGroups, infectionGroups).map((t, i) => [immuneGroups[i], t])
		const infectionTargets = getTargets(infectionGroups, immuneGroups).map((t, i) => [infectionGroups[i], t])
		// boost >= 35 && console.log(immuneTargets, infectionTargets)
	
		const targets = immuneTargets.concat(infectionTargets).sort((a, b) => b[0].initiative - a[0].initiative)
		// console.log("Commensing attack")
		targets.forEach(([attacker, defender]) => {
			if (attacker.units <= 0 || !defender) {
				// boost >= 35 && console.log("No units or defender", attacker.units, defender)
				return
			}
			const damage = Math.floor(attackPower(attacker, defender) / defender.hp)
			// boost >= 35 && console.log("Doing damage", Math.floor(attackPower(attacker, defender) / defender.hp), defender.units)
			// boost >= 35 && damage === 0 && print(immuneGroups, infectionGroups)
			defender.units -= Math.floor(attackPower(attacker, defender) / defender.hp)
		})
		// console.log("Filtering out dead")
		immuneGroups = immuneGroups.filter(g => g.units > 0)
		infectionGroups = infectionGroups.filter(g => g.units > 0)
	}
	
	if (immuneGroups.length > 0) {
		const unitsLeft = immuneGroups.reduce((acc, g) => acc + g.units, 0)
		console.log("Immune groups won with", unitsLeft, "units left")
		break
	}
	
	if (infectionGroups.length > 0) {
		const unitsLeft = infectionGroups.reduce((acc, g) => acc + g.units, 0)
		console.log("Infection groups won with", unitsLeft, "units left")
	}
	boost++
}

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use regex::Regex;
use queues::*;

enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode
}

#[derive(Clone, Debug)]
struct Blueprint {
    id: String,
    ore_cost: i32,
    clay_cost: i32,
    obsidian_ore_cost: i32,
    obsidian_clay_cost: i32,
    geode_ore_cost: i32,
    geode_obsidian_cost: i32
}

impl Blueprint {

}

#[derive(Clone, Debug)]
struct BlueprintState<'a> {
    blueprint: &'a Blueprint,
    minutes_left: i32,
    ore_robots: i32,
    ore: i32,
    clay_robots: i32,
    clay: i32,
    obsidian_robots: i32,
    obsidian: i32,
    geode_robots: i32,
    geode: i32,
    // state_history: Vec<BlueprintState<'a>>
}

impl<'a> BlueprintState<'a> {
    fn new(blueprint: &'a Blueprint, minutes_left: i32) -> Self {
        BlueprintState {
            blueprint,
            minutes_left,
            ore_robots: 1,
            ore: 0,
            clay_robots: 0,
            clay: 0,
            obsidian_robots: 0,
            obsidian: 0,
            geode_robots: 0,
            geode: 0,
            // state_history: Vec::new()
        }
    }

    fn print(&self) {
        println!("Minutes left: {}, ore robots: {}, ore: {}, clay_robots: {}, clay: {}, obsidian_robots: {}, obsidian: {}, geode_robots: {}, geode: {}",
            self.minutes_left, self.ore_robots, self.ore, self.clay_robots, self.clay, self.obsidian_robots, self.obsidian, self.geode_robots, self.geode)
    }

    fn can_build_ore(&self) -> bool {
        self.ore >= self.blueprint.ore_cost
    }

    fn build_ore(&mut self) {
        self.ore -= self.blueprint.ore_cost;
        self.ore_robots += 1;
    }

    fn can_build_clay(&self) -> bool {
        self.ore >= self.blueprint.clay_cost
    }

    fn build_clay(&mut self) {
        self.ore -= self.blueprint.clay_cost;
        self.clay_robots += 1;
    }

    fn can_build_obsidian(&self) -> bool {
        self.ore >= self.blueprint.obsidian_ore_cost && self.clay >= self.blueprint.obsidian_clay_cost
    }

    fn build_obsidian(&mut self) {
        self.ore -= self.blueprint.obsidian_ore_cost;
        self.clay -= self.blueprint.obsidian_clay_cost;
        self.obsidian_robots += 1;
    }

    fn can_build_geode(&self) -> bool {
        self.ore >= self.blueprint.geode_ore_cost && self.obsidian >= self.blueprint.geode_obsidian_cost
    }

    fn build_geode(&mut self) {
        self.ore -= self.blueprint.geode_ore_cost;
        self.obsidian -= self.blueprint.geode_obsidian_cost;
        self.geode_robots += 1;
    }
}

struct Simulation<'a> {
    state_queue: Vec<BlueprintState<'a>>,
    most_geodes: i32,
    // best_state: Option<BlueprintState<'a>>
}

impl <'a> Simulation<'a> {
    fn new(blueprint: &'a Blueprint, minutes: i32) -> Self {
        let mut simulation = Simulation { state_queue: Vec::new(), most_geodes: 0 };
        simulation.state_queue.push(BlueprintState::new(blueprint, minutes));
        simulation
    }

    fn max_possible_geodes(state: &BlueprintState) -> i32 {
        let mut geodes = state.geode;
        // Assume every minute left we can build a new geode robot
        for i in 0..state.minutes_left {
            geodes += state.geode_robots + i;
        }
        geodes
    }

    fn run_simulation(&mut self) {
        while self.state_queue.len() > 0 {
            self.step();
            // let mut buf = String::new();
            // std::io::stdin().read_line(&mut buf).unwrap();
        }
    }

    fn step(&mut self) {
        let state = self.state_queue.pop().unwrap();
        // print!("Processing state ");
        // state.print();
        if state.minutes_left <= 0 {
            if state.geode > self.most_geodes {
                self.most_geodes = state.geode;
                println!("Found new max geodes {}", self.most_geodes);
            }
            return;
        }
        if Simulation::max_possible_geodes(&state) < self.most_geodes {
            return;
        }

        let mut new_robots: Vec<Robot> = Vec::new();
        if state.can_build_ore() {
            new_robots.push(Robot::Ore)
        }
        if state.can_build_clay() {
            new_robots.push(Robot::Clay)
        }
        if state.can_build_obsidian() {
            new_robots.push(Robot::Obsidian)
        }
        if state.can_build_geode() {
            new_robots.push(Robot::Geode)
        }

        let mut can_build_everything = state.can_build_ore() && state.can_build_clay();
        if state.clay_robots > 0 {
            can_build_everything &= state.can_build_obsidian();
        }
        if state.obsidian_robots > 0 {
            can_build_everything &= state.can_build_geode();
        }

        let mut next_state = state.clone();
        // state.state_history.clear();
        // next_state.state_history.push(state);

        next_state.ore += next_state.ore_robots;
        next_state.clay += next_state.clay_robots;
        next_state.obsidian += next_state.obsidian_robots;
        next_state.geode += next_state.geode_robots;

        next_state.minutes_left -= 1;
        // If we can build any robots, create a new state branch for that
        for robot in new_robots.into_iter() {
            let mut new_state = next_state.clone();
            match robot {
                Robot::Ore => {
                    new_state.build_ore();
                }
                Robot::Clay => {
                    new_state.build_clay();
                }
                Robot::Obsidian => {
                    new_state.build_obsidian();
                }
                Robot::Geode => {
                    new_state.build_geode();
                }
            }
            self.state_queue.push(new_state);
        }

        // If we can build everything, there's no benefit of not building anything
        if !can_build_everything {
            self.state_queue.push(next_state);
        } else {
            // print!("Dropping state because everything can be built ");
            // next_state.print();
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

    let mut blueprints: Vec<Blueprint> = Vec::new();

    for line in lines.iter() {
        let cap = re.captures(line).unwrap();

        blueprints.push(Blueprint {
            id: cap.get(1).unwrap().as_str().to_string(),
            ore_cost: cap.get(2).unwrap().as_str().parse().unwrap(),
            clay_cost: cap.get(3).unwrap().as_str().parse().unwrap(),
            obsidian_ore_cost: cap.get(4).unwrap().as_str().parse().unwrap(),
            obsidian_clay_cost: cap.get(5).unwrap().as_str().parse().unwrap(),
            geode_ore_cost: cap.get(6).unwrap().as_str().parse().unwrap(),
            geode_obsidian_cost: cap.get(7).unwrap().as_str().parse().unwrap()
        })
    }

    let mut quality_sum = 0;
    for blueprint in blueprints.iter() {
        let mut simulation = Simulation::new(blueprint, 24);
        simulation.run_simulation();
        println!("{} {}", blueprint.id, simulation.most_geodes);
        quality_sum += blueprint.id.parse::<i32>().unwrap() * simulation.most_geodes;
        // for hist in simulation.best_state.unwrap().state_history.iter() {
        //    hist.print();
        // }
    }
    println!("{}", quality_sum);

    let mut geode_mult = 1;
    for blueprint in blueprints[0..3].iter() {
        let mut simulation = Simulation::new(blueprint, 32);
        simulation.run_simulation();
        println!("{} {}", blueprint.id, simulation.most_geodes);
        geode_mult *= simulation.most_geodes;
    }
    println!("{}", geode_mult);
}

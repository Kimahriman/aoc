use core::num;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use regex::Regex;

// #[derive(Clone, Debug)]
// struct Stack {
    
// }

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let mut line_iter = lines.iter();
    let mut line = line_iter.next().unwrap();

    let num_stacks = line.len() / 4 + 1; // No space on end
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    while *line != "" {
        let mut chars = line.chars();
        chars.next();
        for i in 0..num_stacks {
            let box_type = chars.next().unwrap();
            if box_type != ' ' && box_type >= 'A' && box_type <= 'Z' {
                stacks[i].push(box_type);
            }
            chars.next();
            chars.next();
            chars.next();
        }
        line = line_iter.next().unwrap()
    }
    for stack in stacks.iter() {
        println!("{:?}", stack);
    }

    for i in 0..num_stacks {
        stacks[i].reverse();
    }

    let mut phase2_stacks = stacks.clone();

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in line_iter {
        let caps = re.captures(line).unwrap();
        let num_to_move: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let from: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let to: usize = caps.get(3).unwrap().as_str().parse().unwrap();

        // Part 1
        for _ in 0..num_to_move {
            let val = stacks[from-1].pop().unwrap();
            stacks[to-1].push(val);
        }

        // Part 2
        let mut tmp: Vec<char> = Vec::new();
        for _ in 0..num_to_move {
            let val = phase2_stacks[from-1].pop().unwrap();
            tmp.push(val);
        }
        for _ in 0..num_to_move {
            let val = tmp.pop().unwrap();
            phase2_stacks[to-1].push(val);
        }
    }

    for stack in stacks.iter() {
        print!("{}", stack.last().unwrap());
    }
    println!();
    for stack in phase2_stacks.iter() {
        print!("{}", stack.last().unwrap());
    }
    println!();
    
}

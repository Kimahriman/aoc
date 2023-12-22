use std::collections::{HashMap, HashSet, VecDeque};

#[derive(PartialEq)]
enum Place {
    Plot,
    Rock,
}

fn main() {
    let contents = std::fs::read_to_string("inputs/21.txt").unwrap();

    let mut grid: HashMap<(i32, i32), Place> = Default::default();
    let mut start = (0, 0);
    let height = contents.lines().count() as i32;
    let width = contents.lines().next().unwrap().len() as i32;
    for (i, line) in contents.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid.insert(
                (i as i32, j as i32),
                match c {
                    '.' => Place::Plot,
                    '#' => Place::Rock,
                    'S' => {
                        start = (i as i32, j as i32);
                        Place::Plot
                    }
                    _ => panic!(),
                },
            );
        }
    }

    let mut seen_steps = HashSet::<(i32, i32, i32)>::new();

    let mut step_queue = VecDeque::<(i32, i32, i32)>::new();

    step_queue.push_back((start.0, start.1, 0));

    while let Some((i, j, steps)) = step_queue.pop_front() {
        if steps >= 64 {
            continue;
        }
        for (k, l) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let loc = (i + k, j + l);
            if let Some(place) = grid.get(&loc) {
                if *place == Place::Plot && !seen_steps.contains(&(loc.0, loc.1, steps + 1)) {
                    step_queue.push_back((loc.0, loc.1, steps + 1));
                    seen_steps.insert((loc.0, loc.1, steps + 1));
                }
            }
        }
    }

    println!("{}", seen_steps.iter().filter(|s| s.2 == 64).count());

    seen_steps.clear();
    step_queue.clear();

    step_queue.push_back((start.0, start.1, 0));
    while let Some((i, j, steps)) = step_queue.pop_front() {
        if steps >= 5000 {
            continue;
        }
        for (k, l) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let loc = (i + k, j + l);
            if let Some(place) = grid.get(&(loc.0 % height, loc.1 % width)) {
                if *place == Place::Plot && !seen_steps.contains(&(loc.0, loc.1, steps + 1)) {
                    step_queue.push_back((loc.0, loc.1, steps + 1));
                    seen_steps.insert((loc.0, loc.1, steps + 1));
                }
            }
        }
    }
    println!("{}", seen_steps.iter().filter(|s| s.2 == 5000).count());
}

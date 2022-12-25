use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use regex::Regex;

fn check_up(pos: &(i32, i32), grid: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    let (x, y) = pos;
    if !grid.contains(&(x - 1, y - 1)) && !grid.contains(&(*x, y - 1)) && !grid.contains(&(x + 1, y - 1)) {
        Some((*x, y - 1))
    } else {
        None
    }
}

fn check_down(pos: &(i32, i32), grid: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    let (x, y) = pos;
    if !grid.contains(&(x - 1, y + 1)) && !grid.contains(&(*x, y + 1)) && !grid.contains(&(x + 1, y + 1)) {
        Some((*x, y + 1))
    } else {
        None
    }
}

fn check_left(pos: &(i32, i32), grid: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    let (x, y) = pos;
    if !grid.contains(&(x - 1, y - 1)) && !grid.contains(&(x - 1, *y)) && !grid.contains(&(x - 1, y + 1)) {
        Some((x - 1, *y))
    } else {
        None
    }
}

fn check_right(pos: &(i32, i32), grid: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    let (x, y) = pos;
    if !grid.contains(&(x + 1, y - 1)) && !grid.contains(&(x + 1, *y)) && !grid.contains(&(x + 1, y + 1)) {
        Some((x + 1, *y))
    } else {
        None
    }
}

fn print(grid: &HashSet<(i32, i32)>) {
    let mut min_x = i32::max_value();
    let mut min_y = i32::max_value();
    let mut max_x = i32::min_value();
    let mut max_y = i32::min_value();
    for p in grid.iter() {
        if p.0 < min_x {
            min_x = p.0;
        }
        if p.0 > max_x {
            max_x = p.0;
        }
        if p.1 < min_y {
            min_y = p.1;
        }
        if p.1 > max_y {
            max_y = p.1;
        }
    }
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if grid.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn get_proposed(grid: &HashSet<(i32, i32)>, starting_dir: usize, checks: &[fn(&(i32, i32), &HashSet<(i32, i32)>) -> Option<(i32, i32)>]) -> (Vec<((i32, i32), (i32, i32))>, HashMap<(i32, i32), i32>) {
    let mut proposed = Vec::<((i32, i32), (i32, i32))>::new();
    let mut new_spots = HashMap::<(i32, i32), i32>::new();

    for pos in grid.iter() {
        // First check if nothing is around it
        let mut neighbor = false;
        for i in -1..=1 {
            for j in -1..=1 {
                if grid.contains(&(pos.0 + i, pos.1 + j)) && !(i == 0 && j == 0) {
                    neighbor = true;
                }
            }
        }
        if !neighbor {
            continue;
        }
        for d in 0..4 {
            let func = checks[(starting_dir + d) % 4];
            if let Some(valid) = func(pos, &grid) {
                if new_spots.contains_key(&valid) {
                    new_spots.insert(valid, new_spots[&valid] + 1);
                } else {
                    new_spots.insert(valid, 1);
                }
                proposed.push((*pos, valid));
                break;
            }
        }
    }
    (proposed, new_spots)
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    
    let mut grid = HashSet::<(i32, i32)>::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                grid.insert((x as i32, y as i32));
            }
        }
    }

    // print(&grid);
    // println!();

    let mut starting_dir = 0;
    let checks = [check_up, check_down, check_left, check_right];

    for _ in 0..10 {
        let (proposed, new_spots) = get_proposed(&grid, starting_dir, &checks);
        let invalid: HashSet<_> = new_spots.into_iter().filter(|x| x.1 > 1).map(|x| x.0).collect();
        for p in proposed.iter() {
            let (old, new) = p;
            if !invalid.contains(new) {
                grid.remove(old);
                grid.insert(*new);
            }
        }

        starting_dir = (starting_dir + 1) % 4;
        // print(&grid);
        // println!();
    }

    let mut min_x = i32::max_value();
    let mut min_y = i32::max_value();
    let mut max_x = i32::min_value();
    let mut max_y = i32::min_value();
    for p in grid.iter() {
        if p.0 < min_x {
            min_x = p.0;
        }
        if p.0 > max_x {
            max_x = p.0;
        }
        if p.1 < min_y {
            min_y = p.1;
        }
        if p.1 > max_y {
            max_y = p.1;
        }
    }

    let mut empty = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !grid.contains(&(x, y)) {
                empty += 1;
            }
        }
    }
    println!("{}", empty);

    let mut rounds = 10;
    loop {
        rounds += 1;
        let (proposed, new_spots) = get_proposed(&grid, starting_dir, &checks);
        if proposed.is_empty() {
            break;
        }
        let invalid: HashSet<_> = new_spots.into_iter().filter(|x| x.1 > 1).map(|x| x.0).collect();
        for p in proposed.iter() {
            let (old, new) = p;
            if !invalid.contains(new) {
                grid.remove(old);
                grid.insert(*new);
            }
        }

        starting_dir = (starting_dir + 1) % 4;
        // print(&grid);
        // println!();
    }

    println!("Rounds: {}", rounds);
}

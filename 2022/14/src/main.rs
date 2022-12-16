use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::str::Chars;
use std::string::ToString;
use std::fmt::Debug;

enum Fill {
    Air,
    Sand,
    Rock
}

fn parse_pair(pair: &str) ->(i32, i32) {
    let strs = pair.split_once(",").unwrap();
    return (strs.0.parse().unwrap(), strs.1.parse().unwrap());
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut grid: HashMap<(i32, i32), Fill> = HashMap::new();
    let mut max_y = 0;

    for line in lines.iter() {
        let mut pairs = line.split(" -> ");
        let (mut prev_x, mut prev_y) = parse_pair(&pairs.next().unwrap());

        for pair in pairs {
            let (cur_x, cur_y) = parse_pair(pair);
            if (prev_x == cur_x) {
                if prev_y < cur_y {
                    for y in prev_y..=cur_y {
                        grid.insert((prev_x, y), Fill::Rock);
                    }
                } else {
                    for y in cur_y..=prev_y {
                        grid.insert((prev_x, y), Fill::Rock);
                    }
                }
            } else {
                if prev_x < cur_x {
                    for x in prev_x..=cur_x {
                        grid.insert((x, prev_y), Fill::Rock);
                    }
                } else {
                    for x in cur_x..=prev_x {
                        grid.insert((x, prev_y), Fill::Rock);
                    }
                }
            }
            if prev_y > max_y {
                max_y = prev_y;
            }
            if cur_y > max_y {
                max_y = cur_y;
            }

            prev_x = cur_x;
            prev_y = cur_y;
        }
    }

    let mut finished = false;
    let mut sand_count = 0;
    while !finished {
        let mut sand_x = 500;
        let mut sand_y = 0;
        sand_count += 1;

        let mut stopped = false;
        while !stopped {
            if !grid.contains_key(&(sand_x, sand_y + 1)) {
                sand_y += 1;
            } else if !grid.contains_key(&(sand_x -1, sand_y + 1)) {
                sand_x -= 1;
                sand_y += 1;
            } else if !grid.contains_key(&(sand_x + 1, sand_y + 1)) {
                sand_x += 1;
                sand_y += 1;
            } else {
                stopped = true;
                grid.insert((sand_x, sand_y), Fill::Sand);
            }

            if sand_y > max_y {
                finished = true;
                break;
            }
        }
    }

    println!("{}", sand_count - 1);

    while !grid.contains_key(&(500, 0)) {
        let mut sand_x = 500;
        let mut sand_y = 0;
        sand_count += 1;

        let mut stopped = false;
        while !stopped {
            if !grid.contains_key(&(sand_x, sand_y + 1)) && sand_y <= max_y {
                sand_y += 1;
            } else if !grid.contains_key(&(sand_x - 1, sand_y + 1)) && sand_y <= max_y {
                sand_x -= 1;
                sand_y += 1;
            } else if !grid.contains_key(&(sand_x + 1, sand_y + 1)) && sand_y <= max_y {
                sand_x += 1;
                sand_y += 1;
            } else {
                stopped = true;
                grid.insert((sand_x, sand_y), Fill::Sand);
            }
        }
    }

    println!("{}", sand_count - 1);
}

use std::fs;
use ansi_term::Colour::Red;

use regex::Regex;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut dots: Vec<(usize, usize)> = Vec::new();

    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    let mut lines_iter = lines.iter();

    while let Some(line) = lines_iter.next() {
        if *line == "" { break }
        let splits: Vec<&str> = line.split(",").collect();
        let x: usize = splits[0].parse().unwrap();
        let y: usize = splits[1].parse().unwrap();

        if x > max_x { max_x = x; }
        if y > max_y { max_y = y; }
        dots.push((x, y));
    }

    max_x += 1;
    max_y += 1;
    if (max_x % 2 == 0) {
        max_x += 1;
    }
    if (max_y % 2 == 0) {
        max_y += 1;
    }

    let mut grid = vec![vec![false; max_x]; max_y];
    for dot in dots {
        grid[dot.1][dot.0] = true;
    }

    let re = Regex::new(r"^fold along (\w)=(\d+)$").unwrap();
    while let Some(line) = lines_iter.next() {
        let cap = re.captures(line).unwrap();
        let val: usize = cap[2].parse().unwrap();
        println!("Maxes: {} {}", max_x, max_y);
        if &cap[1] == "x" {
            println!("Folding on x = {} ({})", val, max_x / 2);
            // for r in 0..max_y {
            //     for c in 0..max_x {
            //         if grid[r][c] {
            //             if c == val {
            //                 print!("{}", Red.bold().paint("#"));
            //             } else {
            //                 print!("#");
            //             }
            //         } else {
            //             if c == val {
            //                 print!("{}", Red.bold().paint("."));
            //             } else {
            //                 print!(".");
            //             }
            //         }
            //     }
            //     println!();
            // }
            for r in 0..max_y {
                for c in 0..val {
                    if grid[r][val + (val - c)] {
                        grid[r][c] = true;
                    }
                }
            }
            max_x = val
        } else {
            println!("Folding on y = {} ({})", val, max_y / 2);
            // for r in 0..=max_y {
            //     for c in 0..=max_x {
            //         if grid[r][c] {
            //             if r == val {
            //                 print!("{}", Red.bold().paint("#"));
            //             } else {
            //                 print!("#");
            //             }
            //         } else {
            //             if r == val {
            //                 print!("{}", Red.bold().paint("."));
            //             } else {
            //                 print!(".");
            //             }
            //         }
            //     }
            //     println!();
            // }
            for r in 0..val {
                for c in 0..max_x {
                    if grid[val + (val -  r)][c] {
                        grid[r][c] = true;
                    }
                }
            }
            max_y = val
        }
    }

    let mut total = 0;
    for r in 0..max_y {
        for c in 0..max_x {
            if grid[r][c] {
                total += 1;
            }
        }
    }
    println!("{}", total);

    for r in 0..max_y {
        for c in 0..max_x {
            if grid[r][c] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
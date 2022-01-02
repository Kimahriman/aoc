use std::fs;
use std::collections::HashMap;
use ansi_term::Colour::Red;

fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut lines_iter = lines.iter();
    
    let alg_line = lines_iter.next().unwrap();

    let mut alg = vec![false; alg_line.len()];
    let mut i = 0;
    for c in alg_line.chars() {
        if c == '#' { alg[i] = true };
        i += 1;
    }

    let mut grid = HashMap::<(i32, i32), bool>::new();

    lines_iter.next();

    let mut row = 0;
    let mut col = 0;
    for line in lines_iter {
        col = 0;
        for c in line.chars() {
            grid.insert((row, col), if c == '#' { true } else { false });
            col += 1;
        }
        row += 1;
    }

    println!("Initial count: {}", grid.values().filter(|x| **x).count());
    print(&grid);

    for i in 0..50 {
        grid = enhance(grid, &alg, i);
        println!("New count: {} {}", i, grid.values().filter(|x| **x).count());
        // print(&grid);
    }
    print(&grid);
}

// fn lookup_pixel() {}

fn print(map: &HashMap<(i32, i32), bool>) {
    let mut min_col = 0;
    let mut max_col = 0;
    let mut min_row = 0;
    let mut max_row = 0;
    for (row, col) in map.keys() {
        if *row < min_row { min_row = *row }
        if *row > max_row { max_row = *row }
        if *col < min_col { min_col = *col }
        if *col > max_col { max_col = *col }
    }
    min_col -= 5;
    min_row -=5;
    max_col += 5;
    max_row += 5;

    for r in min_row..=max_row {
        for c in min_col..=max_col {
            if *map.get(&(r, c)).unwrap_or(&false) {
                if r == 0 && c == 0 {
                    print!("{}", Red.bold().paint("#"))
                } else {
                    print!("{}",  "#")
                }
            } else {
                if r == 0 && c == 0 {
                    print!("{}", Red.bold().paint("."))
                } else {
                    print!("{}",  ".")
                }
            }
        }
        println!();
    }
}

fn enhance(map: HashMap<(i32, i32), bool>, alg: &Vec<bool>, iteration: u32) -> HashMap<(i32, i32), bool> {
    let mut min_col = 0;
    let mut max_col = 0;
    let mut min_row = 0;
    let mut max_row = 0;
    for (row, col) in map.keys() {
        if *row < min_row { min_row = *row }
        if *row > max_row { max_row = *row }
        if *col < min_col { min_col = *col }
        if *col > max_col { max_col = *col }
    }

    // Buffer out by one to account for expanding
    min_col -= 1;
    min_row -= 1;
    max_col += 1;
    max_row += 1;

    let mut new_map = HashMap::<(i32, i32), bool>::new();
    for r in min_row..=max_row {
        for c in min_col..=max_col {
            let mut val = 0_u32;
            for i in -1..=1 {
                for j in -1..=1 {
                    val <<= 1;
                    let default = if iteration % 2 == 0 { false } else { true };
                    if *map.get(&(r + i, c + j)).unwrap_or(&default) { val += 1 }
                }
            }
            // println!("Inserting {} into ({}, {})", val, r, c);
            new_map.insert((r, c), alg[val as usize]);
        }
    }
    new_map
}

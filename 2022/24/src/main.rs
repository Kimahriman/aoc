use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::time::Duration;
use regex::Regex;

#[derive(Copy, Clone)]
enum Dir {
    Up,
    Right,
    Down,
    Left
}

fn print(grid: &HashMap<(usize, usize), Vec<Dir>>, positions: &HashSet<(usize, usize)>, width: usize, height: usize) {
    for _ in 0..width {
        print!("#");
    }
    println!();
    for y in 1..(height-1) {
        print!("#");
        for x in 1..(width-1) {
            if let Some(vec) = grid.get(&(x, y)) {
                if vec.len() > 1 {
                    print!("{}", vec.len());
                } else {
                    let c = match vec[0] {
                        Dir::Up => "^",
                        Dir::Right => ">",
                        Dir::Down => "v",
                        Dir::Left => "<"
                    };
                    print!("{}", c);
                }
            } else if positions.contains(&(x, y)) {
                print!("E");
            } else {
                print!(".");
            }
        }
        print!("#");
        println!();
    }
    for _ in 0..width {
        print!("#");
    }
    println!();
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    
    let first_line = lines[0];
    

    let width = first_line.len();
    let height = lines.len();
    println!("{} {}", width, height);
    // Blizzards repeat the same position after this many steps
    let cycle = (width - 2) * (height - 2);
    let start_x = first_line.find(".").unwrap();

    let mut grid = HashMap::<(usize, usize), Vec<Dir>>::new();

    for (y, line) in lines[1..(lines.len()-1)].iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let d = match c {
                '^' => Some(Dir::Up),
                '>' => Some(Dir::Right),
                'v' => Some(Dir::Down),
                '<' => Some(Dir::Left),
                _ => None
            };
            if let Some(dir) = d {
                if !grid.contains_key(&(x, y + 1)) {
                    grid.insert((x, y + 1), Vec::new());
                }
                grid.get_mut(&(x, y + 1)).unwrap().push(dir);
            }
        }
    }

    let final_x = lines[lines.len() - 1].find(".").unwrap();
    let final_y = lines.len() - 1;

    let mut positions = HashSet::<(usize, usize)>::new();
    positions.insert((start_x, 0));

    let mut steps = 0;
    let mut found_start = false;
    let mut found_end_again = false;
    let mut end_is_goal = true;

    while !found_end_again {
        steps += 1;
        if steps % 1000 == 0 {
            println!("{}", steps);
        }
        let mut new_grid = HashMap::<(usize, usize), Vec<Dir>>::new();

        for (p, dirs) in grid.iter() {
            for d in dirs.iter() {
                let mut new_x = p.0;
                let mut new_y = p.1;
                match d {
                    Dir::Up => {
                        if p.1 == 1 {
                            new_y = height - 2;
                        } else {
                            new_y -= 1;
                        }
                    },
                    Dir::Right => {
                        if p.0 == width - 2 {
                            new_x = 1;
                        } else {
                            new_x += 1;
                        }
                    },
                    Dir::Down => {
                        if p.1 == height - 2 {
                            new_y = 1;
                        } else {
                            new_y += 1;
                        }
                    },
                    Dir::Left => {
                        if p.0 == 1 {
                            new_x = width - 2;
                        } else {
                            new_x -= 1;
                        }
                    }
                }
                if !new_grid.contains_key(&(new_x, new_y)) {
                    new_grid.insert((new_x, new_y), Vec::new());
                }
                new_grid.get_mut(&(new_x, new_y)).unwrap().push(*d);
            }
        }

        let mut new_positions = HashSet::<(usize, usize)>::new();
        'outer: for (x, y) in positions.iter() {
            let xi = *x as i32;
            let yi = *y as i32;
            for (i, j) in [(xi-1, yi), (xi+1, yi), (xi, yi-1), (xi, yi+1), (xi, yi)].iter() {
                if *i == final_x as i32 && *j == final_y as i32 && end_is_goal {
                    println!("Reaced the end in {} steps", steps);
                    end_is_goal = false;
                    if found_start {
                        found_end_again = true;
                    }
                    new_positions.clear();
                    new_positions.insert((final_x, final_y));
                    break 'outer;
                }
                if *i == start_x as i32 && *j == 0 && !end_is_goal {
                    println!("Reached start again in {} steps", steps);
                    end_is_goal = true;
                    found_start = true;
                    new_positions.clear();
                    new_positions.insert((start_x, 0));
                    break 'outer;
                }
                if *i <= 0 || *j <= 0 || *i >= width as i32 - 1 || *j >= height as i32 - 1 {
                    // Ignore the starting position
                    if (*i != start_x as i32 || *j != 0) && (*i != final_x as i32 || *j != final_y as i32) {
                        // println!("Skipping {} {} as out of bounds", i, j);
                        continue;
                    }
                }
                if !new_grid.contains_key(&(*i as usize, *j as usize)) {
                    new_positions.insert((*i as usize, *j as usize));
                }
            }
        }
        grid = new_grid;
        positions = new_positions;
        if positions.is_empty() {
            println!("Ran out of positions to try");
            break;
        }
    }
}

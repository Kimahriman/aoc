use std::cmp;
use std::fs;
use regex::Regex;

#[derive(Copy, Clone, PartialEq)]
enum Place {
    EMPTY,
    RIGHT,
    DOWN
}

fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut grid = vec![vec![Place::EMPTY; lines[0].len()]; lines.len()];

    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '>' {
                grid[row][col] = Place::RIGHT;
            } else if c == 'v' {
                grid[row][col] = Place::DOWN;
            }
        }
    }

    let mut count = 0;
    let mut changed = true;
    while changed {
        count += 1;
        let (new_grid, new_changed) = update(grid);
        grid = new_grid;
        changed = new_changed;
    }
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let c = match grid[row][col] {
                Place::RIGHT => '>',
                Place::DOWN => 'v',
                Place::EMPTY => '.'
            };
            print!("{}", c);
        }
        println!();
    }

    println!("{}", count)
}

fn update(grid: Vec<Vec<Place>>) -> (Vec<Vec<Place>>, bool) {
    let mut changed = false;
    let mut new_grid = grid.clone();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == Place::RIGHT && grid[row][(col + 1) % grid[row].len()] == Place::EMPTY {
                changed = true;
                new_grid[row][col] = Place::EMPTY;
                new_grid[row][(col + 1) % grid[row].len()] = Place::RIGHT;
            }
        }
    }
    let mut new_new_grid = new_grid.clone();
    for row in 0..new_grid.len() {
        for col in 0..new_grid[row].len() {
            if new_grid[row][col] == Place::DOWN && new_grid[(row + 1) % new_grid.len()][col] == Place::EMPTY {
                changed = true;
                new_new_grid[row][col] = Place::EMPTY;
                new_new_grid[(row + 1) % new_grid.len()][col] = Place::DOWN;
            }
        }
    }
    (new_new_grid, changed)
}

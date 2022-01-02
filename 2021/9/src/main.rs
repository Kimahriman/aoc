use std::fs;
use ansi_term::Colour::Red;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let height = lines.len();
    let width = lines[0].len();

    let mut state = vec![vec![0u32; width]; height];

    for r in 0..height {
        let line: Vec<char> = lines[r].chars().collect();
        for c in 0..width {
            state[r][c] = line[c].to_digit(10).unwrap();
        }
    }

    let mut low_points: Vec<(u32, u32)> = Vec::new();

    let mut total_risk = 0;
    for r in 0..height {
        for c in 0..width {
            if r > 0 && state[r-1][c] <= state[r][c] { print!("{}", state[r][c]); continue };
            if r < height - 1 && state[r+1][c] <= state[r][c] { print!("{}", state[r][c]); continue };
            if c > 0 && state[r][c-1] <= state[r][c] { print!("{}", state[r][c]); continue };
            if c < width - 1 && state[r][c+1] <= state[r][c] { print!("{}", state[r][c]); continue };
            print!("{}", Red.bold().paint(state[r][c].to_string()));
            total_risk += state[r][c] + 1;
            low_points.push((r.try_into().unwrap(), c.try_into().unwrap()));
        }
        println!();
    }
    println!("{}", total_risk);

    let mut visited = vec![vec![false; width]; height];

    let mut sizes = Vec::<u32>::new();

    for (r, c) in low_points.iter() {
        sizes.push(visit((*r).try_into().unwrap(), (*c).try_into().unwrap(), width, height, &state, &mut visited));
    }
    sizes.sort();
    let final_total = sizes.pop().unwrap() * sizes.pop().unwrap() * sizes.pop().unwrap();
    println!("{}", final_total);
}

fn visit(r: usize, c: usize, width: usize, height: usize, grid: &Vec<Vec<u32>>, visited: &mut Vec<Vec<bool>>) -> u32 {
    if visited[r][c] {
        0
    } else {
        visited[r][c] = true;
        if grid[r][c] == 9 {
            0
        } else {
            let mut total = 1;
            if r > 0 { total += visit(r - 1, c, width, height, grid, visited) }
            if r < height - 1 { total += visit(r + 1, c, width, height, grid, visited) }
            if c > 0 { total += visit(r, c - 1, width, height, grid, visited) }
            if c < width - 1 { total += visit(r, c + 1, width, height, grid, visited) }
            total
        }
    }
}
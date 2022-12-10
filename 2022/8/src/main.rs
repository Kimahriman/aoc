use core::num;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;


fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let width = lines[0].len();
    let height = lines.len();

    let mut grid = vec![vec![0; height]; width];

    let mut y = 0;
    for line in lines.iter() {
        let mut x = 0;
        for c in line.chars() {
            grid[x][y] = c.to_string().parse().unwrap();
            x += 1;
        }
        y += 1;
    }
    
    let mut visible_trees = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let mut visible = false;
            // Check left
            if (0..x).all(|i| grid[i][y] < grid[x][y]) {
                visible = true;
            }
            // Check right
            if ((x+1)..width).all(|i| grid[i][y] < grid[x][y]) {
                visible = true;
            }
            // Check top
            if (0..y).all(|j| grid[x][j] < grid[x][y]) {
                visible = true;
            }
            // Check bottom
            if ((y+1)..height).all(|j| grid[x][j] < grid[x][y]) {
                visible = true;
            }

            if visible {
                visible_trees += 1;
            }
        }
    }
    println!("{}", visible_trees);

    let mut max_score = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let mut left = 0;
            let mut right = 0;
            let mut top = 0;
            let mut bottom = 0;
            
            println!("{} {}", x, y);

            for i in (0..x).rev() {
                left += 1;
                if grid[i][y] >= grid[x][y] {
                    break;
                }
            }
            
            for i in (x+1)..grid.len() {
                right += 1;
                if grid[i][y] >= grid[x][y] {
                    break;
                }
            }

            for j in (0..y).rev() {
                top += 1;
                if grid[x][j] >= grid[x][y] {
                    break;
                }
            }
            
            for j in (y+1)..grid[x].len() {
                bottom += 1;
                if grid[x][j] >= grid[x][y] {
                    break;
                }
            }
            
            println!("{} {} {} {}", left, right, top, bottom);
            let score = left * right * top * bottom;
            if score > max_score {
                max_score = score;
            }
        }
    }
    println!("{}", max_score);

}

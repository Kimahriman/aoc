use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let lookup: HashMap<char, i32> = ('a'..='z').enumerate().map(|(i, v)| (v, i as i32)).collect();

    let height = lines.len();
    let width = lines[0].len();

    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut grid = vec![vec![0; height]; width];

    let mut queue: Vec<((usize, usize), i32)> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c >= 'a' && c <= 'z' {
                grid[x][y] = lookup[&c];
            } else if c == 'S' {
                start = (x, y);
                grid[x][y] = 0;
            } else if c == 'E' {
                end = (x, y);
                grid[x][y] = 25;
            }
        }
    }

    queue.push((start, 0));
    
    while !queue.is_empty() {
        let (cur, dist) = queue.remove(0);
        if cur == end {
            println!("{}", dist);
            break;
        }

        let cur_height = grid[cur.0][cur.1];
        let cur_x: i32 = cur.0.try_into().unwrap();
        let cur_y: i32 = cur.1.try_into().unwrap();

        for (i, j) in neighbors.iter() {
            let (neigh_x, neigh_y) = (cur_x + i, cur_y + j);
            if neigh_x >= 0 && neigh_y >=0 && neigh_x < grid.len() as i32 && neigh_y < grid[0].len() as i32{
                let nx = neigh_x as usize;
                let ny = neigh_y as usize;

                let n_pos = (nx, ny);
                if !visited.contains(&n_pos) && grid[nx][ny] <= cur_height + 1 {
                    visited.insert(n_pos);
                    queue.push((n_pos, dist + 1));
                }
            }
        }
    }

    queue.clear();
    visited.clear();

    queue.push((end, 0));
    while !queue.is_empty() {
        let (cur, dist) = queue.remove(0);
        let cur_height = grid[cur.0][cur.1];
        let cur_x = cur.0 as i32;
        let cur_y = cur.1 as i32;

        if cur_height == 0 {
            println!("{}", dist);
            break;
        }

        for (i, j) in neighbors.iter() {
            let (neigh_x, neigh_y) = (cur_x + i, cur_y + j);
            if neigh_x >= 0 && neigh_y >=0 && neigh_x < grid.len() as i32 && neigh_y < grid[0].len() as i32{
                let nx = neigh_x as usize;
                let ny = neigh_y as usize;

                let n_pos = (nx, ny);
                if !visited.contains(&n_pos) && grid[nx][ny] >= cur_height - 1 {
                    visited.insert(n_pos);
                    queue.push((n_pos, dist + 1));
                }
            }
        }
    }
}

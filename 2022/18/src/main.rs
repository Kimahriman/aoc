use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut height = 0usize;
    let mut width = 0usize;
    let mut depth = 0usize;
    let mut min_x = usize::max_value();
    let mut min_y = usize::max_value();
    let mut min_z = usize::max_value();

    for line in lines.iter() {
        let mut coords = line.split(",");
        let x: usize = coords.next().unwrap().parse().unwrap();
        let y: usize = coords.next().unwrap().parse().unwrap();
        let z: usize = coords.next().unwrap().parse().unwrap();

        if x > width {
            width = x;
        }
        if x < min_x {
            min_x = x;
        }
        if y > height {
            height = y;
        }
        if y < min_y {
            min_y = y;
        }
        if z > depth {
            depth = z;
        }
        if z < min_z {
            min_z = z;
        }
    }

    width -= min_x;
    height -= min_y;
    depth -= min_z;

    width += 1;
    height += 1;
    depth += 1;

    let mut grid = vec![vec![vec![false; depth]; height]; width];

    for line in lines.iter() {
        let mut coords = line.split(",");
        let x: usize = coords.next().unwrap().parse().unwrap();
        let y: usize = coords.next().unwrap().parse().unwrap();
        let z: usize = coords.next().unwrap().parse().unwrap();

        grid[x - min_x][y - min_y][z - min_z] = true;
    }

    let mut surfaces = 0;

    for x in 0..width {
        for y in 0..height {
            for z in 0..depth {
                if grid[x][y][z] {
                    // If there's a cube here, check if we're on the edge and count those
                    if x == 0 || x == width - 1 {
                        surfaces += 1;
                    }
                    if y == 0 || y == height - 1 {
                        surfaces += 1;
                    }
                    if z == 0 || z == depth - 1 {
                        surfaces += 1;
                    }
                } else {
                    // Space here so we can have surfaces, check for cubes on borders
                    if x > 0 && grid[x - 1][y][z] {
                        surfaces += 1;
                    }
                    if x < width - 1 && grid[x + 1][y][z] {
                        surfaces += 1;
                    }
                    if y > 0 && grid[x][y - 1][z] {
                        surfaces += 1;
                    }
                    if y < height - 1 && grid[x][y + 1][z] {
                        surfaces += 1;
                    }
                    if z > 0 && grid[x][y][z - 1] {
                        surfaces += 1;
                    }
                    if z < depth - 1 && grid[x][y][z + 1] {
                        surfaces += 1;
                    }
                }
            }
        }
    }
    println!("{}", surfaces);


    surfaces = 0;
    // Start with all external cubes and work our way in
    let mut visited_cubes: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut cube_queue: Vec<(usize, usize, usize)> = Vec::new();
    for x in 0..width {
        for y in 0..height {
            for z in 0..depth {
                // First count the edges on the border
                if grid[x][y][z] {
                    // If there's a cube here, check if we're on the edge and count those
                    if x == 0 || x == width - 1 {
                        surfaces += 1;
                    }
                    if y == 0 || y == height - 1 {
                        surfaces += 1;
                    }
                    if z == 0 || z == depth - 1 {
                        surfaces += 1;
                    }
                } else {
                    if x == 0 || x == width - 1 || y == 0 || y == height - 1 || z == 0 || z == depth - 1 {
                        cube_queue.push((x, y, z));
                        visited_cubes.insert((x, y, z));
                    }
                }
            }
        }
    }

    while !cube_queue.is_empty() {
        let (x, y, z) = cube_queue.pop().unwrap();
        let mut candidates: Vec<(usize, usize, usize)> = Vec::new();
        if x > 0 && grid[x - 1][y][z] {
            surfaces += 1;
        } else if x > 0 {
            candidates.push((x - 1, y, z));
        }
        if x < width - 1 && grid[x + 1][y][z] {
            surfaces += 1;
        } else if x < width - 1 {
            candidates.push((x + 1, y, z));
        }
        if y > 0 && grid[x][y - 1][z] {
            surfaces += 1;
        } else if y > 0 {
            candidates.push((x, y - 1, z));
        }
        if y < height - 1 && grid[x][y + 1][z] {
            surfaces += 1;
        } else if y < height - 1 {
            candidates.push((x, y + 1, z));
        }
        if z > 0 && grid[x][y][z - 1] {
            surfaces += 1;
        } else if z > 0 {
            candidates.push((x, y, z - 1));
        }
        if z < depth - 1 && grid[x][y][z + 1] {
            surfaces += 1;
        } else if z < depth - 1 {
            candidates.push((x, y, z + 1));
        }

        for cand in candidates.into_iter() {
            if !visited_cubes.contains(&cand) {
                visited_cubes.insert(cand);
                cube_queue.push(cand);
            }
        }
    }

    println!("{}", surfaces);
}

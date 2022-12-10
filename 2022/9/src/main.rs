use core::num;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn move_tail(head_pos: &(i32, i32), tail_pos: &(i32, i32)) -> (i32, i32) {
    let mut new_x = tail_pos.0;
    let mut new_y = tail_pos.1;
    let x_diff = head_pos.0 - tail_pos.0;
    let y_diff = head_pos.1 - tail_pos.1;
    if x_diff.abs() > 1 || y_diff.abs() > 1 {
        if x_diff.abs() > 0 && y_diff.abs() > 0 {
            if x_diff > 0 {
                new_x += 1;
            } else {
                new_x -= 1;
            }
            if y_diff > 0 {
                new_y += 1;
            } else {
                new_y -= 1;
            }
        } else if x_diff.abs() > 0 {
            if x_diff > 0 {
                new_x += 1;
            } else {
                new_x -= 1;
            }
        } else {
            if y_diff > 0 {
                new_y += 1;
            } else {
                new_y -= 1;
            }
        }
    }
    return (new_x, new_y);
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    let mut visted_pos: HashSet<(i32, i32)> = HashSet::new();
    visted_pos.insert(tail_pos);
    for line in lines.iter() {
        let mut parts = line.split(" ");
        let dir = parts.next().unwrap();
        let steps: i32 = parts.next().unwrap().parse().unwrap();

        for _ in 0..steps {
            match dir {
                "R" => {
                    head_pos = (head_pos.0 + 1, head_pos.1);
                }
                "L" => {
                    head_pos = (head_pos.0 - 1, head_pos.1);
                }
                "U" => {
                    head_pos = (head_pos.0, head_pos.1 + 1);
                }
                _ => {
                    head_pos = (head_pos.0, head_pos.1 - 1);
                }
            }
            tail_pos = move_tail(&head_pos, &tail_pos);
            visted_pos.insert(tail_pos);
        }
    }
    println!("{}", visted_pos.len());

    let mut positions = vec![(0, 0); 10];
    visted_pos = HashSet::new();
    visted_pos.insert((0, 0));
    for line in lines.iter() {
        let mut parts = line.split(" ");
        let dir = parts.next().unwrap();
        let steps: i32 = parts.next().unwrap().parse().unwrap();

        for _ in 0..steps {
            match dir {
                "R" => {
                    positions[0] = (positions[0].0 + 1, positions[0].1);
                }
                "L" => {
                    positions[0] = (positions[0].0 - 1, positions[0].1);
                }
                "U" => {
                    positions[0] = (positions[0].0, positions[0].1 + 1);
                }
                _ => {
                    positions[0] = (positions[0].0, positions[0].1 - 1);
                }
            }
            for i in 1..positions.len() {
                positions[i] = move_tail(&positions[i-1], &positions[i]);
            }
            visted_pos.insert(positions[positions.len()-1]);
        }
    }

    println!("{}", visted_pos.len());
}

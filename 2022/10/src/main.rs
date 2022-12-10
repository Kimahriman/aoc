use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut reg_history: Vec<i32> = Vec::new();

    let mut reg = 1;
    // let mut cycle = 1;
    for line in lines.iter() {
        if *line == "noop" {
            reg_history.push(reg);
            continue;
        } else {
            let mut split = line.split(" ");
            split.next();
            let add: i32 = split.next().unwrap().parse().unwrap();

            reg_history.push(reg);

            reg += add;
            reg_history.push(reg);
        }
    }

    let mut strength = 0;
    for x in [20, 60, 100, 140, 180, 220] {
        strength += (x as i32) * reg_history[x - 2];
    }

    println!("{}", strength);

    let mut pixels: Vec<bool> = Vec::new();
    let mut reg_value = 1;
    for i in 0..reg_history.len() {
        if ((i as i32) % 40 - reg_value).abs() <= 1 {
            pixels.push(true);
        } else {
            pixels.push(false);
        }
        reg_value = reg_history[i];
    }

    for j in 0..6 {
        for i in 0..40 {
            if pixels[j * 40 + i] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

}

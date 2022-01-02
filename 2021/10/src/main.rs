use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut values: HashMap<char, u32> = HashMap::new();
    values.insert(')', 3);
    values.insert(']', 57);
    values.insert('}', 1197);
    values.insert('>', 25137);

    let mut score = 0;
    let mut incomplete_scores: Vec<u64> = Vec::new();
    for line in lines.iter() {
        let mut char_stack: Vec<char> = Vec::new();
        let mut corrupt = false;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => char_stack.push(c),
                _ => {
                    let last = char_stack.pop();
                    if last.is_none() { break }
                    if c == ')' && last.unwrap() == '(' { continue }
                    if c == ']' && last.unwrap() == '[' { continue }
                    if c == '}' && last.unwrap() == '{' { continue }
                    if c == '>' && last.unwrap() == '<' { continue }
                    score += values.get(&c).unwrap();
                    corrupt = true;
                }
            }
        }
        if !corrupt {
            let mut extra_score = 0_u64;
            for c in char_stack.iter().rev() {
                let char_score = match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0
                };
                extra_score *= 5;
                extra_score += char_score;
            }
            if extra_score > 0 {
                incomplete_scores.push(extra_score);
            }
        }
    }

    incomplete_scores.sort();

    println!("{}, {}", incomplete_scores.len(), incomplete_scores.len() / 2);

    println!("{}, {}", score, incomplete_scores[incomplete_scores.len() / 2]);
}
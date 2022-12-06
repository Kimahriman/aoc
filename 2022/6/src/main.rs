use core::num;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    for i in 0..(contents.len()-4) {
        let substr = &contents[i..i+4];
        if substr.chars().into_iter().collect::<HashSet<_>>().len() == 4 {
            println!("{}", i+4);
            break;
        }
    }

    for i in 0..(contents.len()-14) {
        let substr = &contents[i..i+14];
        if substr.chars().into_iter().collect::<HashSet<_>>().len() == 14 {
            println!("{}", i+14);
            break;
        }
    }
    
}

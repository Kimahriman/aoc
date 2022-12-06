use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Debug)]
struct ElfPair {
    left: Vec<i32>,
    right: Vec<i32>
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut elves: Vec<ElfPair> = Vec::new();

    for line in lines.iter() {
        let mut split = line.split(',');
        
        let left = split.next().unwrap();
        let mut left_split = left.split('-');
        let left_min: i32 = left_split.next().unwrap().parse().unwrap();
        let left_max: i32 = left_split.next().unwrap().parse().unwrap();
        let left_vec: Vec<i32> = (left_min..(left_max+1)).into_iter().collect();

        let right = split.next().unwrap();
        let mut right_split = right.split('-');
        let right_min: i32 = right_split.next().unwrap().parse().unwrap();
        let right_max: i32 = right_split.next().unwrap().parse().unwrap();
        let right_vec: Vec<i32> = (right_min..(right_max+1)).into_iter().collect();

        // println!("{:?} {:?}", left_vec, right_vec);

        let elf_pair = ElfPair { left: left_vec, right: right_vec };
        elves.push(elf_pair);
    }

    let mut full_overlapping: i32 = 0;
    let mut partial_overlapping: i32 = 0;
    for elf_pair in elves.iter() {
        let left_set: HashSet<_> = elf_pair.left.iter().collect();
        let right_set: HashSet<_> = elf_pair.right.iter().collect();

        let diff1: HashSet<_> = left_set.difference(&right_set).into_iter().collect();
        let diff2: HashSet<_> = right_set.difference(&left_set).into_iter().collect();

        let inter: HashSet<_> = left_set.intersection(&right_set).into_iter().collect();

        if diff1.is_empty() || diff2.is_empty() {
            full_overlapping += 1;
        }

        if !inter.is_empty() {
            partial_overlapping += 1;
        }
    }
    println!("{}", full_overlapping);
    println!("{}", partial_overlapping);
}

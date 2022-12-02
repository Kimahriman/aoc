use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut elf_list: Vec<Vec<i32>> = Vec::new();

    let mut current_elf: Vec<i32> = Vec::new();

    for line in lines.iter() {
        if *line == "" {
            elf_list.push(current_elf);
            current_elf = Vec::new();
        } else {
            current_elf.push(line.parse().unwrap());
        }
    }
    elf_list.push(current_elf);

    let mut cals_list: Vec<i32> = Vec::new();

    for elf in elf_list.iter() {
        let mut cals: i32 = 0;
        for meal in elf.iter() {
            cals += meal;
        }
        cals_list.push(cals);
    }
    
    cals_list.sort_by(|a, b| b.cmp(a));
    cals_list.truncate(3);

    let mut sum: i32 = 0;
    for cal in cals_list.iter() {
        println!("{}", cal);
        sum += cal;
    }
    
    println!("{}", sum);

    // let mut count = 0;
    // let mut overall_total = 0;
    // for line in lines.iter() {
    //     let splits: Vec<&str> = line.split(" | ").collect();
    //     let patterns: Vec<&str> = splits[0].split(" ").collect();
    //     let outputs: Vec<&str> = splits[1].split(" ").collect();

    //     for output in outputs.iter() {
    //         count += match output.len() {
    //             2 | 3 | 4 | 7 => 1,
    //             _ => 0
    //         }
    //     }
    //     let mut num_map: HashMap<String, i32> = HashMap::new();
    //     let mut reverse_map: HashMap<i32, HashSet<char>> = HashMap::new();
    //     for pattern in patterns.iter() {
    //         let mut pattern_chars: Vec<char> = pattern.chars().into_iter().collect();
    //         pattern_chars.sort();
    //         let sorted_chars: String = pattern_chars.into_iter().collect();
    //         if pattern.len() == 2 {
    //             num_map.insert(sorted_chars, 1);
    //             reverse_map.insert(1, pattern.chars().into_iter().collect());
    //         } else if pattern.len() == 3 {
    //             num_map.insert(sorted_chars, 7);
    //             reverse_map.insert(7, pattern.chars().into_iter().collect());
    //         } else if pattern.len() == 4 {
    //             num_map.insert(sorted_chars, 4);
    //             reverse_map.insert(4, pattern.chars().into_iter().collect());
    //         } else if pattern.len() == 7 {
    //             num_map.insert(sorted_chars, 8);
    //         }
    //     }
    //     for pattern in patterns.iter() {
    //         let mut pattern_chars: Vec<char> = pattern.chars().into_iter().collect();
    //         pattern_chars.sort();
    //         let sorted_chars: String = pattern_chars.into_iter().collect();
    //         if pattern.len() == 5 {
    //             let char_set: HashSet<char> = pattern.chars().into_iter().collect();
    //             if reverse_map.get(&7).unwrap().intersection(&char_set).collect::<HashSet<&char>>().len() == 3 {
    //                 // 7 is only fully in 3
    //                 num_map.insert(sorted_chars, 3);
    //             } else if reverse_map.get(&4).unwrap().intersection(&char_set).collect::<HashSet<&char>>().len() == 3 {
    //                 // 4 has three parts overlapped with 5
    //                 num_map.insert(sorted_chars, 5);
    //             } else {
    //                 num_map.insert(sorted_chars, 2);
    //             }
    //         } else if pattern.len() == 6 {
    //             let char_set: HashSet<char> = pattern.chars().into_iter().collect();
    //             if reverse_map.get(&4).unwrap().intersection(&char_set).collect::<HashSet<&char>>().len() == 4 {
    //                 // 4 has three parts overlapped with 5
    //                 num_map.insert(sorted_chars, 9);
    //             } else if reverse_map.get(&7).unwrap().intersection(&char_set).collect::<HashSet<&char>>().len() == 3 {
    //                 // 7 is only fully in 0
    //                 num_map.insert(sorted_chars, 0);
    //             } else {
    //                 num_map.insert(sorted_chars, 6);
    //             }
    //         }
    //     }
    //     // for (k, v) in num_map.iter() {
    //     //     println!("{}: {}", k, v);
    //     // }

    //     let mut output_value = 0;
    //     for output in outputs.iter() {
    //         output_value *= 10;
    //         let mut pattern_chars: Vec<char> = output.chars().into_iter().collect();
    //         pattern_chars.sort();
    //         let sorted_chars: String = pattern_chars.into_iter().collect();
    //         output_value += num_map.get(&sorted_chars).unwrap();
    //     }
    //     overall_total += output_value;
    //     // println!("{}", output_value);
    // }
    // println!("{}, {}", count, overall_total);
}

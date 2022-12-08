use core::num;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use regex::Regex;


fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut dirs: HashMap<Vec<String>, i32> = HashMap::new();

    let mut current_dir: Vec<String> = Vec::new();

    let cmd = Regex::new(r"^\$ (.*)").unwrap();
    let cd = Regex::new(r"^cd (.*)").unwrap();
    let dir = Regex::new(r"dir (.*)").unwrap();
    let file_size = Regex::new(r"(\d+) (.*)").unwrap();

    for line in lines.iter() {
        if let Some(cmd_cap) = cmd.captures(line) {
            if let Some(cd_cap) = cd.captures(cmd_cap.get(1).unwrap().as_str()) {
                match cd_cap.get(1).unwrap().as_str() {
                    "/" => {
                        current_dir = Vec::new();
                    }
                    ".." => {
                        current_dir.pop();
                    }
                    s => {
                        current_dir.push(s.to_string());
                    }
                }
            }
        } else if let Some(file_size_cap) = file_size.captures(line) {
            let cur_file_size: i32 = file_size_cap.get(1).unwrap().as_str().parse().unwrap();
            for i in 0..(current_dir.len()+1) {
                let slice = current_dir[0..i].to_vec();
                if let Some(v) = dirs.get_mut(&slice) {
                    *v += cur_file_size;
                } else {
                    dirs.insert(slice.clone(), cur_file_size);
                }
            }
        }
    }

    let mut sum: i32 = 0;
    for (k, v) in dirs.iter() {
        if *v <= 100000 {
            sum += v;
        }
    }
    println!("{}", sum);

    let used_space = dirs.get(&Vec::new()).unwrap();

    let space_to_free = 30000000 - (70000000 - used_space);
    let mut dir_size_to_delete = 70000000;

    for (k, v) in dirs.iter() {
        if *v >= space_to_free {
            if *v < dir_size_to_delete {
                dir_size_to_delete = *v;
            }
        }
    }
    println!("{}", dir_size_to_delete);


}

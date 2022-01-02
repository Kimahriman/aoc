use std::collections::{HashMap, VecDeque};
use std::fs;

struct Cave {
    key: String,
    big: bool,
    connections: Vec<String>
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut cave_map: HashMap<String, Cave> = HashMap::new();
    let mut final_paths: Vec<Vec<String>> = Vec::new();
    let mut path_queue: VecDeque<Vec<String>> = VecDeque::new();

    for line in lines {
        let parts: Vec<&str> = line.split("-").collect();
        let start = parts[0];
        let end = parts[1];
        println!("{}, {}", start, end);

        let start_cave = cave_map.entry(start.to_string()).or_insert(Cave {
            key: start.to_string(),
            big: start.chars().nth(0).unwrap() >= 'A' && start.chars().nth(0).unwrap() <= 'Z',
            connections: Vec::new()
        });
        start_cave.connections.push(end.to_string());

        let end_cave = cave_map.entry(end.to_string()).or_insert(Cave {
            key: start.to_string(),
            big: start.chars().nth(0).unwrap() >= 'A' && start.chars().nth(0).unwrap() <= 'Z',
            connections: Vec::new()
        });
        end_cave.connections.push(start.to_string());
    }

    path_queue.push_back(vec!["start".to_string()]);

    while !path_queue.is_empty() {
        let path = path_queue.pop_front().unwrap();
        let cave = cave_map.get(path.last().unwrap()).unwrap();
        for link in cave.connections.iter() {
            let linked_cave = cave_map.get(link).unwrap();
            if linked_cave.big || !path.contains(link) {
                let mut new_path: Vec<String> = path.clone();
                new_path.push(link.to_string());

                if link == "end" {
                    final_paths.push(new_path);
                } else {
                    path_queue.push_back(new_path);
                }
            }
        }
    }
    println!("{}", final_paths.len());
}
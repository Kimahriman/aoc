use std::collections::{HashMap, VecDeque};
use std::fs;

struct Cave {
    key: String,
    big: bool,
    connections: Vec<String>
}

struct Path {
    path: Vec<String>,
    visited_small: bool
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut cave_map: HashMap<String, Cave> = HashMap::new();
    let mut final_paths: Vec<Vec<String>> = Vec::new();
    let mut path_queue: VecDeque<Path> = VecDeque::new();

    for line in lines {
        let parts: Vec<&str> = line.split("-").collect();
        let start = parts[0];
        let end = parts[1];
        // println!("{}, {}", start, end);

        let start_cave = cave_map.entry(start.to_string()).or_insert(Cave {
            key: start.to_string(),
            big: start.chars().nth(0).unwrap() >= 'A' && start.chars().nth(0).unwrap() <= 'Z',
            connections: Vec::new()
        });
        start_cave.connections.push(end.to_string());

        let end_cave = cave_map.entry(end.to_string()).or_insert(Cave {
            key: end.to_string(),
            big: end.chars().nth(0).unwrap() >= 'A' && end.chars().nth(0).unwrap() <= 'Z',
            connections: Vec::new()
        });
        end_cave.connections.push(start.to_string());
    }
    for cave in cave_map.values() {
        println!("{}, {}", cave.key, cave.big);
    }

    path_queue.push_back(Path { path: vec!["start".to_string()], visited_small: false });

    while !path_queue.is_empty() {
        // println!("{}", path_queue.len());
        let path = path_queue.pop_front().unwrap();
        // print!("Processing ({}): ", path.visited_small);
        // for p in path.path.iter() {
        //     print!("{} ", p);
        // }
        // println!();
        let cave = cave_map.get(path.path.last().unwrap()).unwrap();
        for link in cave.connections.iter() {
            let linked_cave = cave_map.get(link).unwrap();
            // println!("Checking link {}, {}, {}", link, linked_cave.big, path.visited_small);
            // println!("{} {}", linked_cave.key, linked_cave.big);
            if linked_cave.big || !path.path.iter().any(|x| x == link) {
                // println!("Doesn't contain {}", link);
                let mut new_path: Vec<String> = path.path.clone();
                new_path.push(link.to_string());

                if link == "end" {
                    final_paths.push(new_path);
                } else {
                    path_queue.push_back(Path { path: new_path, visited_small: path.visited_small });
                }
            } else if !path.visited_small && link != "end" && link != "start" {
                let mut new_path: Vec<String> = path.path.clone();
                new_path.push(link.to_string());

                if link == "end" {
                    final_paths.push(new_path);
                } else {
                    path_queue.push_back(Path { path: new_path, visited_small: true });
                }
            }
        }
    }
    println!("{}", final_paths.len());
}
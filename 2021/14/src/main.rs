use std::fs;
use std::mem;
use std::collections::HashMap;
// use ansi_term::Colour::Red;

use regex::Regex;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut lines_iter = lines.iter();

    let starting = lines_iter.next().unwrap();
    let mut chars: Vec<char> = starting.chars().collect();

    lines_iter.next();

    let mut pattern_map: HashMap<String, char> = HashMap::new();

    let re = Regex::new(r"^(\w{2}) -> (\w)$").unwrap();
    while let Some(line) = lines_iter.next() {
        let cap = re.captures(line).unwrap();
        pattern_map.insert(cap[1].to_string(), cap[2].chars().nth(0).unwrap());
    }

    let mut cache: HashMap<(String, u32), HashMap<char, u64>> = HashMap::new();
    let mut total_counts: HashMap<char, u64> = HashMap::new();

    // let 
    for i in 0..lines[0].len() - 1 {
        let pair = String::from_iter(&chars[i..=i+1]);
        let counts = get_counts(&pair, 40, &pattern_map, &mut cache);
        for (key, val) in counts.iter() {
            let cur = total_counts.entry(*key).or_insert(0);
            *cur += val;
        }
    }
    for c in chars.iter() {
        let cur = total_counts.entry(*c).or_insert(0);
        *cur += 1;
    }
    println!("Final counts");
    let mut min = 0u64;
    let mut max = 0u64;
    for (key, val) in total_counts.iter() {
        println!("{}: {}", key, val);
        if *val > max { max = *val };
        if min == 0 || *val < min { min = *val };
    }

    println!("{}", max - min);

}

fn get_counts(pair: &str, depth: u32, patterns: &HashMap<String, char>, cache: &mut HashMap<(String, u32), HashMap<char, u64>>) -> HashMap<char, u64> {
    let mut ret_map = HashMap::<char, u64>::new();
    if depth == 0 {
        return ret_map;
    }
    if let Some(cached_val) = cache.get(&(pair.to_string(), depth)) {
        return cached_val.clone();
    } else if let Some(pattern_match) = patterns.get(pair) {
        let left = String::from_iter([pair.chars().nth(0).unwrap(), *pattern_match]);
        let right = String::from_iter([*pattern_match, pair.chars().nth(1).unwrap()]);
        let left_res = get_counts(&left, depth - 1, patterns, cache);
        let right_res = get_counts(&right, depth - 1, patterns, cache);
        for (key, val) in left_res.iter() {
            ret_map.insert(*key, *val);
        }
        for (key, val) in right_res.iter() {
            let cur = ret_map.entry(*key).or_insert(0);
            *cur += val;
        }
        *ret_map.entry(*pattern_match).or_insert(0) += 1;
        cache.insert((pair.to_string(), depth), ret_map.clone());
    }

    return ret_map;
}

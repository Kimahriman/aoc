use std::collections::HashMap;

fn factors(mut value: u64) -> HashMap<u64, usize> {
    let mut facts: Vec<u64> = Vec::new();
    let mut factor = 2;

    while value > 1 {
        if value % factor == 0 {
            value /= factor;
            facts.push(factor);
        } else {
            factor += 1;
        }
    }

    let mut map: HashMap<u64, usize> = Default::default();
    for factor in facts.into_iter() {
        let new_value = map.remove(&factor).unwrap_or(0) + 1;
        map.insert(factor, new_value);
    }
    map
}

fn main() {
    let contents = std::fs::read_to_string("inputs/8.txt").unwrap();

    let mut lines = contents.lines();

    let path = lines.next().unwrap().to_string();

    lines.next().unwrap();

    let mut paths: HashMap<String, (String, String)> = Default::default();
    let path_pattern = regex::Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    for line in lines {
        let captures = path_pattern.captures(line).unwrap();
        paths.insert(
            captures[1].to_string(),
            (captures[2].to_string(), captures[3].to_string()),
        );
    }

    let mut pos = "AAA".to_string();
    let mut steps = 0;
    while pos != "ZZZ" {
        let dir = path.chars().nth(steps % path.len()).unwrap();
        let (left, right) = paths.get(&pos).unwrap();
        pos = match dir {
            'L' => left.clone(),
            'R' => right.clone(),
            _ => panic!(),
        };
        steps += 1;
    }

    println!("{}", steps);

    let mut positions: Vec<String> = Vec::new();
    for p in paths.keys() {
        if p.ends_with('A') {
            positions.push(p.clone());
        }
    }

    println!("Num starting positions: {}", positions.len());

    let mut pos_factors: Vec<HashMap<u64, usize>> = Default::default();
    for pos in positions.iter() {
        let mut cur_pos = pos.clone();
        let mut cur_steps = 0;
        loop {
            let dir = path.chars().nth(cur_steps % path.len()).unwrap();
            let (left, right) = paths.get(&cur_pos).unwrap();
            cur_pos = match dir {
                'L' => left.clone(),
                'R' => right.clone(),
                _ => panic!(),
            };
            cur_steps += 1;
            if cur_pos.ends_with('Z') {
                pos_factors.push(factors(cur_steps as u64));
                break;
            }
        }
    }

    let mut combined_factor: HashMap<u64, usize> = Default::default();
    for map in pos_factors.into_iter() {
        for (key, value) in map.into_iter() {
            let max_value = usize::max(combined_factor.remove(&key).unwrap_or(0), value);
            combined_factor.insert(key, max_value);
        }
    }

    let mut product = 1;
    for (key, value) in combined_factor.into_iter() {
        for _ in 0..value {
            product *= key;
        }
    }
    println!("{}", product);
}

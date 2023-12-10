fn simulate(time: u64) -> Vec<u64> {
    (0..=time).map(|s| s * (time - s)).collect()
}

fn main() {
    let contents = std::fs::read_to_string("inputs/6.txt").unwrap();

    let pattern = regex::Regex::new(r"\w+:\s+(\d.+)$").unwrap();

    let mut lines = contents.lines();

    let captures = pattern.captures(lines.next().unwrap()).unwrap();

    let times: Vec<u64> = regex::Regex::new(r"\s+")
        .unwrap()
        .split(&captures[1])
        .map(|n| n.parse().unwrap())
        .collect();

    let captures = pattern.captures(lines.next().unwrap()).unwrap();

    let distances: Vec<u64> = regex::Regex::new(r"\s+")
        .unwrap()
        .split(&captures[1])
        .map(|n| n.parse().unwrap())
        .collect();

    let mut total = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        let mut sum = 0;
        for d in simulate(*time).iter() {
            if d > distance {
                sum += 1;
            }
        }
        total *= sum;
    }

    println!("{}", total);

    let mut combined_time = String::new();
    let mut combined_dist = String::new();

    for time in times.iter() {
        combined_time.push_str(&time.to_string());
    }
    for dist in distances.iter() {
        combined_dist.push_str(&dist.to_string());
    }

    let time: u64 = combined_time.parse().unwrap();
    let dist: u64 = combined_dist.parse().unwrap();
    total = 0;
    for d in simulate(time).iter() {
        if *d > dist {
            total += 1;
        }
    }
    println!("{}", total);
}

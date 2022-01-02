use std::fs;
use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Clone, Debug)]
struct Scanner {
    id: u32,
    orientation: i32,
    beacons: [Vec<[i32; 3]>; 24],
    offset: [i32; 3]
}

impl Scanner {
    fn new(id: u32) -> Self {
        let beacons: [Vec<[i32; 3]>; 24] = Default::default();
        Scanner { id, orientation: -1, beacons, offset: [0; 3] }
    }

    fn add_beacon(&mut self, beacon: [i32; 3]) {
        for (i, rot) in get_rotations(&beacon).into_iter().enumerate() {
            self.beacons[i].push(rot);
        }
    }

    fn matches_scanner(&self, other: &Scanner) -> (i32, [i32; 3]) {
        assert_ne!(self.orientation, -1);
        for (i, other_beacons) in other.beacons.iter().enumerate() {
            let mut match_count = HashMap::<[i32; 3], u32>::new();
            for other_beacon in other_beacons {
                for beacon in self.beacons[self.orientation as usize].iter() {
                    let beacon_diff = [
                        beacon[0] - other_beacon[0],
                        beacon[1] - other_beacon[1],
                        beacon[2] - other_beacon[2]
                    ];
                    *match_count.entry(beacon_diff).or_insert(0) += 1;
                }
            }
            assert!(match_count.values().filter(|x| **x >= 12).count() <= 1);
            for (m, count) in match_count.iter() {
                if *count >= 12 {
                    println!("Found match between {} and {} with offset {:?} and cur offset {:?}", self.id, other.id, *m, self.offset);
                    return (i as i32, [
                        m[0] + self.offset[0],
                        m[1] + self.offset[1],
                        m[2] + self.offset[2]
                    ]);
                }
            }
        }
        (-1, [0; 3])
    }
}

// static ROTATION_MULTIPLIERS = [[1, -1], [-1, 1], ]

fn get_rotations(beacon: &[i32; 3]) -> Vec<[i32; 3]> {
    let mut rotations = Vec::<[i32; 3]>::new();
    
    for j in 0..4 {
        let tmp_beacon = rotate_around_y(&beacon, j);
        for i in 0..4 {
            rotations.push(rotate_around_z(&tmp_beacon, i));
        }
    }

    for j in [1, 3] {
        let tmp_beacon = rotate_around_x(&beacon, j);
        for i in 0..4 {
            rotations.push(rotate_around_z(&tmp_beacon, i));
        }
    }

    rotations
}

fn rotate_around_z(beacon: &[i32; 3], rotations: u32) -> [i32; 3] {
    let mut new_beacon = beacon.clone();

    let mut new_x = new_beacon[0];
    let mut new_y = new_beacon[1];

    if rotations == 1 {
        new_x = new_beacon[1] * -1;
        new_y = new_beacon[0];
    } else if rotations == 2 {
        new_x = new_beacon[0] * -1;
        new_y = new_beacon[1] * -1;
    } else if rotations == 3 {
        new_x = new_beacon[1];
        new_y = new_beacon[0] * -1;
    }
    new_beacon[0] = new_x;
    new_beacon[1] = new_y;

    new_beacon
}

fn rotate_around_y(beacon: &[i32; 3], rotations: u32) -> [i32; 3] {
    let mut new_beacon = beacon.clone();

    let mut new_x = new_beacon[0];
    let mut new_z = new_beacon[2];

    if rotations == 1 {
        new_x = new_beacon[2] * -1;
        new_z = new_beacon[0];
    } else if rotations == 2 {
        new_x = new_beacon[0] * -1;
        new_z = new_beacon[2] * -1;
    } else if rotations == 3 {
        new_x = new_beacon[2];
        new_z = new_beacon[0] * -1;
    }
    new_beacon[0] = new_x;
    new_beacon[2] = new_z;

    new_beacon
}

fn rotate_around_x(beacon: &[i32; 3], rotations: u32) -> [i32; 3] {
    let mut new_beacon = beacon.clone();

    let mut new_z = new_beacon[2];
    let mut new_y = new_beacon[1];

    if rotations == 1 {
        new_z = new_beacon[1] * -1;
        new_y = new_beacon[2];
    } else if rotations == 2 {
        new_z = new_beacon[2] * -1;
        new_y = new_beacon[1] * -1;
    } else if rotations == 3 {
        new_z = new_beacon[1];
        new_y = new_beacon[2] * -1;
    }
    new_beacon[2] = new_z;
    new_beacon[1] = new_y;

    new_beacon
}

fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut scanners: Vec<Scanner> = Vec::new();

    let re = Regex::new(r"^--- scanner (\d+) ---$").unwrap();
    let beacon_re = Regex::new(r"^(-?\d+),(-?\d+),(-?\d+)$").unwrap();
    let mut cur_scanner: Scanner = Scanner::new(0);
    for line in lines.iter() {
        if let Some(cap) = re.captures(line) {
            cur_scanner = Scanner::new(cap[1].parse().unwrap());
        } else if *line == "" {
            scanners.push(cur_scanner.clone());
        } else if let Some(cap) = beacon_re.captures(line) {
            let beacon: [i32; 3] = [
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap()
            ];
            cur_scanner.add_beacon(beacon);
        }
    }
    scanners.push(cur_scanner);

    let mut resolved_scanners: Vec<Scanner> = Vec::new();
    let mut starting_scanner = scanners.remove(0);
    starting_scanner.orientation = 0;
    starting_scanner.offset = [0, 0, 0];
    resolved_scanners.push(starting_scanner);

    while !scanners.is_empty() {
        for i in 0..scanners.len() {
            let test_scanner = &scanners[i];

            let mut orientation = -1;
            let mut offset: [i32; 3] = [0; 3];
            for resolved_scanner in resolved_scanners.iter() {
                let res = resolved_scanner.matches_scanner(test_scanner);
                orientation = res.0;
                offset = res.1;
                if orientation >= 0 { break; }
            }
            if orientation >= 0 {
                let mut moved_scanner = scanners.remove(i);
                println!("Found match {}: {:?}", moved_scanner.id, offset);
                moved_scanner.orientation = orientation;
                moved_scanner.offset = offset;
                resolved_scanners.push(moved_scanner);
                break;
            }
        }
    }
    println!("Resolved all scanners");

    let mut beacon_set = HashSet::<[i32; 3]>::new();
    for scanner in resolved_scanners.iter() {
        for beacon in scanner.beacons[scanner.orientation as usize].iter() {
            let abs_pos = [
                beacon[0] + scanner.offset[0],
                beacon[1] + scanner.offset[1],
                beacon[2] + scanner.offset[2]
            ];
            beacon_set.insert(abs_pos);
        }
    }
    println!("Found {} total beacons", beacon_set.len());

    let mut max_dist = 0;
    for scanner in resolved_scanners.iter() {
        for other in resolved_scanners.iter() {
            if scanner.id == other.id { continue }
            let dist = (scanner.offset[0] - other.offset[0]).abs() + (scanner.offset[1] - other.offset[1]).abs() + (scanner.offset[2] - other.offset[2]).abs();
            if dist > max_dist { max_dist = dist };
        }
    }
    println!("Maximum distance: {}", max_dist);
}

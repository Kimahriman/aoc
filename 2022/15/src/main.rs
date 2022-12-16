use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use regex::Regex;

fn hamming_dist(a: &(i32, i32), b: &(i32, i32)) -> i32 {
    return (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

struct Sensor {
    sensor_pos: (i32, i32),
    beacon_pos: (i32, i32),
    dist: i32
}

impl Sensor {
    fn covers(&self, pos: &(i32, i32)) -> bool {
        let dist = hamming_dist(&self.sensor_pos, pos);
        return dist <= self.dist && self.beacon_pos != *pos
    }

    fn overlaps(&self, pos: &(i32, i32)) -> bool {
        let dist = hamming_dist(&self.sensor_pos, pos);
        return dist <= self.dist
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let re = Regex::new(r"Sensor at x=(\d+), y=(\d+): closest beacon is at x=(-?\d+), y=(\d+)").unwrap();

    let y_find = 2000000;
    // let y_find = 10;

    let x_mult: i64 = 4000000;
    let total_size = 4000000;
    // let total_size = 20;

    let mut sensors: Vec<Sensor> = Vec::new();

    let mut min_x = i32::max_value();
    let mut max_x = 0;

    for line in lines.iter() {
        println!("{}", line);
        let capt = re.captures(&line).unwrap();
        let sensor_x: i32 = capt.get(1).unwrap().as_str().parse().unwrap();
        let sensor_y: i32 = capt.get(2).unwrap().as_str().parse().unwrap();
        let beacon_x: i32 = capt.get(3).unwrap().as_str().parse().unwrap();
        let beacon_y: i32 = capt.get(4).unwrap().as_str().parse().unwrap();

        if sensor_x < min_x {
            min_x = sensor_x;
        }
        if beacon_x < min_x {
            min_x = beacon_x;
        }
        if sensor_x > max_x {
            max_x = sensor_x;
        }
        if beacon_x > max_x {
            max_x = beacon_x;
        }
        
        let dist = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();

        sensors.push(Sensor { sensor_pos: (sensor_x, sensor_y), beacon_pos: (beacon_x, beacon_y), dist });
    }

    let mut positions: i32 = 0;

    // Start at the min x and go left until something isn't covered
    let mut x = min_x;
    loop {
        if sensors.iter().any(|s| s.covers(&(x, y_find))) {
            positions += 1;
            x -= 1;
        } else {
            break;
        }
    }

    // Go to the max x and check them all
    x = min_x + 1;
    while x <= max_x {
        if sensors.iter().any(|s| s.covers(&(x, y_find))) {
            positions += 1;
        }
        x += 1;
    }

    // Go to the right of max x until it's not covered
    loop {
        if sensors.iter().any(|s| s.covers(&(x, y_find))) {
            positions += 1;
            x += 1;
        } else {
            break;
        }
    }

    println!("{}", positions);

    for sensor in sensors.iter() {
        let mut x = sensor.sensor_pos.0;
        let mut y = sensor.sensor_pos.1 + sensor.dist + 1;

        while y > sensor.sensor_pos.1 {
            if x >= 0 && x <= total_size && y >= 0 && y <= total_size && sensors.iter().all(|s| !s.overlaps(&(x, y))) {
                println!("{} {} {}", x, y, (x as i64 * x_mult + y as i64));
            }
            y -= 1;
            x += 1;
        }

        if x >= 0 && x <= total_size && y >= 0 && y <= total_size && sensors.iter().all(|s| !s.overlaps(&(x, y))) {
            println!("{} {} {}", x, y, (x as i64 * x_mult + y as i64));
        }

        y -= 1;
        x -= 1;

        while x > sensor.sensor_pos.0 {
            if x >= 0 && x <= total_size && y >= 0 && y <= total_size && sensors.iter().all(|s| !s.overlaps(&(x, y))) {
                println!("{} {} {}", x, y, (x as i64 * x_mult + y as i64));
            }
            y -= 1;
            x -= 1;
        }

        if x >= 0 && x <= total_size && y >= 0 && y <= total_size && sensors.iter().all(|s| !s.overlaps(&(x, y))) {
            println!("{} {} {}", x, y, (x as i64 * x_mult + y as i64));
        }

        x -= 1;
        y += 1;

        while y < sensor.sensor_pos.1 {
            if x >= 0 && x <= total_size && y >= 0 && y <= total_size && sensors.iter().all(|s| !s.overlaps(&(x, y))) {
                println!("{} {} {}", x, y, (x as i64 * x_mult + y as i64));
            }
            y += 1;
            x -= 1;
        }

        if x >= 0 && x <= total_size && y >= 0 && y <= total_size && sensors.iter().all(|s| !s.overlaps(&(x, y))) {
            println!("{} {} {}", x, y, (x as i64 * x_mult + y as i64));
        }

        x += 1;
        y += 1;

        while x < sensor.sensor_pos.0 {
            if x >= 0 && x <= total_size && y >= 0 && y <= total_size && sensors.iter().all(|s| !s.overlaps(&(x, y))) {
                println!("{} {} {}", x, y, (x as i64 * x_mult + y as i64));
            }
            y += 1;
            x += 1;
        }
    }

}

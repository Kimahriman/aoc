use std::cmp;
use std::fs;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
struct Cube {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32
}

impl Cube {
    fn new(min_x: i32, max_x: i32, min_y: i32, max_y: i32, min_z: i32, max_z: i32) -> Self {
        Cube { min_x, max_x, min_y, max_y, min_z, max_z }
    }

    // fn contains(&self, x: i32, y: i32, z: i32) -> bool {
    //    self.min_x <= x && x <= self.max_x && self.min_y <= y && y <= self.max_y && self.min_z <= z && z <= self.max_z
    // }

    fn intersects(&self, other: &Cube) -> bool {
        self.min_x <= other.max_x && self.max_x >= other.min_x &&
        self.min_y <= other.max_y && self.max_y >= other.min_y &&
        self.min_z <= other.max_z && self.max_z >= other.min_z
    }

    fn area(&self) -> u64 {
        let x = (self.max_x - self.min_x) as u64 + 1;
        let y = (self.max_y - self.min_y) as u64 + 1;
        let z = (self.max_z - self.min_z) as u64 + 1;
        println!("Getting area of {} {} {} {:?}", x, y, z, self);
        x * y * z
    }
}

struct Sequence {
    cubes: Vec<Cube> 
}

impl Sequence {
    fn new() -> Self {
        Sequence { cubes: Vec::new() }
    }

    fn add(&mut self, other: Cube) {
        // println!("Adding {:?}", other);
        let mut cubes_to_add = Vec::<Cube>::new();
        let mut intersects = false;
        for cube in self.cubes.iter() {
            if cube.intersects(&other) {
                // println!("Intersects {:?}", cube);
                let mut right = other.clone();
                right.min_x = cube.max_x + 1;
                if right.min_x <= right.max_x {
                    right.min_y = cmp::max(right.min_y, cube.min_y);
                    right.max_y = cmp::min(right.max_y, cube.max_y);
                    right.min_z = cmp::max(right.min_z, cube.min_z);
                    right.max_z = cmp::min(right.max_z, cube.max_z);
                    // println!("Pushing right {:?}", right);
                    cubes_to_add.push(right);
                }
                let mut left = other.clone();
                left.max_x = cube.min_x - 1;
                if left.max_x >= left.min_x {
                    left.min_y = cmp::max(left.min_y, cube.min_y);
                    left.max_y = cmp::min(left.max_y, cube.max_y);
                    left.min_z = cmp::max(left.min_z, cube.min_z);
                    left.max_z = cmp::min(left.max_z, cube.max_z);
                    // println!("Pushing left {:?}", left);
                    cubes_to_add.push(left);
                }
                let mut top = other.clone();
                top.min_y = cube.max_y + 1;
                if top.min_y <= top.max_y {
                    // println!("Pushing top {:?}", top);
                    cubes_to_add.push(top);
                }
                let mut bottom = other.clone();
                bottom.max_y = cube.min_y - 1;
                if bottom.max_y >= bottom.min_y {
                    // println!("Pushing bottom {:?}", bottom);
                    cubes_to_add.push(bottom);
                }
                let mut front = other.clone();
                front.min_z = cube.max_z + 1;
                if front.min_z <= front.max_z {
                    front.min_y = cmp::max(front.min_y, cube.min_y);
                    front.max_y = cmp::min(front.max_y, cube.max_y);
                    // println!("Pushing front {:?}", front);
                    cubes_to_add.push(front);
                }
                let mut back = other.clone();
                back.max_z = cube.min_z - 1;
                if back.max_z >= back.min_z {
                    back.min_y = cmp::max(back.min_y, cube.min_y);
                    back.max_y = cmp::min(back.max_y, cube.max_y);
                    // println!("Pushing back {:?}", back);
                    cubes_to_add.push(back);
                }
                intersects = true;
                break;
            }
        }

        while let Some(c) = cubes_to_add.pop() {
            self.add(c);
        }
        if !intersects {
            self.cubes.push(other);
        }
    }

    fn remove(&mut self, other: Cube) {
        let mut cubes_to_add = Vec::<Cube>::new();
        self.cubes = self.cubes
            .iter()
            .filter(|cube| {
                if !cube.intersects(&other) {
                    return true
                }
                let mut right = *cube.clone();
                right.min_x = other.max_x + 1;
                if right.min_x <= right.max_x {
                    right.min_y = cmp::max(right.min_y, other.min_y);
                    right.max_y = cmp::min(right.max_y, other.max_y);
                    right.min_z = cmp::max(right.min_z, other.min_z);
                    right.max_z = cmp::min(right.max_z, other.max_z);
                    cubes_to_add.push(right);
                }
                let mut left = *cube.clone();
                left.max_x = other.min_x - 1;
                if left.min_x <= left.max_x {
                    left.min_y = cmp::max(left.min_y, other.min_y);
                    left.max_y = cmp::min(left.max_y, other.max_y);
                    left.min_z = cmp::max(left.min_z, other.min_z);
                    left.max_z = cmp::min(left.max_z, other.max_z);
                    cubes_to_add.push(left);
                }
                let mut top = *cube.clone();
                top.min_y = other.max_y + 1;
                if top.min_y <= top.max_y {
                    cubes_to_add.push(top);
                }
                let mut bottom = *cube.clone();
                bottom.max_y = other.min_y - 1;
                if bottom.min_y <= bottom.max_y {
                    cubes_to_add.push(bottom);
                }
                let mut front = *cube.clone();
                front.min_z = other.max_z + 1;
                if front.min_z <= front.max_z {
                    front.min_y = cmp::max(front.min_y, other.min_y);
                    front.max_y = cmp::min(front.max_y, other.max_y);
                    cubes_to_add.push(front);
                }
                let mut back = *cube.clone();
                back.max_z = other.min_z - 1;
                if back.min_z <= back.max_z {
                    back.min_y = cmp::max(back.min_y, other.min_y);
                    back.max_y = cmp::min(back.max_y, other.max_y);
                    cubes_to_add.push(back);
                }
                return false
            })
            .cloned()
            .collect();
        while let Some(c) = cubes_to_add.pop() {
            assert!(c.min_x <= c.max_x);
            assert!(c.min_y <= c.max_y);
            assert!(c.min_z <= c.max_z);
            self.add(c);
        }
    }
}
// 2758514936282235
// 2758514936282235
// 6105985270142509
fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut sequence = Sequence::new();

    let re = Regex::new(r"^(on|off) x=(.+)\.\.(.+),y=(.+)\.\.(.+),z=(.+)\.\.(.+)$").unwrap();
    for line in lines.iter() {
        if let Some(cap) = re.captures(line) {
            let on = cap[1].eq("on");
            let min_x: i32 = cap[2].parse().unwrap();
            let max_x: i32 = cap[3].parse().unwrap();
            let min_y: i32 = cap[4].parse().unwrap();
            let max_y: i32 = cap[5].parse().unwrap();
            let min_z: i32 = cap[6].parse().unwrap();
            let max_z: i32 = cap[7].parse().unwrap();

            let cube = Cube::new(min_x, max_x, min_y, max_y, min_z, max_z);
            if on {
                sequence.add(cube);
            } else {
                sequence.remove(cube);
            }
        }
    }

    let mut total = 0u64;

    for cube in sequence.cubes.iter() {
        total += cube.area();
    }
    println!("{}", total);
}


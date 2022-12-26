use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use regex::Regex;

#[derive(Debug)]
enum Space {
    Empty,
    Wall
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Dir {
    Right,
    Down,
    Left,
    Up
}

fn reverse_dir(dir: Dir) -> Dir {
    match dir {
        Dir::Right => Dir::Left,
        Dir::Left => Dir::Right,
        Dir::Up => Dir::Down,
        Dir::Down => Dir::Up
    }
}

struct Grid {
    map: HashMap<(i32, i32), Space>,
    start: (i32, i32),
    x: i32,
    y: i32,
    direction: Dir,
    face_lookup: HashMap<(i32, i32, Dir), (i32, i32, Dir)>,
    track: HashMap<(i32, i32), Dir>
}

impl Grid {
    fn new(grid_lines: &[&str]) -> Self {
        let mut map = HashMap::<(i32, i32), Space>::new();
        let mut start: Option<(i32, i32)> = None;

        for (y, line) in grid_lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    map.insert((x as i32, y as i32), Space::Empty);
                    if start.is_none() {
                        start = Some((x as i32, y as i32));
                    }
                } else if c == '#' {
                    map.insert((x as i32, y as i32), Space::Wall);
                }
            }
        }

        let (x, y) = start.unwrap();
        Grid { map, start: start.unwrap(), x, y, direction: Dir::Right, face_lookup: HashMap::new(), track: HashMap::new() }
    }

    fn reset(&mut self) {
        self.x = self.start.0;
        self.y = self.start.1;
        self.direction = Dir::Right;
        self.track = HashMap::new();
        self.track.insert((self.x, self.y), Dir::Right);
    }

    fn build_face_lookup(&mut self, corner_pairs: &[&[(i32, i32)]], edge_dirs: &[(Dir, Dir)]) {
        for (pair, dirs) in corner_pairs.iter().zip(edge_dirs.iter()) {
            let start1 = pair[0];
            let end1 = pair[1];
            let start2 = pair[2];
            let end2 = pair[3];

            // println!("{:?} {:?} {:?} {:?}", start1, end1, start2, end2);

            let range1: Vec<(i32, i32)> = if start1.0 == end1.0 {
                // X is the same, iterate over y
                let y_range: Vec<i32> = if start1.1 < end1.1 {
                    (start1.1..=end1.1).collect()
                } else {
                    (end1.1..=start1.1).rev().collect()
                };
                y_range.into_iter().map(|x| (start1.0, x)).collect()
            } else {
                // Y is the same, iterate over x
                let x_range: Vec<i32> = if start1.0 < end1.0 {
                    (start1.0..=end1.0).collect()
                } else {
                    (end1.0..=start1.0).rev().collect()
                };
                x_range.into_iter().map(|x| (x, start1.1)).collect()
            };

            let range2: Vec<(i32, i32)> = if start2.0 == end2.0 {
                // X is the same, iterate over y
                let y_range: Vec<i32> = if start2.1 < end2.1 {
                    (start2.1..=end2.1).collect()
                } else {
                    (end2.1..=start2.1).rev().collect()
                };
                y_range.into_iter().map(|x| (start2.0, x)).collect()
            } else {
                // Y is the same, iterate over x
                let x_range: Vec<i32> = if start2.0 < end2.0 {
                    (start2.0..=end2.0).collect()
                } else {
                    (end2.0..=start2.0).rev().collect()
                };
                x_range.into_iter().map(|x| (x, start2.1)).collect()
            };

            // let x_range1: Vec<i32> = if start1.0 < end1.0 {
            //     (start1.0..=end1.0).collect()
            // } else {
            //     (end1.0..=start1.0).rev().collect()
            // };
            // let x_range2: Vec<i32> = if start2.0 < end2.0 {
            //     (start2.0..=end2.0).collect()
            // } else {
            //     (end2.0..=start2.0).rev().collect()
            // };
            // let y_range1: Vec<i32> = if start1.1 < end1.1 {
            //     (start1.1..=end1.1).collect()
            // } else {
            //     (end1.1..=start1.1).rev().collect()
            // };
            // let y_range2: Vec<i32> = if start2.1 < end2.1 {
            //     (start2.1..=end2.1).collect()
            // } else {
            //     (end2.1..=start2.1).rev().collect()
            // };
            // println!("{:?} {:?} {:?} {:?}", x_range1, x_range2, y_range1, y_range2);

            // let range1 = x_range1.iter().zip(y_range1.iter());
            // let range2 = x_range2.iter().zip(y_range2.iter());

            // println!("{:?} {:?}", range1, range2);

            // for (x1, x2) in x_range1.iter().zip(x_range2.iter()) {
            //     for (y1, y2) in y_range1.iter().zip(y_range2.iter()) {
            for ((x1, y1), (x2, y2)) in range1.iter().zip(range2.iter()) {
                if self.face_lookup.contains_key(&(*x1, *y1, dirs.0)) {
                    println!("Found dupe at {} {} {:?}", x1, y1, dirs.0);
                }
                // println!("Inserting {} {} {:?}", x1, y1, dirs.0);
                self.face_lookup.insert((*x1, *y1, dirs.0), (*x2, *y2, reverse_dir(dirs.1)));
                self.face_lookup.insert((*x2, *y2, dirs.1), (*x1, *y1, reverse_dir(dirs.0)));    
            }
            // }
            // for p in range1.zip(range2) {
            //     let ((x1, y1), (x2, y2)) = p;
            //     if self.face_lookup.contains_key(&(*x1, *y1, dirs.0)) {
            //         println!("Found dupe at {} {} {:?}", x1, y1, dirs.0);
            //     }
            //     println!("Inserting {} {} {:?}", x1, y1, dirs.0);
            //     self.face_lookup.insert((*x1, *y1, dirs.0), (*x2, *y2, reverse_dir(dirs.1)));
            //     self.face_lookup.insert((*x2, *y2, dirs.1), (*x1, *y1, reverse_dir(dirs.0)));
            // }
        }
    }

    fn forward(&mut self, length: i32, cube: bool) {
        match self.direction {
            Dir::Right => self.move_right(length, cube),
            Dir::Down => self.move_down(length, cube),
            Dir::Left => self.move_left(length, cube),
            Dir::Up => self.move_up(length, cube)
        }
    }

    fn move_right(&mut self, length: i32, cube: bool) {
        for i in 0..length {
            self.track.insert((self.x, self.y), Dir::Right);
            if let Some(s) = self.map.get(&(self.x + 1, self.y)) {
                match s {
                    Space::Wall => return,
                    _ => self.x += 1
                }
            } else if !cube {
                // Find the left most space
                let mut x = self.x;
                while self.map.contains_key(&(x - 1, self.y)) {
                    x -= 1;
                }
                match self.map[&(x, self.y)] {
                    Space::Wall => return,
                    _ => self.x = x
                }
            } else {
                let (new_x, new_y, new_dir) = self.face_lookup.get(&(self.x, self.y, Dir::Right))
                    .expect(&format!("Failed to find edge mapping {} {} {:?}", self.x, self.y, Dir::Right));
                match self.map[&(*new_x, *new_y)] {
                    Space::Wall => return,
                    _ => {
                        self.direction = *new_dir;
                        self.x = *new_x;
                        self.y = *new_y;
                        println!("Changing to {} {} {:?}", self.x, self.y, self.direction);
                        self.forward(length - i - 1, cube);
                        return;
                    }
                }
            }
        }
    }

    fn move_down(&mut self, length: i32, cube: bool) {
        for i in 0..length {
            self.track.insert((self.x, self.y), Dir::Down);
            if let Some(s) = self.map.get(&(self.x, self.y + 1)) {
                match s {
                    Space::Wall => return,
                    _ => self.y += 1
                }
            } else if !cube {
                // Find the upper most space
                let mut y = self.y;
                while self.map.contains_key(&(self.x, y - 1)) {
                    y -= 1;
                }
                match self.map[&(self.x, y)] {
                    Space::Wall => return,
                    _ => self.y = y
                }
            } else {
                let (new_x, new_y, new_dir) = self.face_lookup.get(&(self.x, self.y, Dir::Down))
                    .expect(&format!("Failed to find edge mapping {} {} {:?}", self.x, self.y, Dir::Down));
                match self.map[&(*new_x, *new_y)] {
                    Space::Wall => return,
                    _ => {
                        self.direction = *new_dir;
                        self.x = *new_x;
                        self.y = *new_y;
                        self.forward(length - i - 1, cube);
                        return;
                    }
                }
            }
        }
    }

    fn move_left(&mut self, length: i32, cube: bool) {
        for i in 0..length {
            self.track.insert((self.x, self.y), Dir::Left);
            if let Some(s) = self.map.get(&(self.x - 1, self.y)) {
                match s {
                    Space::Wall => return,
                    _ => self.x -= 1
                }
            } else if !cube {
                // Find the left most space
                let mut x = self.x;
                while self.map.contains_key(&(x + 1, self.y)) {
                    x += 1;
                }
                match self.map[&(x, self.y)] {
                    Space::Wall => return,
                    _ => self.x = x
                }
            } else {
                let (new_x, new_y, new_dir) = self.face_lookup.get(&(self.x, self.y, Dir::Left))
                    .expect(&format!("Failed to find edge mapping {} {} {:?}", self.x, self.y, Dir::Left));
                match self.map[&(*new_x, *new_y)] {
                    Space::Wall => return,
                    _ => {
                        self.direction = *new_dir;
                        self.x = *new_x;
                        self.y = *new_y;
                        self.forward(length - i - 1, cube);
                        return;
                    }
                }
            }
        }
    }

    fn move_up(&mut self, length: i32, cube: bool) {
        for i in 0..length {
            self.track.insert((self.x, self.y), Dir::Up);
            if let Some(s) = self.map.get(&(self.x, self.y - 1)) {
                match s {
                    Space::Wall => return,
                    _ => self.y -= 1
                }
            } else if !cube {
                // Find the upper most space
                let mut y = self.y;
                while self.map.contains_key(&(self.x, y + 1)) {
                    y += 1;
                }
                match self.map[&(self.x, y)] {
                    Space::Wall => return,
                    _ => self.y = y
                }
            } else {
                let (new_x, new_y, new_dir) = self.face_lookup.get(&(self.x, self.y, Dir::Up))
                    .expect(&format!("Failed to find edge mapping {} {} {:?}", self.x, self.y, Dir::Up));
                match self.map[&(*new_x, *new_y)] {
                    Space::Wall => return,
                    _ => {
                        self.direction = *new_dir;
                        self.x = *new_x;
                        self.y = *new_y;
                        self.forward(length - i - 1, cube);
                        return;
                    }
                }
            }
        }
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up
        }
    }

    fn print(&self) {
        let mut max_x = 0;
        let mut max_y = 0;
        for ((x, y), _) in self.map.iter() {
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
        }
    
        for y in 0..=max_y {
            for x in 0..=max_x {
                if !self.map.contains_key(&(x, y)) {
                    print!(" ");
                } else if self.track.contains_key(&(x, y)) {
                    let c = match self.track[&(x, y)] {
                        Dir::Up => "^",
                        Dir::Right => ">",
                        Dir::Down => "v",
                        Dir::Left => "<"
                    };
                    print!("{}", c);
                } else {
                    let c = match self.map[&(x, y)] {
                        Space::Empty => ".",
                        Space::Wall => "#"
                    };
                    print!("{}", c);
                }
            }
            println!();
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    
    let (grid_lines, directions) = lines.split_at(lines.len() - 2);

    let mut grid = Grid::new(grid_lines);

    let path = directions[1];
    let mut buf = String::new();
    for c in path.chars() {
        if c == 'L' || c == 'R' {
            if !buf.is_empty() {
                grid.forward(buf.parse().unwrap(), false);
                buf.clear();
            }
            if c == 'L' {
                grid.turn_left();
            } else {
                grid.turn_right();
            }
        } else {
            buf += &c.to_string();
        }
    }

    if !buf.is_empty() {
        grid.forward(buf.parse().unwrap(), false);
        buf.clear();
    }

    println!("{} {} {:?}", grid.x + 1, grid.y + 1, grid.direction);
    let facing_val = match grid.direction {
        Dir::Right => 0,
        Dir::Down => 1,
        Dir::Left => 2,
        Dir::Up => 3
    };

    let password = 1000 * (grid.y + 1) + 4 * (grid.x + 1) + facing_val;
    println!("{}", password);

    grid.build_face_lookup(&[
        &[(50, 0), (99, 0), (0, 150), (0, 199)],
        // &[(99, 0), (99, 49), (100, 0), (100, 49)], inteior lines don't matter
        &[(149, 0), (149, 49), (99, 149), (99, 100)],
        &[(50, 0), (50, 49), (0, 149), (0, 100)],
        &[(50, 50), (50, 99), (0, 100), (49, 100)],
        &[(100, 49), (149, 49), (99, 50), (99, 99)],
        &[(0, 199), (49, 199), (100, 0), (149, 0)],
        &[(49, 150), (49, 199), (50, 149), (99, 149)],
        // &[(100, 49), (149, 49), (99, 50), (99, 99)],
        // &[(50, 49), (99, 49), (50, 50), (99, 50)], interior lines don't matter
        // &[(49, 100), (49, 149), (50, 100), (50, 149)] interior lines don't matter
    ], &[
        (Dir::Up, Dir::Left),
        (Dir::Right, Dir::Right),
        (Dir::Left, Dir::Left),
        (Dir::Left, Dir::Up),
        (Dir::Down, Dir::Right),
        (Dir::Down, Dir::Up),
        (Dir::Right, Dir::Down)
    ]);

    // grid.build_face_lookup(&[
    //     &[(8, 0), (11, 0), (3, 4), (0, 4)],
    //     &[(8, 0), (8, 3), (4, 4), (7, 4)],
    //     &[(11, 0), (11, 3), (15, 11), (15, 8)],
    //     &[(11, 4), (11, 7), (15, 8), (12, 8)],
    //     &[(15, 11), (12, 11), (0, 4), (0, 7)],
    //     &[(8, 11), (11, 11), (3, 7), (0, 7)],
    //     &[(8, 8), (8, 11), (7, 7), (4, 7)],
    // ], &[
    //     (Dir::Up, Dir::Up),
    //     (Dir::Left, Dir::Up),
    //     (Dir::Right, Dir::Right),
    //     (Dir::Right, Dir::Up),
    //     (Dir::Down, Dir::Left),
    //     (Dir::Down, Dir::Down),
    //     (Dir::Left, Dir::Down)
    // ]);

    grid.reset();

    for c in path.chars() {
        if c == 'L' || c == 'R' {
            if !buf.is_empty() {
                grid.forward(buf.parse().unwrap(), true);
                buf.clear();
                // grid.print();
                // println!();
            }
            if c == 'L' {
                grid.turn_left();
            } else {
                grid.turn_right();
            }
        } else {
            buf += &c.to_string();
        }
    }

    if !buf.is_empty() {
        grid.forward(buf.parse().unwrap(), true);
        buf.clear();
    }
    grid.print();

    println!("{} {} {:?}", grid.x + 1, grid.y + 1, grid.direction);
    let facing_val = match grid.direction {
        Dir::Right => 0,
        Dir::Down => 1,
        Dir::Left => 2,
        Dir::Up => 3
    };

    let password = 1000 * (grid.y + 1) + 4 * (grid.x + 1) + facing_val;
    println!("{}", password);
}

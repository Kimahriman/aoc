use std::{
    collections::HashMap,
    ops::{Add, AddAssign},
};

use geo::{Contains, Coord, LineString, Polygon};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Pos(i32, i32);

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl From<Pos> for Coord<f64> {
    fn from(value: Pos) -> Self {
        Coord {
            x: value.1 as f64,
            y: value.0 as f64,
        }
    }
}

#[derive(Debug)]
enum Pipe {
    Start,
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    Ground,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::Ground,
            _ => panic!(),
        }
    }
}

impl Pipe {
    fn follow(&self, direction: Pos) -> Option<Pos> {
        match (self, direction) {
            (Self::Vertical, Pos(-1, 0)) => Some(Pos(-1, 0)),
            (Self::Vertical, Pos(1, 0)) => Some(Pos(1, 0)),
            (Self::Horizontal, Pos(0, -1)) => Some(Pos(0, -1)),
            (Self::Horizontal, Pos(0, 1)) => Some(Pos(0, 1)),
            (Self::NE, Pos(1, 0)) => Some(Pos(0, 1)),
            (Self::NE, Pos(0, -1)) => Some(Pos(-1, 0)),
            (Self::NW, Pos(1, 0)) => Some(Pos(0, -1)),
            (Self::NW, Pos(0, 1)) => Some(Pos(-1, 0)),
            (Self::SW, Pos(0, 1)) => Some(Pos(1, 0)),
            (Self::SW, Pos(-1, 0)) => Some(Pos(0, -1)),
            (Self::SE, Pos(0, -1)) => Some(Pos(1, 0)),
            (Self::SE, Pos(-1, 0)) => Some(Pos(0, 1)),
            _ => None,
        }
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/10.txt").unwrap();

    let mut grid: HashMap<Pos, Pipe> = Default::default();

    let mut start_pos = Pos(0, 0);
    let mut points: Vec<Pos> = Vec::new();

    for (row, line) in contents.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            let pipe = char.into();
            if let Pipe::Start = pipe {
                start_pos = Pos(row as i32, col as i32);
            }
            grid.insert(Pos(row as i32, col as i32), pipe);
        }
    }

    // Find a starting pipe connected to S
    let mut current_dir = Pos(0, 0);
    for pos in [Pos(-1, 0), Pos(1, 0), Pos(0, -1), Pos(0, 1)] {
        if let Some(pipe) = grid.get(&(start_pos + pos)) {
            if pipe.follow(pos).is_some() {
                current_dir = pos;
                break;
            }
        }
    }

    assert!(current_dir != Pos(0, 0));

    let mut steps = 0;
    let mut current_pos = start_pos;
    points.push(start_pos);
    loop {
        steps += 1;
        current_pos += current_dir;

        if current_pos == start_pos {
            break;
        }

        points.push(current_pos);

        current_dir = grid
            .get(&current_pos)
            .expect("Missing path in grid")
            .follow(current_dir)
            .expect("Path didn't connect");
    }

    println!("Total steps: {}. Furthest: {}", steps, (steps + 1) / 2);

    let line = LineString::new(points.iter().cloned().map(Into::into).collect());
    let polygon = Polygon::new(line, vec![]);

    let interiors = grid
        .keys()
        .filter(|pos| !points.contains(*pos) && polygon.contains(&Into::<Coord>::into(**pos)))
        .count();

    println!("{}", interiors);
}

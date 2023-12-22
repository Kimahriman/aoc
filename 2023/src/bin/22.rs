use std::{collections::HashMap, convert::Infallible, str::FromStr};

struct Brick {
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

impl Brick {
    fn cells(&self) -> impl Iterator<Item = (usize, usize, usize)> {
        if self.start.0 != self.end.0 {
            return (usize::min(self.start.0, self.end.0)..=usize::max(self.start.0, self.end.0))
                .map(|x| (x, self.start.1, self.start.2));
        } else if self.start.1 != self.end.1 {
            return (usize::min(self.start.1, self.end.1)..=usize::max(self.start.1, self.end.1))
                .map(|y| (self.start.0, y, self.start.2));
        } else {
            return (usize::min(self.start.2, self.end.2)..=usize::max(self.start.2, self.end.2))
                .map(|z| (self.start.0, self.start.1, z));
        }
    }
}

impl FromStr for Brick {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('~');
        let mut left_split = split.next().unwrap().split(',');
        let mut right_split = split.next().unwrap().split(',');

        let left_x = left_split.next().unwrap().parse().unwrap();
        let left_y = left_split.next().unwrap().parse().unwrap();
        let left_z = left_split.next().unwrap().parse().unwrap();

        let right_x = right_split.next().unwrap().parse().unwrap();
        let right_y = right_split.next().unwrap().parse().unwrap();
        let right_z = right_split.next().unwrap().parse().unwrap();

        Ok(Brick {
            start: (left_x, left_y, left_z),
            end: (right_x, right_y, right_z),
        })
    }
}

struct Grid {
    state: HashMap<(usize, usize, usize), usize>,
    bricks: Vec<Brick>,
}

impl Grid {
    fn new(bricks: Vec<Brick>) -> Self {
        let mut state = HashMap::new();

        for brick in bricks.iter() {}

        Self { state, bricks }
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/22.txt").unwrap();

    let bricks: Vec<Brick> = contents.lines().map(|l| l.parse().unwrap()).collect();
}

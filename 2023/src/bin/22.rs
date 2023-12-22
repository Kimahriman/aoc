use std::{collections::HashMap, convert::Infallible, str::FromStr};

type Position = (usize, usize, usize);

#[derive(Clone)]
struct Brick {
    cells: Vec<Position>,
}

impl Brick {
    fn new(start: Position, end: Position) -> Self {
        let cells = if start.0 != end.0 {
            assert_eq!(start.1, end.1);
            assert_eq!(start.2, end.2);

            (usize::min(start.0, end.0)..=usize::max(start.0, end.0))
                .map(|x| (x, start.1, start.2))
                .collect()
        } else if start.1 != end.1 {
            assert_eq!(start.0, end.0);
            assert_eq!(start.2, end.2);

            (usize::min(start.1, end.1)..=usize::max(start.1, end.1))
                .map(|y| (start.0, y, start.2))
                .collect()
        } else {
            assert_eq!(start.0, end.0);
            assert_eq!(start.1, end.1);

            (usize::min(start.2, end.2)..=usize::max(start.2, end.2))
                .map(|z| (start.0, start.1, z))
                .collect()
        };

        Self { cells }
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

        Ok(Brick::new(
            (left_x, left_y, left_z),
            (right_x, right_y, right_z),
        ))
    }
}

#[derive(Clone)]
struct Grid {
    state: HashMap<Position, usize>,
    bricks: HashMap<usize, Brick>,
}

impl Grid {
    fn new(bricks: Vec<Brick>) -> Self {
        let mut state = HashMap::new();

        for (i, brick) in bricks.iter().enumerate() {
            for cell in brick.cells.iter() {
                state.insert(*cell, i);
            }
        }

        Self {
            state,
            bricks: bricks.into_iter().enumerate().collect(),
        }
    }

    fn fall(&mut self) -> usize {
        let max_z = *self.state.values().max().unwrap();

        let mut cur_z = 2usize;
        let mut dropped_bricks = 0;

        while cur_z <= max_z {
            let brick_ids: Vec<usize> = self
                .state
                .iter()
                .filter_map(|(pos, id)| if pos.2 == cur_z { Some(*id) } else { None })
                .collect();

            for brick_id in brick_ids {
                let mut dropped = false;
                while self.can_drop(brick_id) {
                    dropped = true;
                    for cell in self.bricks.get_mut(&brick_id).unwrap().cells.iter_mut() {
                        self.state.remove(cell);
                        cell.2 -= 1;
                        self.state.insert(*cell, brick_id);
                    }
                }
                if dropped {
                    dropped_bricks += 1;
                }
            }

            cur_z += 1;
        }
        dropped_bricks
    }

    fn can_drop(&self, brick_id: usize) -> bool {
        for mut cell in self.bricks.get(&brick_id).unwrap().cells.iter().cloned() {
            if cell.2 <= 1 {
                return false;
            }
            cell.2 -= 1;
            if self.state.get(&cell).is_some_and(|id| brick_id != *id) {
                return false;
            }
        }
        true
    }

    fn remove(&mut self, brick_id: usize) -> Brick {
        for cell in self.bricks.get(&brick_id).unwrap().cells.iter() {
            self.state.remove(cell);
        }
        self.bricks.remove(&brick_id).unwrap()
    }

    fn add(&mut self, brick_id: usize, brick: Brick) {
        for cell in brick.cells.iter() {
            self.state.insert(*cell, brick_id);
        }
        self.bricks.insert(brick_id, brick);
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/22.txt").unwrap();

    let bricks: Vec<Brick> = contents.lines().map(|l| l.parse().unwrap()).collect();
    let mut grid = Grid::new(bricks);
    grid.fall();

    let mut can_remove = 0;
    for i in 0..grid.bricks.len() {
        let brick = grid.remove(i);
        let mut can_drop = false;
        for j in 0..grid.bricks.len() {
            if i == j {
                continue;
            }
            if grid.can_drop(j) {
                can_drop = true;
                break;
            }
        }
        if !can_drop {
            can_remove += 1;
        }
        grid.add(i, brick);
    }

    println!("{}", can_remove);

    let mut total_dropped = 0;
    for i in 0..grid.bricks.len() {
        let mut simulate = grid.clone();
        simulate.remove(i);
        total_dropped += simulate.fall();
    }

    println!("{}", total_dropped);
}

use std::{collections::HashMap, fmt::Debug};

use bitvec::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone)]
enum State {
    Empty,
    Round,
    Cube,
}

#[derive(Clone)]
struct Platform {
    grid: Vec<Vec<State>>,
}

impl Debug for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for cell in row.iter() {
                f.write_str(match cell {
                    State::Empty => ".",
                    State::Round => "O",
                    State::Cube => "#",
                })?
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Platform {
    fn tilt_up(&mut self) {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if self.grid[row][col] == State::Round {
                    self.move_rock_up(row, col);
                }
            }
        }
    }

    fn move_rock_up(&mut self, mut row: usize, col: usize) {
        assert_eq!(self.grid[row][col], State::Round);
        while row > 0 {
            if self.grid[row - 1][col] == State::Empty {
                self.grid[row - 1][col] = State::Round;
                self.grid[row][col] = State::Empty;
            } else {
                break;
            }

            row -= 1;
        }
    }

    fn tilt_down(&mut self) {
        for row in (0..self.grid.len()).rev() {
            for col in 0..self.grid[row].len() {
                if self.grid[row][col] == State::Round {
                    self.move_rock_down(row, col);
                }
            }
        }
    }

    fn move_rock_down(&mut self, mut row: usize, col: usize) {
        assert_eq!(self.grid[row][col], State::Round);
        while row < self.grid.len() - 1 {
            if self.grid[row + 1][col] == State::Empty {
                self.grid[row + 1][col] = State::Round;
                self.grid[row][col] = State::Empty;
            } else {
                break;
            }

            row += 1;
        }
    }

    fn tilt_left(&mut self) {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if self.grid[row][col] == State::Round {
                    self.move_rock_left(row, col);
                }
            }
        }
    }

    fn move_rock_left(&mut self, row: usize, mut col: usize) {
        assert_eq!(self.grid[row][col], State::Round);
        while col > 0 {
            if self.grid[row][col - 1] == State::Empty {
                self.grid[row][col - 1] = State::Round;
                self.grid[row][col] = State::Empty;
            } else {
                break;
            }

            col -= 1;
        }
    }

    fn tilt_right(&mut self) {
        for row in 0..self.grid.len() {
            for col in (0..self.grid[row].len()).rev() {
                if self.grid[row][col] == State::Round {
                    self.move_rock_right(row, col);
                }
            }
        }
    }

    fn move_rock_right(&mut self, row: usize, mut col: usize) {
        assert_eq!(self.grid[row][col], State::Round);
        while col < self.grid[row].len() - 1 {
            if self.grid[row][col + 1] == State::Empty {
                self.grid[row][col + 1] = State::Round;
                self.grid[row][col] = State::Empty;
            } else {
                break;
            }

            col += 1;
        }
    }

    fn cycle(&mut self) {
        self.tilt_up();
        self.tilt_left();
        self.tilt_down();
        self.tilt_right();
    }

    fn load(&self) -> u32 {
        let mut sum = 0;
        for (r, row) in self.grid.iter().enumerate() {
            for cell in row.iter() {
                if *cell == State::Round {
                    sum += self.grid.len() - r;
                }
            }
        }

        sum as u32
    }

    fn bitvec(&self) -> BitVec {
        let mut bv = BitVec::with_capacity(self.grid.len() * self.grid[0].len());
        for row in self.grid.iter() {
            for cell in row.iter() {
                bv.push(*cell == State::Round);
            }
        }
        bv
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/14.txt").unwrap();

    let grid = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => State::Empty,
                    'O' => State::Round,
                    '#' => State::Cube,
                    _ => panic!("Unknown space"),
                })
                .collect()
        })
        .collect();

    let mut platform = Platform { grid };

    let mut platform1 = platform.clone();
    platform1.tilt_up();
    println!("{}", platform1.load());

    let mut map: HashMap<BitVec, u32> = Default::default();
    map.insert(platform.bitvec(), 0);

    let mut cycle = 0;
    let cycle_length;
    loop {
        cycle += 1;
        platform.cycle();
        let bv = platform.bitvec();
        if let Some(first) = map.get(&bv) {
            println!("Found cycle at {} with len {}", cycle, cycle - *first);
            cycle_length = cycle - *first;
            break;
        } else {
            map.insert(bv, cycle);
        }
    }

    cycle += ((1000000000 - cycle) / cycle_length) * cycle_length;

    while cycle < 1000000000 {
        platform.cycle();
        cycle += 1;
    }

    println!("{}", platform.load());
}

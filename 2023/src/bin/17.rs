use std::collections::{BinaryHeap, HashMap, HashSet};

struct Position {
    pos: (i16, i16),
    dir: (i16, i16),
    heat_loss: i32,
    continuous: u8,
    prev: Vec<(i16, i16)>,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heat_loss.cmp(&other.heat_loss)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.heat_loss.partial_cmp(&other.heat_loss)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.heat_loss == other.heat_loss
    }
}

impl Eq for Position {}

struct Town {
    grid: HashMap<(i16, i16), i32>,
    queue: BinaryHeap<Position>,
    cache: HashSet<((i16, i16), (i16, i16), u8)>,
}

impl Town {
    fn new(grid: HashMap<(i16, i16), i32>) -> Self {
        Self {
            grid,
            queue: BinaryHeap::new(),
            cache: Default::default(),
        }
    }

    fn reset(&mut self) {
        self.queue.clear();
        self.cache.clear();
    }

    fn step(&mut self, pos: &Position) {
        let mut new_prev = pos.prev.clone();
        new_prev.push(pos.pos);
        if pos.continuous < 3 {
            let new_pos = (pos.pos.0 + pos.dir.0, pos.pos.1 + pos.dir.1);
            if let Some(tile) = self.grid.get(&new_pos) {
                let heat_loss = pos.heat_loss - *tile;
                self.queue.push(Position {
                    pos: new_pos,
                    dir: pos.dir,
                    heat_loss,
                    continuous: pos.continuous + 1,
                    prev: new_prev.clone(),
                })
            }
        }
        let new_dir = (pos.dir.1, pos.dir.0);
        let new_pos = (pos.pos.0 + new_dir.0, pos.pos.1 + new_dir.1);
        if let Some(tile) = self.grid.get(&new_pos) {
            let heat_loss = pos.heat_loss - *tile;
            self.queue.push(Position {
                pos: new_pos,
                dir: new_dir,
                heat_loss,
                continuous: 1,
                prev: new_prev.clone(),
            })
        }

        let new_dir = (-pos.dir.1, -pos.dir.0);
        let new_pos = (pos.pos.0 + new_dir.0, pos.pos.1 + new_dir.1);
        if let Some(tile) = self.grid.get(&new_pos) {
            let heat_loss = pos.heat_loss - *tile;
            self.queue.push(Position {
                pos: new_pos,
                dir: new_dir,
                heat_loss,
                continuous: 1,
                prev: new_prev,
            })
        }
    }

    fn step_ultra(&mut self, pos: &Position) {
        let mut new_prev = pos.prev.clone();
        new_prev.push(pos.pos);

        if pos.continuous < 10 {
            let new_pos = (pos.pos.0 + pos.dir.0, pos.pos.1 + pos.dir.1);
            if let Some(tile) = self.grid.get(&new_pos) {
                let heat_loss = pos.heat_loss - *tile;
                self.queue.push(Position {
                    pos: new_pos,
                    dir: pos.dir,
                    heat_loss,
                    continuous: pos.continuous + 1,
                    prev: new_prev.clone(),
                })
            }
        }
        if pos.continuous >= 4 {
            let new_dir = (pos.dir.1, pos.dir.0);
            let new_pos = (pos.pos.0 + new_dir.0, pos.pos.1 + new_dir.1);
            if let Some(tile) = self.grid.get(&new_pos) {
                let heat_loss = pos.heat_loss - *tile;
                self.queue.push(Position {
                    pos: new_pos,
                    dir: new_dir,
                    heat_loss,
                    continuous: 1,
                    prev: new_prev.clone(),
                })
            }

            let new_dir = (-pos.dir.1, -pos.dir.0);
            let new_pos = (pos.pos.0 + new_dir.0, pos.pos.1 + new_dir.1);
            if let Some(tile) = self.grid.get(&new_pos) {
                let heat_loss = pos.heat_loss - *tile;
                self.queue.push(Position {
                    pos: new_pos,
                    dir: new_dir,
                    heat_loss,
                    continuous: 1,
                    prev: new_prev,
                })
            }
        }
    }

    fn search(&mut self, ultra: bool) -> (i32, Position) {
        let end_pos = *self.grid.keys().max().unwrap();
        self.queue.push(Position {
            pos: (0, 0),
            dir: (0, 1),
            heat_loss: 0,
            continuous: 0,
            prev: Vec::new(),
        });

        let mut current_heat = 0;
        while let Some(next) = self.queue.pop() {
            if self.cache.contains(&(next.pos, next.dir, next.continuous)) {
                continue;
            } else {
                self.cache.insert((next.pos, next.dir, next.continuous));
            }

            if next.heat_loss < current_heat {
                current_heat = next.heat_loss;
                println!("On heat loss {}", current_heat);
            }

            if ultra {
                if next.pos == end_pos && next.continuous >= 4 {
                    return (-next.heat_loss, next);
                }
                self.step_ultra(&next);
            } else {
                if next.pos == end_pos {
                    return (-next.heat_loss, next);
                }
                self.step(&next);
            }
        }

        panic!();
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/17.txt").unwrap();

    let mut grid: HashMap<(i16, i16), i32> = HashMap::new();
    for (i, row) in contents.lines().enumerate() {
        for (j, char) in row.chars().enumerate() {
            grid.insert((i as i16, j as i16), char.to_digit(10).unwrap() as i32);
        }
    }

    let mut town = Town::new(grid);
    let (heat_loss, path) = town.search(false);
    println!("{}", heat_loss);

    for (i, row) in contents.lines().enumerate() {
        for (j, char) in row.chars().enumerate() {
            if path.prev.contains(&(i as i16, j as i16)) {
                print!("*");
            } else {
                print!("{}", char);
            }
        }
        println!();
    }

    town.reset();

    let (heat_loss, path) = town.search(true);
    println!("{}", heat_loss);

    for (i, row) in contents.lines().enumerate() {
        for (j, char) in row.chars().enumerate() {
            if path.prev.contains(&(i as i16, j as i16)) {
                print!("*");
            } else {
                print!("{}", char);
            }
        }
        println!();
    }
}

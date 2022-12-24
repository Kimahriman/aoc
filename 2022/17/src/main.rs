use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

enum Dir {
    Left,
    Right
}

#[derive(Clone, Debug)]
struct Rock {
    parts: Vec<(i32, i32)>
}

impl Rock {
    fn abs_positions(&self, (px, py): (i32, i32)) -> Vec<(i32, i32)> {
        self.parts.iter().map(|(x, y)| (x + px, y + py)).collect()
    }
}

#[derive(Clone, Debug)]
struct PlacedRock {
    position: (i32, i32),
    rock: Rock
}

impl PlacedRock {
    fn abs_positions(&self) -> Vec<(i32, i32)> {
        self.rock.abs_positions(self.position)
    }
}

#[derive(Clone, Debug)]
struct Grid {
    placed_rocks: Vec<PlacedRock>,
    filled_space: HashSet<(i32, i32)>,
    current_rock: Option<PlacedRock>
}

impl Grid {
    fn new() -> Self {
        Grid { placed_rocks: Vec::new(), filled_space: HashSet::new(), current_rock: None }
    }

    fn draw(&self) {
        let cur_rock_pos: HashSet<(i32, i32)> = self.current_rock.as_ref().map_or_else(|| Vec::new(), |x| x.abs_positions()).iter().map(|x| x.clone()).collect();

        let mut y = self.height() + 5;
        while y >= 0 {
            print!("|");
            for x in 0..7 {
                if cur_rock_pos.contains(&(x, y)) {
                    print!("@");
                }
                else if self.filled_space.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("|");
            println!();
            y -= 1;
        }
        println!("+-------+");
    }

    fn get_pattern(&self, rows: i32) -> Option<Vec<(i32, i32)>> {
        let mut pattern: Vec<(i32, i32)> = Vec::new();

        if self.height() + 1 < rows {
            return None;
        }

        for y in (self.height() - rows + 1)..=self.height() {
            for x in 0..7 {
                if self.filled_space.contains(&(x, y)) {
                    pattern.push((x, y + rows - self.height() - 1));
                }
            }
        }

        Some(pattern)
    }

    fn height(&self) -> i32 {
        let mut max_y = -1;
        for (_, y) in self.filled_space.iter() {
            if *y > max_y {
                max_y = *y;
            }
        }
        max_y
    }

    fn insert_rock(&mut self, rock: Rock) {
        self.current_rock = Some(PlacedRock { position: (2, self.height() + 4), rock });
    }

    fn finalize_rock(&mut self) {
        for p in self.current_rock.as_ref().unwrap().abs_positions() {
            self.filled_space.insert(p);
        }
        self.placed_rocks.push(self.current_rock.take().unwrap());
    }

    fn intersects(&self, rock: &Rock, position: (i32, i32)) -> bool {
        for (x, y) in rock.abs_positions(position) {
            if x < 0 || x >= 7 || y < 0 {
                return true;
            }
            if self.filled_space.contains(&(x, y)) {
                return true;
            }
        }
        return false;
    }

    fn shift_rock(&mut self, dir: &Dir) {
        let mut new_pos = self.current_rock.as_ref().unwrap().position;
        match dir {
            Dir::Left => {
                new_pos.0 -= 1;
            }
            Dir::Right => {
                new_pos.0 += 1;
            }
        }
        if !self.intersects(&self.current_rock.as_ref().unwrap().rock, new_pos) {
            self.current_rock.as_mut().unwrap().position = new_pos;
        }
    }

    /**
     * Returns whether the rock has finished moving
     */
    fn drop_rock(&mut self) -> bool {
        let mut new_pos = self.current_rock.as_ref().unwrap().position;
        new_pos.1 -= 1;
        if !self.intersects(&self.current_rock.as_ref().unwrap().rock, new_pos) {
            self.current_rock.as_mut().unwrap().position = new_pos;
            false
        } else {
            true
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut dir_list: Vec<Dir> = Vec::new();
    for c in contents.chars() {
        if c == '<' {
            dir_list.push(Dir::Left);
        } else {
            dir_list.push(Dir::Right);
        }
    }

    let rock1 = Rock { parts: vec![(0, 0), (1, 0), (2, 0), (3, 0)] };
    let rock2 = Rock { parts: vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)] };
    let rock3 = Rock { parts: vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)] };
    let rock4 = Rock { parts: vec![(0, 0), (0, 1), (0, 2), (0, 3)] };
    let rock5 = Rock { parts: vec![(0, 0), (1, 0), (0, 1), (1, 1)] };
    
    let rocks = [rock1, rock2, rock3, rock4, rock5];

    let mut rock_index = 0;
    let mut dir_index = 0;

    let mut grid = Grid::new();
    grid.insert_rock(rocks[rock_index].clone());
    rock_index += 1;

    // Value is # of rocks, height
    let mut pattern_map: HashMap<Vec<(i32, i32)>, (i32, i32)> = HashMap::new();
    let mut rock_count_diff: Option<i32> = None;
    let mut rock_height_diff: Option<i32> = None;

    loop {
        grid.shift_rock(&dir_list[dir_index]);
        dir_index = (dir_index + 1) % dir_list.len();
        // grid.draw();
        if grid.drop_rock() {
            grid.finalize_rock();
            grid.insert_rock(rocks[rock_index].clone());
            rock_index = (rock_index + 1) % rocks.len();

            if grid.placed_rocks.len() == 2022 {
                println!("At 2022: {}", grid.height() + 1);
                break;
            }

            if rock_index == 1 && rock_count_diff.is_none() {
                // Starting a new iteration, save the pattern
                if let Some(pattern) = grid.get_pattern(20) {
                    if pattern_map.contains_key(&pattern) {
                        let (r, h) = pattern_map[&pattern];
                        rock_count_diff = Some(grid.placed_rocks.len() as i32 - r);
                        rock_height_diff = Some(grid.height() - h);
                    } else {
                        pattern_map.insert(pattern, (grid.placed_rocks.len() as i32, grid.height()));
                    }
                }
            }
        }

        // grid.draw();
    }

    println!("{:?} {:?}", rock_count_diff, rock_height_diff);

    let total_rocks = 1000000000000i64;
    let rocks_remaining = total_rocks - grid.placed_rocks.len() as i64;

    let height_offset = (rocks_remaining / rock_count_diff.unwrap() as i64) * rock_height_diff.unwrap() as i64;
    let mut rocks_left = rocks_remaining % rock_count_diff.unwrap() as i64;

    while rocks_left > 0 {
        grid.shift_rock(&dir_list[dir_index]);
        dir_index = (dir_index + 1) % dir_list.len();
        // grid.draw();
        if grid.drop_rock() {
            grid.finalize_rock();
            grid.insert_rock(rocks[rock_index].clone());
            rock_index = (rock_index + 1) % rocks.len();
            rocks_left -= 1;
        }
    }

    let final_height = grid.height() as i64 + height_offset + 1;
    println!("{}", final_height);

    // println!("{}", grid.height() + 1);
    // grid.draw();

    // while grid.placed_rocks.len() < 1000000000000 {
    //     grid.shift_rock(&dir_list[dir_index]);
    //     // grid.draw();
    //     if grid.drop_rock() {
    //         grid.finalize_rock();
    //         grid.insert_rock(rocks[rock_index].clone());
    //         rock_index = (rock_index + 1) % rocks.len();

    //         if grid.placed_rocks.len() % 10000 == 0 {
    //             println!("{}", grid.placed_rocks.len());
    //         }
    //     }

    //     // grid.draw();

    //     dir_index = (dir_index + 1) % dir_list.len();
    // }
}

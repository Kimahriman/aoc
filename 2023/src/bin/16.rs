use std::collections::{HashMap, HashSet};

struct Beam {
    pos: (i32, i32),
    dir: (i32, i32),
}

impl Default for Beam {
    fn default() -> Self {
        Beam {
            pos: (-1, 0),
            dir: (1, 0),
        }
    }
}

impl Beam {
    fn step(&mut self) {
        self.pos.0 += self.dir.0;
        self.pos.1 += self.dir.1;
    }

    fn turn(&mut self, c: &char) {
        match c {
            '\\' if self.dir.0 != 0 => {
                self.dir.1 = self.dir.0;
                self.dir.0 = 0;
            }
            '\\' if self.dir.1 != 0 => {
                self.dir.0 = self.dir.1;
                self.dir.1 = 0;
            }
            '/' if self.dir.0 != 0 => {
                self.dir.1 = -self.dir.0;
                self.dir.0 = 0;
            }
            '/' if self.dir.1 != 0 => {
                self.dir.0 = -self.dir.1;
                self.dir.1 = 0;
            }
            _ => (),
        }
    }

    fn should_split(&self, c: &char) -> bool {
        match c {
            '|' if self.dir.1 == 0 => true,
            '-' if self.dir.0 == 0 => true,
            _ => false,
        }
    }

    fn split(self) -> [Beam; 2] {
        if self.dir.0 != 0 {
            [
                Beam {
                    pos: self.pos,
                    dir: (0, 1),
                },
                Beam {
                    pos: self.pos,
                    dir: (0, -1),
                },
            ]
        } else {
            [
                Beam {
                    pos: self.pos,
                    dir: (1, 0),
                },
                Beam {
                    pos: self.pos,
                    dir: (-1, 0),
                },
            ]
        }
    }
}

#[derive(Default)]
struct Grid {
    grid: HashMap<(i32, i32), char>,
    energized: HashMap<(i32, i32), HashSet<(i32, i32)>>,
}

impl Grid {
    fn beam(&mut self, mut beam: Beam) {
        loop {
            beam.step();

            if let Some(grid_pos) = self.grid.get(&beam.pos) {
                let set = if let Some(dirs) = self.energized.get_mut(&beam.pos) {
                    if dirs.contains(&beam.dir) {
                        break;
                    } else {
                        dirs
                    }
                } else {
                    self.energized.insert(beam.pos, Default::default());
                    self.energized.get_mut(&beam.pos).unwrap()
                };
                set.insert(beam.dir);

                if beam.should_split(grid_pos) {
                    beam.split().into_iter().for_each(|b| self.beam(b));
                    break;
                } else {
                    beam.turn(grid_pos)
                }
            } else {
                break;
            }
        }
    }

    fn reset(&mut self) {
        self.energized = Default::default();
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/16.txt").unwrap();

    let rows = contents.lines().count();
    let cols = contents.lines().next().unwrap().len();

    let mut grid = Grid::default();

    for (row, line) in contents.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            grid.grid.insert((col as i32, row as i32), c);
        }
    }

    grid.beam(Beam::default());

    println!("{}", grid.energized.len());

    let mut max_energized = 0;
    for row in 0..rows as i32 {
        grid.reset();
        grid.beam(Beam {
            pos: (-1, row),
            dir: (1, 0),
        });
        let energized = grid.energized.len();
        max_energized = usize::max(energized, max_energized);

        grid.reset();
        grid.beam(Beam {
            pos: (cols as i32, row),
            dir: (-1, 0),
        });
        let energized = grid.energized.len();
        max_energized = usize::max(energized, max_energized);
    }

    for col in 0..cols as i32 {
        grid.reset();
        grid.beam(Beam {
            pos: (col, -1),
            dir: (0, 1),
        });
        let energized = grid.energized.len();
        max_energized = usize::max(energized, max_energized);

        grid.reset();
        grid.beam(Beam {
            pos: (col, rows as i32),
            dir: (0, -1),
        });
        let energized = grid.energized.len();
        max_energized = usize::max(energized, max_energized);
    }

    println!("{}", max_energized);
}

use std::fmt::Debug;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum State {
    Ash = 0,
    Rock = 1,
}

impl State {
    fn swap(&mut self) {
        *self = match self {
            State::Ash => State::Rock,
            State::Rock => State::Ash,
        };
    }
}

struct Pattern {
    grid: Vec<Vec<State>>,
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for s in row.iter() {
                f.write_str(match s {
                    State::Ash => ".",
                    State::Rock => "#",
                })?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Pattern {
    fn new(lines: &[&str]) -> Self {
        let grid = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => State::Ash,
                        '#' => State::Rock,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();
        Self { grid }
    }

    fn find_smudge(&mut self) -> u32 {
        let original_score = self.find_reflection(0).unwrap();
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                self.grid[row][col].swap();
                if let Some(score) = self.find_reflection(original_score) {
                    self.grid[row][col].swap();
                    return score;
                }
                self.grid[row][col].swap();
            }
        }
        panic!("No smudge found");
    }

    fn find_reflection(&self, ignore: u32) -> Option<u32> {
        let rows: Vec<u32> = self
            .grid
            .iter()
            .map(|line| {
                let mut val = 0;
                for (i, s) in line.iter().enumerate() {
                    val += (*s as u32) << i;
                }
                val
            })
            .collect();

        let columns: Vec<u32> = (0..self.grid[0].len())
            .map(|c| {
                let mut val = 0;
                for (i, line) in self.grid.iter().enumerate() {
                    val += (line[c] as u32) << i;
                }
                val
            })
            .collect();

        // Try the rows first
        for i in 1..rows.len() {
            let mut j = i - 1;
            let mut k = i;
            let mut mirror = true;
            loop {
                if rows[j] != rows[k] {
                    mirror = false;
                    break;
                }
                if j == 0 || k == rows.len() - 1 {
                    break;
                }
                j -= 1;
                k += 1;
            }
            if mirror {
                let val = i as u32 * 100;
                if val != ignore {
                    return Some(val);
                }
            }
        }

        // Next columns
        for i in 1..columns.len() {
            let mut j = i - 1;
            let mut k = i;
            let mut mirror = true;
            loop {
                if columns[j] != columns[k] {
                    mirror = false;
                    break;
                }
                if j == 0 || k == columns.len() - 1 {
                    break;
                }
                j -= 1;
                k += 1;
            }
            if mirror {
                let val = i as u32;
                if val != ignore {
                    return Some(val);
                }
            }
        }

        None
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/13.txt").unwrap();

    let mut start = 0;
    let lines: Vec<&str> = contents.lines().collect();

    let mut patterns: Vec<Pattern> = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            patterns.push(Pattern::new(&lines[start..index]));
            start = index + 1
        }
    }

    patterns.push(Pattern::new(&lines[start..lines.len()]));

    let mut sum = 0;
    for pattern in patterns.iter() {
        sum += pattern.find_reflection(0).unwrap();
    }
    println!("{}", sum);

    sum = 0;
    for pattern in patterns.iter_mut() {
        sum += pattern.find_smudge();
    }
    println!("{}", sum);
}

use std::collections::HashSet;

#[derive(Debug)]
struct Number {
    value: u32,
    row: usize,
    start: usize,
    end: usize,
}

impl Number {
    fn bounding_positions(&self) -> Vec<(usize, usize)> {
        let mut positions: Vec<(usize, usize)> = Vec::new();
        if self.start > 0 {
            if self.row > 0 {
                positions.push((self.row - 1, self.start - 1));
            }
            positions.push((self.row, self.start - 1));
            positions.push((self.row + 1, self.start - 1));
        }
        if self.row > 0 {
            for col in (self.start)..=(self.end + 1) {
                positions.push((self.row - 1, col));
            }
        }
        positions.push((self.row, self.end + 1));
        for col in (self.start)..=(self.end + 1) {
            positions.push((self.row + 1, col));
        }
        positions
    }

    fn touches(&self, row: usize, col: usize) -> bool {
        for (r, c) in self.bounding_positions().iter() {
            if row == *r && col == *c {
                return true;
            }
        }
        false
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/3.txt").unwrap();

    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: HashSet<(usize, usize)> = HashSet::new();

    for (row, line) in contents.lines().enumerate() {
        let mut current_number = String::new();
        for (col, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                current_number.push(char);
            } else {
                if !current_number.is_empty() {
                    numbers.push(Number {
                        value: current_number.parse().unwrap(),
                        row,
                        start: col - current_number.len(),
                        end: col - 1,
                    });
                    current_number = String::new();
                }

                if char != '.' {
                    symbols.insert((row, col));
                }
            }
        }
        if !current_number.is_empty() {
            numbers.push(Number {
                value: current_number.parse().unwrap(),
                row,
                start: line.len() - current_number.len(),
                end: line.len() - 1,
            });
        }
    }

    let mut sum = 0u32;
    for number in numbers.iter() {
        for (row, col) in number.bounding_positions().iter() {
            if symbols.contains(&(*row, *col)) {
                sum += number.value;
                break;
            }
        }
    }
    println!("{}", sum);

    sum = 0;
    for (row, line) in contents.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '*' {
                let mut connected: Vec<u32> = Vec::new();
                // Brute force look for any two numbers adjacent
                for number in numbers.iter() {
                    if number.touches(row, col) {
                        connected.push(number.value);
                    }
                }
                if connected.len() > 2 {
                    panic!()
                }
                if connected.len() == 2 {
                    sum += connected.into_iter().reduce(|acc, e| acc * e).unwrap();
                }
            }
        }
    }

    println!("{}", sum);
}

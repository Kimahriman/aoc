use std::{convert::Infallible, str::FromStr};

struct Sequence {
    nums: Vec<i32>,
}

impl FromStr for Sequence {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Sequence {
            nums: s.split(' ').map(|s| s.parse().unwrap()).collect(),
        })
    }
}

impl Sequence {
    fn all_zero(&self) -> bool {
        self.nums.iter().all(|n| *n == 0)
    }

    fn reduce(&self) -> Self {
        Sequence {
            nums: self
                .nums
                .iter()
                .take(self.nums.len() - 1)
                .enumerate()
                .map(|(index, val)| self.nums[index + 1] - val)
                .collect(),
        }
    }

    fn predict_next(&self) -> i32 {
        if self.all_zero() {
            0
        } else {
            self.nums.last().unwrap() + self.reduce().predict_next()
        }
    }

    fn predict_previous(&self) -> i32 {
        if self.all_zero() {
            0
        } else {
            self.nums.first().unwrap() - self.reduce().predict_previous()
        }
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/9.txt").unwrap();

    let sequences: Vec<Sequence> = contents.lines().map(|s| s.parse().unwrap()).collect();

    let mut sum = 0;
    for sequence in sequences.iter() {
        sum += sequence.predict_next();
    }
    println!("{}", sum);

    sum = 0;
    for sequence in sequences.iter() {
        sum += sequence.predict_previous();
    }
    println!("{}", sum);
}

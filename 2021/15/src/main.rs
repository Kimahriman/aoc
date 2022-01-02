use std::fs;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::cmp::Ordering;
// use ansi_term::Colour::Red;

struct Pos {
    risk: u32,
    row: usize,
    col: usize
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.risk.cmp(&other.risk)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.risk == other.risk
    }
}

impl Eq for Pos {}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let width = lines[0].len();
    let height = lines.len();
    let full_width = width * 5;
    let full_height = height * 5;
    let mut grid = vec![vec![0u8; full_width]; full_height];

    for i in 0..5 {
        for j in 0..5 {
            for r in 0..lines.len() {
                let offset_rows = i * height;
                let offset_cols = j * width;
                let bump = (i + j) as u32;
                let mut col = 0;
                for c in lines[r].chars() {
                    grid[offset_rows + r][offset_cols + col] = ((c.to_digit(10).unwrap() + bump - 1) % 9 + 1) as u8;
                    col += 1;
                }
            }
        }
    }

    for i in 0..5 {
        for r in 0..3 {
            for j in 0..5 {
                for c in 0..3 {
                    print!("{}", grid[i * height + r][j * width + c]);
                }
                print!(" ");
            }
            println!();
        }
        println!();
    }

    let mut heap = BinaryHeap::<Reverse<Pos>>::new();
    let mut visisted = vec![vec![false; full_width]; full_height];
    heap.push(Reverse(Pos{ risk: 0, col: 0, row: 0 }));

    let final_pos = loop {
        let Reverse(next_pos) = heap.pop().unwrap();
        if next_pos.row == full_height - 1 && next_pos.col == full_width - 1 {
            break next_pos;
        }

        if next_pos.row < full_height - 1 && !visisted[next_pos.row + 1][next_pos.col] {
            let new_row = next_pos.row + 1;
            let new_risk = next_pos.risk + grid[new_row][next_pos.col] as u32;
            heap.push(Reverse(Pos {risk: new_risk, row: new_row, col: next_pos.col}));
            visisted[next_pos.row + 1][next_pos.col] = true;
        }

        if next_pos.col < full_width - 1 && !visisted[next_pos.row][next_pos.col + 1] {
            let new_col = next_pos.col + 1;
            let new_risk = next_pos.risk + grid[next_pos.row][new_col] as u32;
            heap.push(Reverse(Pos {risk: new_risk, row: next_pos.row, col: new_col}));
            visisted[next_pos.row][next_pos.col + 1] = true;
        }

        if next_pos.row > 0 && !visisted[next_pos.row - 1][next_pos.col] {
            let new_row = next_pos.row - 1;
            let new_risk = next_pos.risk + grid[new_row][next_pos.col] as u32;
            heap.push(Reverse(Pos {risk: new_risk, row: new_row, col: next_pos.col}));
            visisted[next_pos.row - 1][next_pos.col] = true;
        }

        if next_pos.col > 0 && !visisted[next_pos.row][next_pos.col - 1] {
            let new_col = next_pos.col - 1;
            let new_risk = next_pos.risk + grid[next_pos.row][new_col] as u32;
            heap.push(Reverse(Pos {risk: new_risk, row: next_pos.row, col: new_col}));
            visisted[next_pos.row][next_pos.col - 1] = true;
        }
    };

    println!("Found final pos {} {} {}", final_pos.row, final_pos.col, final_pos.risk);
}
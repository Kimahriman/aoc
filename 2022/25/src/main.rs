use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn convert_from_snafu(s: &str) -> i64 {
    let mut result = 0i64;
    for (digit, val) in s.chars().rev().enumerate() {
        let mult: i64 = match val {
            '-' => -1,
            '=' => -2,
            _ => val.to_string().parse().unwrap()
        };
        // println!("{} {}", 5i64.pow(digit as u32), mult);
        result += 5i64.pow(digit as u32) * mult
    }
    result
}

fn convert_to_snafu(i: i64) -> String {
    // Find how many digits we need
    let mut digits = 0;
    let mut sum = 0;
    while sum < i {
        sum += 5i64.pow(digits as u32) * 2;
        digits += 1;
    }

    let mut buf = String::new();

    // Start with each digit being 2 and decrease it until just before the number would be too low
    while digits > 0 {
        let mut cur_digit = 2;
        while cur_digit > -2 {
            if sum - 5i64.pow(digits - 1) >= i {
                sum -= 5i64.pow(digits - 1);
                cur_digit -= 1;
            } else {
                break;
            }
        }
        buf += match cur_digit {
            2 => "2",
            1 => "1",
            0 => "0",
            -1 => "-",
            -2 => "=",
            _ => ""
        };
        digits -= 1;
    }

    buf
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut sum = 0i64;
    for line in lines.iter() {
        println!("{}\t{}\t{}", line, convert_from_snafu(line), convert_to_snafu(convert_from_snafu(line)));
        sum += convert_from_snafu(line);
    }

    println!("{}", convert_to_snafu(sum));
}

use std::fs;

fn get_value(val: &str) -> u32 {
    match val {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        v => v.parse::<u32>().unwrap(),
    }
}

fn reverse_string(val: &str) -> String {
    val.chars().rev().collect()
}

fn main() {
    let contents = fs::read_to_string("inputs/1.txt").unwrap();

    let mut sum = 0u32;
    for line in contents.lines() {
        let mut first = None;
        let mut last = None;

        for char in line.chars() {
            if ('1'..='9').contains(&char) {
                if first.is_none() {
                    first = char.to_digit(10);
                }
                last = char.to_digit(10);
            }
        }

        sum += first.unwrap() * 10 + last.unwrap();
    }
    println!("{}", sum);

    let word_pattern = "one|two|three|four|five|six|seven|eight|nine";
    let forward_pattern = regex::Regex::new(&format!("{}|[1-9]", word_pattern)).unwrap();
    let reverse_pattern =
        regex::Regex::new(&format!("{}|[1-9]", reverse_string(word_pattern))).unwrap();

    sum = 0;
    for line in contents.lines() {
        let first = get_value(forward_pattern.find(line).unwrap().as_str());

        let last = get_value(&reverse_string(
            reverse_pattern
                .find(&reverse_string(line))
                .unwrap()
                .as_str(),
        ));

        sum += first * 10 + last;
    }
    println!("{}", sum);
}

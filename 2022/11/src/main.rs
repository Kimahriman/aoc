use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use regex::Regex;

#[derive(Clone, Debug)]
enum Operation {
    Add,
    Multiply
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    operation_num: i64,
    divisible_by: i64,
    true_target: usize,
    false_target: usize,
    inspected_items: i32
}

impl Monkey {
    fn has_items(&self) -> bool {
        return !self.items.is_empty()
    }

    fn inspect(&mut self, reduced_worry: bool) -> (i64, usize) {
        assert!(self.has_items());
        
        let mut item = self.items.remove(0);
        self.inspected_items += 1;

        let op_num = match self.operation_num {
            -1 => item,
            d => d
        };

        item = match self.operation {
            Operation::Add => item + op_num,
            Operation::Multiply => item * op_num
        };

        if reduced_worry {
            item /= 3;
        }

        let target = match item % self.divisible_by {
            0 => self.true_target,
            _ => self.false_target
        };

        return (item, target);
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut monkeys: Vec<Monkey> = Vec::new();

    let op_re = Regex::new(r"new = old ([*+]) (old|\d+)").unwrap();
    let div_re = Regex::new(r"divisible by (\d+)").unwrap();
    let monkey_re = Regex::new(r"throw to monkey (\d+)").unwrap();

    for chunk in lines.chunks(7) {
        let starting_items: Vec<&str> = chunk[1].split_once(": ").unwrap().1.split(", ").collect();
        let items: Vec<i64> = starting_items.into_iter().map(|x| x.parse().unwrap()).collect();

        println!("{}", chunk[2]);
        let op_match = op_re.captures(chunk[2]).unwrap();
        let op = match op_match.get(1).unwrap().as_str() {
            "+" => {
                Operation::Add
            }
            _ => {
                Operation::Multiply
            }
        };
        let op_num: i64 = match op_match.get(2).unwrap().as_str() {
            "old" => -1,
            s => s.parse().unwrap()
        };

        let div_match = div_re.captures(chunk[3]).unwrap();
        let div_by: i64 = div_match.get(1).unwrap().as_str().parse().unwrap();

        let true_monkey: usize = monkey_re.captures(chunk[4]).unwrap().get(1).unwrap().as_str().parse().unwrap();
        let false_monkey: usize = monkey_re.captures(chunk[5]).unwrap().get(1).unwrap().as_str().parse().unwrap();

        let monkey = Monkey {
            items,
            operation: op,
            operation_num: op_num,
            divisible_by: div_by,
            true_target: true_monkey,
            false_target: false_monkey,
            inspected_items: 0
        };

        monkeys.push(monkey);
    }

    let mut monkey_clones = monkeys.clone();

    let rounds = 20;
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while monkeys[i].has_items() {
                let (score, target) = monkeys[i].inspect(true);
                monkeys[target].items.push(score);
            }
        }
    }

    let mut inspected_items: Vec<i32> = monkeys.iter().map(|x| x.inspected_items).collect();
    inspected_items.sort_by(|a, b| b.cmp(a));
    println!("{}", inspected_items[0] * inspected_items[1]);

    let mut total_mod = 1;
    for m in monkey_clones.iter() {
        total_mod *= m.divisible_by;
    }

    for _ in 0..10000 {
        for i in 0..monkey_clones.len() {
            while monkey_clones[i].has_items() {
                let (score, target) = monkey_clones[i].inspect(false);
                monkey_clones[target].items.push(score % total_mod);
            }
        }
    }

    let mut inspected_items: Vec<i32> = monkey_clones.iter().map(|x| x.inspected_items).collect();
    inspected_items.sort_by(|a, b| b.cmp(a));
    println!("{}", (inspected_items[0] as i64) * (inspected_items[1] as i64));
}

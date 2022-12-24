use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use regex::Regex;

enum Op {
    Add,
    Sub,
    Mult,
    Divide
}

struct Monkey {
    id: String,
    value: Option<i64>,
    left: Option<String>,
    right: Option<String>,
    op: Option<Op>
}

impl Monkey {
    fn new_static(id: String, value: i64) -> Self {
        Self {
            id,
            value: Some(value),
            left: None,
            right: None,
            op: None
        }
    }

    fn new_op(id: String, left: String, right: String, op: Op) -> Self {
        Self {
            id,
            value: None,
            left: Some(left),
            right: Some(right),
            op: Some(op)
        }
    }

    fn contains(&self, id: &str, monkey_map: &HashMap<String, Monkey>) -> bool {
        self.id == id ||
            self.left.as_ref().filter(|x| monkey_map[*x].contains(id, monkey_map)).is_some() ||
            self.right.as_ref().filter(|x| monkey_map[*x].contains(id, monkey_map)).is_some()
    }

    fn eval(&self, monkey_map: &HashMap<String, Monkey>) -> i64 {
        if let Some(v) = self.value {
            v
        } else {
            let left_val = monkey_map[self.left.as_ref().unwrap()].eval(monkey_map);
            let right_val = monkey_map[self.right.as_ref().unwrap()].eval(monkey_map);
            match self.op.as_ref().unwrap() {
                Op::Add => {
                    left_val + right_val
                }
                Op::Sub => {
                    left_val - right_val
                }
                Op::Mult => {
                    left_val * right_val
                }
                Op::Divide => {
                    left_val / right_val
                }
            }
        }
    }

    fn uneval(&self, target: &str, value: i64, monkey_map: &HashMap<String, Monkey>) -> i64 {
        if self.id == target {
            return value;
        }

        assert!(self.value.is_none(), "Tried to uneval a static value");

        // We know left <op> right = value
        if monkey_map[self.left.as_ref().unwrap()].contains(target, monkey_map) {
            let right_val = monkey_map[self.right.as_ref().unwrap()].eval(monkey_map);
            // left = value <op inv> right
            let left_val = match self.op.as_ref().unwrap() {
                Op::Add => value - right_val,
                Op::Sub => value + right_val,
                Op::Mult => value / right_val,
                Op::Divide => value * right_val
            };
            monkey_map[self.left.as_ref().unwrap()].uneval(target, left_val, monkey_map)
        } else {
            let left_val = monkey_map[self.left.as_ref().unwrap()].eval(monkey_map);
            let right_val = match self.op.as_ref().unwrap() {
                Op::Add => value - left_val,
                Op::Sub => left_val - value,
                Op::Mult => value / left_val,
                Op::Divide => left_val / value
            };
            monkey_map[self.right.as_ref().unwrap()].uneval(target, right_val, monkey_map)
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    let op_re = Regex::new(r"(\w+): (\w+) (.) (\w+)").unwrap();
    let st_re = Regex::new(r"(\w+): (-?\d+)").unwrap();

    let mut monkey_map: HashMap<String, Monkey> = HashMap::new();

    for line in lines.iter() {
        if let Some(cap) = op_re.captures(line) {
            let id = cap.get(1).unwrap().as_str();
            let left = cap.get(2).unwrap().as_str();
            let op = match cap.get(3).unwrap().as_str() {
                "+" => Op::Add,
                "-" => Op::Sub,
                "*" => Op::Mult,
                _ => Op::Divide
            };
            let right = cap.get(4).unwrap().as_str();
            monkey_map.insert(id.to_string(), Monkey::new_op(id.to_string(), left.to_string(), right.to_string(), op));
        } else if let Some(cap) = st_re.captures(line) {
            let id = cap.get(1).unwrap().as_str();
            let value = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
            monkey_map.insert(id.to_string(), Monkey::new_static(id.to_string(), value));
        }
    }

    println!("{}", monkey_map["root"].eval(&monkey_map));

    let root_left = &monkey_map[monkey_map["root"].left.as_ref().unwrap()];
    let root_right = &monkey_map[monkey_map["root"].right.as_ref().unwrap()];

    let humn = if root_left.contains("humn", &monkey_map) {
        let right_val = root_right.eval(&monkey_map);
        root_left.uneval("humn", right_val, &monkey_map)
    } else {
        let left_val = root_left.eval(&monkey_map);
        root_right.uneval("humn", left_val, &monkey_map)
    };

    println!("{}", humn);
}

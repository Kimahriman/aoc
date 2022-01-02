use std::fs;
use std::mem;
use std::str::Chars;

#[derive(Debug)]
struct TreeNode {
    value: Option<u32>,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new() -> Self {
        TreeNode { value: None, left: None, right: None }
    }

    fn print(&self) {
        if self.value.is_some() {
            print!("{}", self.value.unwrap());
        } else if self.left.is_some() && self.right.is_some() {
            print!("[");
            self.left.as_ref().unwrap().print();
            print!(",");
            self.right.as_ref().unwrap().print();
            print!("]");
        }
    }

    fn add(&mut self, other: Box<TreeNode>) {
        let old_left = mem::replace(&mut self.left, None);
        let old_right = mem::replace(&mut self.right, None);
        let old_value = mem::replace(&mut self.value, None);
        let new_left = Box::new(TreeNode { value: old_value, left: old_left, right: old_right });
        self.left = Some(new_left);
        self.right = Some(other);
    }

    fn explode(&mut self, height: u32) -> Option<(Option<u32>, Option<u32>)> {
        // println!("Checking at height {}", height);
        if self.left.is_none() || self.right.is_none() {
            return None
        } else if height >= 4 {
            let left_val = self.left.as_ref().expect("left val at height 5").value.unwrap();
            let right_val = self.right.as_ref().expect("right val at height 5").value.unwrap();
            self.left = None;
            self.right = None;
            self.value = Some(0);
            return Some((Some(left_val), Some(right_val)))
        } else {
            if self.left.is_some() {
                let exploded_left = self.left.as_mut().unwrap().explode(height + 1);
                if let Some((left_val, right_val)) = exploded_left {
                    if let Some(rv) = right_val {
                        self.right.as_mut().unwrap().explode_left(rv);
                        return Some((left_val, None))
                    } else {
                        return exploded_left
                    }
                }
            }
            if self.right.is_some() {
                let exploded_right = self.right.as_mut().unwrap().explode(height + 1);
                if let Some((left_val, right_val)) = exploded_right {
                    if let Some(lv) = left_val {
                        self.left.as_mut().unwrap().explode_right(lv);
                        return Some((None, right_val))
                    } else {
                        return exploded_right
                    }
                }
            }
        }
        return None
    }

    fn explode_left(&mut self, val: u32) {
        if self.value.is_some() {
            self.value = Some(self.value.unwrap() + val);
        } else {
            self.left.as_mut().unwrap().explode_left(val);
        }
    }

    fn explode_right(&mut self, val: u32) {
        if self.value.is_some() {
            self.value = Some(self.value.unwrap() + val);
        } else {
            self.right.as_mut().unwrap().explode_right(val);
        }
    }

    fn split(&mut self) -> bool {
        if self.value.is_some() && self.value.unwrap() > 9 {
            let val = self.value.take().unwrap();
            self.left = Some(Box::new(TreeNode {
                value: Some(val / 2),
                left: None,
                right: None
            }));
            self.right = Some(Box::new(TreeNode {
                value: Some((val + 1) / 2),
                left: None,
                right: None
            }));
            return true
        } else {
            if self.left.is_some() {
                if self.left.as_mut().unwrap().split() {
                    return true
                }
            }
            if self.right.is_some() {
                if self.right.as_mut().unwrap().split() {
                    return true
                }
            }
        }
        return false
    }

    fn magnitude(&self) -> u32 {
        if self.value.is_some() {
            self.value.unwrap()
        } else {
            3 * self.left.as_ref().unwrap().magnitude() + 2 * self.right.as_ref().unwrap().magnitude()
        }
    }

    fn reduce(&mut self) {
        loop {
            if self.explode(0).is_some() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut lines_iter = lines.iter();
    let mut root = parse_pair(&mut lines_iter.next().unwrap().chars());

    for line in lines_iter {
        let next_tree = parse_pair(&mut line.chars());
        root.add(next_tree);
        root.reduce();


        // break;
        // println!("Parsed! {:?}", root);
    }
    println!("Magnitude: {}", root.magnitude());

    let mut max_mag = 0_u32;
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i == j { continue }
                let mut node = parse_pair(&mut lines[i].chars());
                node.add(parse_pair(&mut lines[j].chars()));
                node.reduce();
                let mag = node.magnitude();
                if mag > max_mag { max_mag = mag; }
        }
    }
    println!("Max magnitude: {}", max_mag);
}

fn parse_pair(line: &mut Chars) -> Box<TreeNode> {
    let mut node = Box::new(TreeNode::new());

    let mut next_char = line.next().unwrap();
    if next_char >= '0' && next_char <= '9' {
        node.value = Some(next_char.to_digit(10).unwrap());
        return node
    }

    assert_eq!(next_char, '[');
    node.left = Some(parse_pair(line));

    next_char = line.next().unwrap();
    assert_eq!(next_char, ',');

    node.right = Some(parse_pair(line));

    next_char = line.next().unwrap();
    assert_eq!(next_char, ']');

    node
}

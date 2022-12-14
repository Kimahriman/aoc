use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::str::Chars;
use std::string::ToString;
use std::fmt::Debug;


#[derive(Clone, Debug)]
struct Tree<T: Ord + ToString + Clone + Debug> {
    data: Option<T>,
    children: Vec<Tree<T>>
}

impl<T: Ord + ToString + Clone + Debug> Tree<T> {
    fn new() -> Tree<T> {
        Tree { data: None, children: Vec::new() }
    }

    fn from(data: T) -> Tree<T> {
        Tree { data: Some(data), children: Vec::new() }
    }

    fn format(&self) -> String {
        let mut s = String::new();

        if self.data.is_some() {
            s += &self.data.as_ref().unwrap().to_string();
        } else {
            s += "[";
            for child in self.children.iter() {
                s += &child.format();
                s += ",";
            }
            s += "]";
        }
        s
    }

    fn as_tree(&self) -> Tree<T> {
        assert!(self.data.is_some());

        let mut tree: Tree<T> = Tree::new();
        tree.children.push(Tree::from(self.data.as_ref().unwrap().clone()));
        tree
    }

    fn cmp(&self, other: &Self) -> Ordering {
        if self.data.is_some() && other.data.is_some() {
            return self.data.as_ref().unwrap().cmp(&other.data.as_ref().unwrap());
        } else if self.data.is_some() {
            return self.as_tree().cmp(&other);
        } else if other.data.is_some() {
            return self.cmp(&other.as_tree());
        } else {
            for (i, v) in self.children.iter().enumerate() {
                if i >= other.children.len() {
                    // More on the left, this is greater
                    return Ordering::Greater;
                }
                // println!("Comparing {:?} {:?}", v, other.children[i]);
                let cmp = v.cmp(&other.children[i]);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            // Haven't returned yet
            if self.children.len() < other.children.len() {
                // Things still left in other, so we are less
                return Ordering::Less;
            } else {
                // These are equal
                return Ordering::Equal;
            }
        }
    }
}

fn parse_line(msg: &mut Chars) -> Tree<i32> {
    let mut tree: Tree<i32> = Tree::new();
    let mut num_str: String = String::new();

    while let Some(c) = msg.next() {
        match c {
            '[' => {
                tree.children.push(parse_line(msg));
            }
            ']' => {
                if num_str.len() > 0 {
                    tree.children.push(Tree::from(num_str.parse().unwrap()));
                }
                return tree;
            }
            ',' => {
                if num_str.len() > 0 {
                    tree.children.push(Tree::from(num_str.parse().unwrap()));
                    num_str = String::new();
                }
                continue;
            }
            n => {
                num_str += &n.to_string();
            }
        }
    }
    
    tree
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut pairs: Vec<(Tree<i32>, Tree<i32>)> = Vec::new();

    for chunk in lines.chunks(3) {
        let msg1 = parse_line(&mut chunk[0].chars());
        let msg2 = parse_line(&mut chunk[1].chars());
        pairs.push((msg1, msg2));
    }

    let mut correct = 0;
    for (i, pair) in pairs.iter().enumerate() {
        println!("{}", pair.0.format());
        println!("{}", pair.1.format());
        println!("{:?}", pair.0.cmp(&pair.1));
        if pair.0.cmp(&pair.1) == Ordering::Less {
            correct += i + 1;
        }
        println!();
    }
    println!("{}", correct);

    let mut signal_list: Vec<Tree<i32>> = Vec::new();
    for pair in pairs.into_iter() {
        signal_list.push(pair.0);
        signal_list.push(pair.1);
    }

    let decoder_1: Tree<i32> = Tree::from(2).as_tree();
    let decoder_2: Tree<i32> = Tree::from(6).as_tree();
    signal_list.push(decoder_1.clone());
    signal_list.push(decoder_2.clone());

    signal_list.sort_by(|a, b| a.cmp(b));

    for signal in signal_list.iter() {
        println!("{}", signal.format());
    }

    let mut i1 = 0;
    let mut i2 = 0;
    for (i, signal) in signal_list.iter().enumerate() {
        if signal.cmp(&decoder_1) == Ordering::Equal {
            i1 = i + 1;
        }
        if signal.cmp(&decoder_2) == Ordering::Equal {
            i2 = i + 1;
        }
    }
    println!("{}", i1 * i2);
}

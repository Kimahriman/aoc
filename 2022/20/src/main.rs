use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::{cell::RefCell, rc::Rc};

struct ListNode<T> {
    item: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> ListNode<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            next: None,
            prev: None,
        }
    }

    fn shift_left(&mut self) {
        let prev = self.prev.take().unwrap();
        let next = self.next.take().unwrap();

        let prev_prev = {
            prev.borrow_mut().prev.take().unwrap()
        };
        let prev_prev_next = {
            prev_prev.borrow_mut().next.take().unwrap()
        };

        let next_prev = {
            next.borrow_mut().prev.take().unwrap()
        };
        let prev_next = {
            prev.borrow_mut().next.take().unwrap()
        };

        prev_prev.borrow_mut().next = Some(prev_next);
        prev.borrow_mut().next = Some(Rc::clone(&next));
        prev.borrow_mut().prev = Some(next_prev);
        next.borrow_mut().prev = Some(Rc::clone(&prev));
        self.next = Some(prev_prev_next);
        self.prev = Some(prev_prev);
    }

    fn shift_right(&mut self) {
        let prev = self.prev.take().unwrap();
        let next = self.next.take().unwrap();

        let next_next = {
            next.borrow_mut().next.take().unwrap()
        };
        let next_next_prev = {
            next_next.borrow_mut().prev.take().unwrap()
        };

        let prev_next = {
            prev.borrow_mut().next.take().unwrap()
        };
        let next_prev = {
            next.borrow_mut().prev.take().unwrap()
        };

        next_next.borrow_mut().prev = Some(next_prev);
        next.borrow_mut().prev = Some(Rc::clone(&prev));
        next.borrow_mut().next = Some(prev_next);
        prev.borrow_mut().next = Some(Rc::clone(&next));
        self.prev = Some(next_next_prev);
        self.next = Some(next_next);
    }
}

type Link<T> = Option<Rc<RefCell<ListNode<T>>>>;

struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>
}

impl<T: std::fmt::Display> DoublyLinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None
        }
    }

    fn push_back(&mut self, val: T) -> Rc<RefCell<ListNode<T>>> {
        let node = Rc::new(RefCell::new(ListNode::new(val)));
        if let Some(prev_tail) = self.tail.take() {
            prev_tail.borrow_mut().next = Some(Rc::clone(&node));
            node.borrow_mut().prev = Some(prev_tail);
            self.tail = Some(Rc::clone(&node));
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(Rc::clone(&node));
        }
        node
    }

    fn connect(&mut self) {
        self.tail.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(self.head.as_ref().unwrap()));
        self.head.as_ref().unwrap().borrow_mut().prev = Some(Rc::clone(self.tail.as_ref().unwrap()));
    }

    fn print(&self, len: usize) {
        let mut node_ref = self.head.as_ref().unwrap().as_ptr();
        unsafe {
            for _ in 0..len {
                println!("{}", (*node_ref).item);
                node_ref = (*node_ref).next.as_ref().unwrap().as_ptr();
            }
        }
    }
}

fn compute(mult: Option<i64>) {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut linked_list = DoublyLinkedList::<i64>::new();
    let mut node_list = Vec::<Rc<RefCell<ListNode<i64>>>>::new();

    for line in lines.iter() {
        node_list.push(linked_list.push_back(line.parse::<i64>().unwrap() * mult.unwrap_or(1)));
    }

    let num_items = node_list.len();

    linked_list.connect();

    let loop_count = if mult.is_some() {
        10
    } else {
        1
    };

    for _ in 0..loop_count {
        for node in node_list.iter() {
            let mut node_mut = node.borrow_mut();
            let shift_num = node_mut.item.abs() % (num_items - 1) as i64;
            // println!("{} {}", node_mut.item, shift_num);
            if node_mut.item < 0 {
                for _ in 0..shift_num {
                    node_mut.shift_left();
                }
            } else if node_mut.item > 0 {
                for _ in 0..shift_num {
                    node_mut.shift_right();
                }
            }

            // linked_list.print(node_list.len());
            // println!();
        }
    }

    // linked_list.print(node_list.len());
    // println!();

    let mut node_ref = linked_list.head.as_ref().unwrap().as_ptr();
    let mut sum = 0;
    unsafe {
        while (*node_ref).item != 0 {
            node_ref = (*node_ref).next.as_ref().unwrap().as_ptr();
        }

        for _ in 0..1000 {
            node_ref = (*node_ref).next.as_ref().unwrap().as_ptr();
        }
        println!("{}", (*node_ref).item);
        sum += (*node_ref).item;

        for _ in 0..1000 {
            node_ref = (*node_ref).next.as_ref().unwrap().as_ptr();
        }
        println!("{}", (*node_ref).item);
        sum += (*node_ref).item;

        for _ in 0..1000 {
            node_ref = (*node_ref).next.as_ref().unwrap().as_ptr();
        }
        println!("{}", (*node_ref).item);
        sum += (*node_ref).item;
    }

    println!("{}", sum);
}

fn main() {
    compute(None);
    compute(Some(811589153));
}
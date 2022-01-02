use std::cmp;
use std::fs;
use regex::Regex;

struct ALU {
    registers: [i32; 4]
}

impl ALU {
    fn new() -> Self {
        ALU { registers: [0; 4] }
    }

    fn eval(&mut self, instructions: &Vec<Instruction>, input: &str) {
        let mut chars = input.chars();

        for inst in instructions.iter() {
            if inst.op == "inp" {
                self.registers[inst.dest] = chars.next().unwrap().to_digit(10).unwrap() as i32;
            } else {
                // println!("{:?}", inst);
                let src_value = inst.src_lit.unwrap_or_else(|| self.registers[inst.src_reg.unwrap()]);
                let dst_value = self.registers[inst.dest];
                if inst.op == "add" {
                    self.registers[inst.dest] = dst_value + src_value;
                } else if inst.op == "mul" {
                    self.registers[inst.dest] = dst_value * src_value;
                } else if inst.op == "div" {
                    self.registers[inst.dest] = dst_value / src_value;
                } else if inst.op == "mod" {
                    self.registers[inst.dest] = dst_value % src_value;
                } else if inst.op == "eql" {
                    self.registers[inst.dest] = if dst_value == src_value { 1 } else { 0 };
                }
            }
        }
    }
}

#[derive(Debug)]
struct Instruction {
    op: String,
    dest: usize,
    src_lit: Option<i32>,
    src_reg: Option<usize>
}

fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut instructions = Vec::<Instruction>::new();

    for line in lines.iter() {
        let splits: Vec<&str> = line.split(" ").collect();
        let op = splits[0].to_string();
        let dest = match splits[1] {
            "w" => 0,
            "x" => 1,
            "y" => 2,
            "z" => 3,
            _ => 4
        };
        let mut src_lit: Option<i32> = None;
        let mut src_reg: Option<usize> = None;
        if op != "inp" {
            src_reg = match splits[2] {
                "w" => Some(0),
                "x" => Some(1),
                "y" => Some(2),
                "z" => Some(3),
                _ => None
            };
            if src_reg.is_none() {
                src_lit = Some(splits[2].parse().unwrap());
            }
        }
        // println!("Adding instruction {} {} {:?} {:?}", op, dest, src_lit, src_reg);

        instructions.push(Instruction { op, dest, src_lit, src_reg })
    }

    // for i in 11111111111111_u64..99999999999999_u64 {
    //     let s = i.to_string();
    //     if s.contains("0") { continue }

    //     if i % 1000000 == 111111 {
    //         println!("Testing {}", i);
    //     }
    //     let mut alu = ALU::new();
    //     alu.eval(&instructions, &s);
    //     // println!("{} {}", s, alu.registers[3]);
    //     if alu.registers[3] == 0 {
    //         println!("Found match {}", s);
    //     }
    // }
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut alu = ALU::new();
        alu.eval(&instructions, &line);
        println!("{}", alu.registers[3]);
    }
}

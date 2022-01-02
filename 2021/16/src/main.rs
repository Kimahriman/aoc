use std::fs;
use std::str::Chars;
use std::fmt::Display;
use ansi_term::Colour::Red;

struct HexParser<'a> {
    state: u32,
    current_bits: u32,
    chars: Chars<'a>
}

impl HexParser<'_> {

    fn new(chars: Chars) -> HexParser {
        HexParser { state: 0, current_bits: 0, chars: chars }
    }

    fn get_bits(&mut self, count: u32) -> Option<u32> {
        // println!("Getting bits: {}. Current bits {} Current state {}", count, self.current_bits, self.state);
        while self.current_bits < count {
            self.state <<= 4;
            let next_char = self.chars.next();
            if next_char.is_none() {
                return None
            }
            let new_bits = next_char.unwrap().to_digit(16).unwrap();
            // println!("Adding new bits to state {} {}", next_char.unwrap(), new_bits);
            self.state |= new_bits;
            self.current_bits += 4;
        }
        let ret_val = self.state >> (self.current_bits - count);
        self.current_bits = self.current_bits - count;
        self.state &= (1 << self.current_bits) - 1;
        // println!("Returning {}, New state {}", ret_val, self.state);
        Some(ret_val)
    }
}

#[derive(Debug)]
struct Packet {
    version: u32,
    ptype: u32,
    bits: u32,
    value: Option<u64>,
    sub_packets: Vec<Packet>
}

impl Packet {
    
    fn new(version: u32, ptype: u32) -> Packet {
        Packet { version: version, ptype: ptype, bits: 0, value: None, sub_packets: vec![] }
    }

    fn version_sum(&self) -> u32 {
        let mut s = self.version;
        for p in self.sub_packets.iter() {
            s += p.version_sum();
        }
        return s;
    }

    fn calculate(&self) -> u64 {
        match self.ptype {
            0 => {
                self.sub_packets.iter().map(|x| x.calculate()).sum()
            },
            1 => {
                self.sub_packets.iter().map(|x| x.calculate()).reduce(|accum, x| accum * x).unwrap()
            },
            2 => {
                self.sub_packets.iter().map(|x| x.calculate()).min().unwrap()
            },
            3 => {
                self.sub_packets.iter().map(|x| x.calculate()).max().unwrap()
            },
            4 => self.value.unwrap(),
            5 => {
                assert_eq!(self.sub_packets.len(), 2);
                if self.sub_packets[0].calculate() > self.sub_packets[1].calculate() { 1 } else { 0 }
            },
            6 => {
                assert_eq!(self.sub_packets.len(), 2);
                if self.sub_packets[0].calculate() < self.sub_packets[1].calculate() { 1 } else { 0 }
            },
            7 => {
                assert_eq!(self.sub_packets.len(), 2);
                if self.sub_packets[0].calculate() == self.sub_packets[1].calculate() { 1 } else { 0 }
            },
            _ => 0
        }
    }
}

// impl Display for Packet {
//     fn fmt(f: &mut Formatter<'_>) -> String {

//     }   
// }

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    // let lines: Vec<&str> = contents.split("\n").collect();
    let chars = contents.chars();
    
    let mut hex_parser = HexParser::new(chars);

    let ret = parse_packet(&mut hex_parser).unwrap();
    println!("{} {}", ret.version_sum(), ret.calculate());
}

fn parse_packet(hex_parser: &mut HexParser) -> Option<Packet> {
    let version = hex_parser.get_bits(3);
    if version.is_none() {
        return None;
    }
    // println!("Parsing packet with version {}", Red.bold().paint(version.unwrap().to_string()));

    let ptype = hex_parser.get_bits(3).unwrap();
    // println!("Packet type {}", Red.bold().paint(ptype.to_string()));
    let mut packet = Packet::new(version.unwrap(), ptype);
    packet.bits += 6;

    if ptype == 4 {
        let mut value = 0_u64;
        loop {
            value <<= 4;
            let first = hex_parser.get_bits(1).unwrap();
            let num_part = hex_parser.get_bits(4).unwrap() as u64;
            value |= num_part;
            // println!("Parsing literal chunk {} {}", first, num_part);
            packet.bits += 5;
            if first == 0 {
                break;
            }
        }
        packet.value = Some(value as u64);
    } else {
        let length_type = hex_parser.get_bits(1).unwrap();
        packet.bits += 1;
        if length_type == 0 {
            let total_bits = hex_parser.get_bits(15).unwrap();
            packet.bits += 15;
            let mut found_bits = 0;
            while found_bits < total_bits {
                let child_packet = parse_packet(hex_parser).unwrap();
                // println!("Got child packet {:?}", child_packet);
                found_bits += child_packet.bits;
                packet.sub_packets.push(child_packet);
            }
            assert_eq!(found_bits, total_bits);
            packet.bits += found_bits;
        } else {
            let total_packets = hex_parser.get_bits(11).unwrap();
            packet.bits += 11;
            for _ in 0..total_packets {
                let child_packet = parse_packet(hex_parser).unwrap();
                packet.bits += child_packet.bits;
                packet.sub_packets.push(child_packet);
            }
        }
    }
    Some(packet)
}

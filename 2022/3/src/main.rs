use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Debug)]
struct RuckSack {
    left: Vec<char>,
    right: Vec<char>
}

fn get_value(c: &char) -> i32 {
    let mut buf: [u8; 1] = [0; 1];
    
    'a'.encode_utf8(&mut buf);
    let a_val = i32::from(buf[0]);
    
    'A'.encode_utf8(&mut buf);
    let A_val = i32::from(buf[0]);

    if *c >= 'a' && *c <= 'z' {
        c.encode_utf8(&mut buf);
        return i32::from(buf[0]) - a_val + 1;
    } else {
        c.encode_utf8(&mut buf);
        return i32::from(buf[0]) - A_val + 27;
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut buf: [u8; 1] = [0; 1];

    let mut sacks: Vec<RuckSack> = Vec::new();

    let mut pri_sum: i32 = 0;
    for line in lines.iter() {
        let (left, right) = line.split_at(line.len() / 2);
        let sack = RuckSack { left: left.chars().collect(), right: right.chars().collect() };

        let left_set: HashSet<&char> = sack.left.iter().collect();
        let right_set: HashSet<&char> = sack.right.iter().collect();
        let common = left_set.intersection(&right_set).next().unwrap();

        pri_sum += get_value(*common);
        sacks.push(sack);
    }
    println!("{}", pri_sum);

    pri_sum = 0;
    for batch in sacks.chunks(3) {
        let mut first_sack: HashSet<&char> = HashSet::new();
        first_sack.extend(batch[0].left.iter());
        first_sack.extend(batch[0].right.iter());
        let mut second_sack: HashSet<&char> = HashSet::new();
        second_sack.extend(batch[1].left.iter());
        second_sack.extend(batch[1].right.iter());
        let mut third_sack: HashSet<&char> = HashSet::new();
        third_sack.extend(batch[2].left.iter());
        third_sack.extend(batch[2].right.iter());

        let mut types: HashSet<&char> = HashSet::new();
        types.extend(first_sack.iter());
        types.retain(|x| second_sack.contains(x));
        types.retain(|x| third_sack.contains(x));
        assert!(types.len() == 1);

        pri_sum += get_value(types.iter().next().unwrap());
    }
    println!("{}", pri_sum);
}

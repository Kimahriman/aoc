trait CustomHash {
    fn custom_hash(&self) -> u8;
}

impl CustomHash for &str {
    fn custom_hash(&self) -> u8 {
        let mut cur = 0u32;
        for c in self.chars() {
            cur += c as u32;
            cur *= 17;
            cur %= 256;
        }
        cur as u8
    }
}

struct Lens {
    label: String,
    length: u64,
}

impl Lens {
    fn update(&mut self, length: u64) {
        self.length = length;
    }
}

#[derive(Default)]
struct Box {
    lenses: Vec<Lens>,
}

impl Box {
    fn insert(&mut self, label: impl Into<String>, length: u64) {
        let label = label.into();
        for lens in self.lenses.iter_mut() {
            if lens.label == label {
                lens.update(length);
                return;
            }
        }
        self.lenses.push(Lens { label, length });
    }

    fn remove(&mut self, label: impl Into<String>) {
        let label = label.into();
        if let Some(index) = self
            .lenses
            .iter()
            .enumerate()
            .find(|(_, l)| l.label == label)
            .map(|(i, _)| i)
        {
            self.lenses.remove(index);
        }
    }

    fn power(&self) -> u64 {
        self.lenses
            .iter()
            .enumerate()
            .fold(0, |acc, (i, lens)| acc + (i as u64 + 1) * lens.length)
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/15.txt").unwrap();

    let mut sum: u32 = 0;
    for code in contents.split(',') {
        sum += code.custom_hash() as u32;
    }
    println!("{}", sum);

    let mut boxes: Vec<Box> = (0..256).map(|_| Box::default()).collect();

    for code in contents.split(',') {
        if code.contains('=') {
            let mut split = code.split('=');
            let label = split.next().unwrap();
            let length: u64 = split.next().unwrap().parse().unwrap();

            let hash = label.custom_hash();
            boxes[hash as usize].insert(label, length);
        } else {
            let mut split = code.split('-');
            let label = split.next().unwrap();

            let hash = label.custom_hash();
            boxes[hash as usize].remove(label);
        }
    }

    let mut total_power = 0;
    for (i, b) in boxes.iter().enumerate() {
        total_power += (i as u64 + 1) * b.power();
    }
    println!("{}", total_power);
}

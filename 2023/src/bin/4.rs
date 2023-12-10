#[derive(Debug)]
struct Card {
    winning: Vec<u32>,
    numbers: Vec<u32>,
    count: u32,
}

impl Card {
    fn winning_numbers(&self) -> u32 {
        let mut count = 0;
        for number in self.numbers.iter() {
            if self.winning.contains(number) {
                count += 1;
            }
        }
        count
    }

    fn points(&self) -> u32 {
        let mut total = 0;

        for _ in 0..self.winning_numbers() {
            if total == 0 {
                total = 1;
            } else {
                total *= 2;
            }
        }

        total
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let card_pattern =
            regex::Regex::new(r"^Card\s+(\d+):\s+([\d\s]+)\s+\|\s+([\d\s]+)$").unwrap();

        let captures = card_pattern.captures(value).unwrap();

        let winning_str: Vec<&str> = regex::Regex::new(r"\s+")
            .unwrap()
            .split(&captures[2])
            .collect();

        let numbers_str: Vec<&str> = regex::Regex::new(r"\s+")
            .unwrap()
            .split(&captures[3])
            .collect();

        Self {
            winning: winning_str
                .into_iter()
                .map(|n| n.parse().unwrap())
                .collect(),
            numbers: numbers_str
                .into_iter()
                .map(|n| n.parse().unwrap())
                .collect(),
            count: 1,
        }
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/4.txt").unwrap();

    let mut cards: Vec<Card> = Vec::new();
    for line in contents.lines() {
        cards.push(Card::from(line));
    }

    let mut sum = 0;
    for card in cards.iter() {
        sum += card.points();
    }

    println!("{}", sum);

    for i in 0..cards.len() {
        let winning = cards[i].winning_numbers();
        for j in 1..=winning {
            cards[i + j as usize].count += cards[i].count;
        }
    }

    sum = 0;
    for card in cards.iter() {
        sum += card.count;
    }

    println!("{}", sum);
}

use std::{collections::HashMap, convert::Infallible, str::FromStr};

static CARDS: &str = "23456789TJQKA";
static CARDS_WITH_JOKER: &str = "J23456789TQKA";

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum Type {
    High = 1,
    OnePair = 2,
    TwoPair = 3,
    Three = 4,
    Full = 5,
    Four = 6,
    Five = 7,
}

struct Hand {
    cards: Vec<char>,
    bid: u64,
}

impl Hand {
    fn get_type(&self, jokers_wild: bool) -> Type {
        let mut map: HashMap<char, u8> = HashMap::new();
        self.cards.iter().for_each(|c| {
            map.insert(*c, map.get(c).unwrap_or(&0) + 1);
        });

        let joker_count = if jokers_wild {
            map.remove(&'J').unwrap_or(0)
        } else {
            0
        };

        if map.len() == 1 || (jokers_wild && map.is_empty()) {
            Type::Five
        } else if map
            .values()
            .any(|c| *c == 4 || (jokers_wild && *c + joker_count == 4))
        {
            Type::Four
        } else if map.len() == 2 {
            Type::Full
        } else if map
            .values()
            .any(|c| *c == 3 || (jokers_wild && *c + joker_count == 3))
        {
            Type::Three
        } else if (map.values().filter(|c| **c == 2).count() == 2)
            || (jokers_wild
                && (joker_count > 2 || joker_count > 1 && map.values().any(|c| *c == 2)))
        {
            Type::TwoPair
        } else if map
            .values()
            .any(|c| *c == 2 || (jokers_wild && joker_count > 0))
        {
            Type::OnePair
        } else {
            Type::High
        }
    }
}

impl FromStr for Hand {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');

        Ok(Self {
            cards: split.next().unwrap().chars().collect(),
            bid: split.next().unwrap().parse().unwrap(),
        })
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/7.txt").unwrap();

    let mut hands: Vec<Hand> = contents.lines().map(|l| l.parse().unwrap()).collect();

    hands.sort_by(|a, b| match a.get_type(false).cmp(&b.get_type(false)) {
        core::cmp::Ordering::Equal => {
            for (this, o) in a.cards.iter().zip(b.cards.iter()) {
                match CARDS.find(*this).unwrap().cmp(&CARDS.find(*o).unwrap()) {
                    core::cmp::Ordering::Equal => {}
                    ord => return ord,
                }
            }
            core::cmp::Ordering::Equal
        }
        ord => ord,
    });

    let mut sum = 0;
    for (index, hand) in hands.iter().enumerate() {
        sum += (index as u64 + 1) * hand.bid;
    }
    println!("{}", sum);

    hands.sort_by(|a, b| match a.get_type(true).cmp(&b.get_type(true)) {
        core::cmp::Ordering::Equal => {
            for (this, o) in a.cards.iter().zip(b.cards.iter()) {
                match CARDS_WITH_JOKER
                    .find(*this)
                    .unwrap()
                    .cmp(&CARDS_WITH_JOKER.find(*o).unwrap())
                {
                    core::cmp::Ordering::Equal => {}
                    ord => return ord,
                }
            }
            core::cmp::Ordering::Equal
        }
        ord => ord,
    });

    let mut sum = 0;
    for (index, hand) in hands.iter().enumerate() {
        sum += (index as u64 + 1) * hand.bid;
    }
    println!("{}", sum);
}

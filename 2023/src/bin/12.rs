use std::{collections::HashMap, convert::Infallible, str::FromStr, sync::Mutex};

use once_cell::sync::Lazy;

static CACHE: Lazy<Mutex<HashMap<Spring, u64>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Eq, PartialEq, Hash)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!(),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Spring {
    states: Vec<State>,
    groups: Vec<u32>,
    current: Option<u32>,
}

impl FromStr for Spring {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let states = split.next().unwrap().chars().map(Into::into).collect();
        let groups = split
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        Ok(Self {
            states,
            groups,
            current: None,
        })
    }
}

impl Spring {
    fn unfold(&mut self) {
        let initial_state = self.states.clone();
        for _ in 0..4 {
            self.states.push(State::Unknown);
            self.states.extend(initial_state.clone());
        }

        let initial_groups = self.groups.clone();
        for _ in 0..4 {
            self.groups.extend(initial_groups.clone());
        }
    }

    fn count_valid(&self) -> u64 {
        if let Some(count) = CACHE.lock().unwrap().get(self) {
            return *count;
        }

        let valid = if let Some(next_state) = self.states.first() {
            if let Some(current) = self.current {
                if current == 0 {
                    match *next_state {
                        State::Damaged => 0,
                        // Either way treat as operational
                        _ => Spring {
                            states: self.states[1..].to_vec(),
                            groups: self.groups.clone(),
                            current: None,
                        }
                        .count_valid(),
                    }
                } else {
                    match *next_state {
                        State::Operational => 0,
                        _ => Spring {
                            states: self.states[1..].to_vec(),
                            groups: self.groups.clone(),
                            current: Some(current - 1),
                        }
                        .count_valid(),
                    }
                }
            } else {
                let operational_count = Spring {
                    states: self.states[1..].to_vec(),
                    groups: self.groups.clone(),
                    current: None,
                }
                .count_valid();
                let damaged_count = if let Some(next_group) = self.groups.first() {
                    Spring {
                        states: self.states[1..].to_vec(),
                        groups: self.groups[1..].to_vec(),
                        current: Some(*next_group - 1),
                    }
                    .count_valid()
                } else {
                    0
                };
                match *next_state {
                    State::Operational => operational_count,
                    State::Damaged => damaged_count,
                    State::Unknown => operational_count + damaged_count,
                }
            }
        } else if self.current.filter(|c| *c > 0).is_some() || !self.groups.is_empty() {
            // End of the states but still expected more damaged
            0
        } else {
            // Good end of state
            1
        };

        CACHE.lock().unwrap().insert(self.clone(), valid);

        valid
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/12.txt").unwrap();

    let mut springs: Vec<Spring> = contents.lines().map(|s| s.parse().unwrap()).collect();

    let mut sum = 0;
    for spring in springs.iter() {
        let spring_count = spring.count_valid();
        sum += spring_count;
    }
    println!("{}", sum);

    let mut sum = 0u64;
    for spring in springs.iter_mut() {
        spring.unfold();
        let spring_count = spring.count_valid();
        println!("Spring count: {}", spring_count);
        sum += spring_count;
    }
    println!("{}", sum);
}

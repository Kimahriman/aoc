use std::fs;
use std::collections::HashMap;
// use ansi_term::Colour::Red;

struct DetermDie {
    next_val: u32,
    count: u32
}

impl DetermDie {
    fn new() -> Self {
        DetermDie { next_val: 1, count: 0 }
    }

    fn roll(&mut self) -> u32 {
        let ret = self.next_val;
        self.next_val += 1;
        if self.next_val > 100 {
            self.next_val = 1;
        }
        self.count += 1;
        ret
    }
}

fn main() {
    let mut p1 = 3;
    let mut p2 = 2;

    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut p1_turn = true;

    let mut die = DetermDie::new();
    while p1_score < 1000 && p2_score < 1000 {
        let moves = die.roll() + die.roll() + die.roll();
        if p1_turn {
            p1 += moves;
            p1 %= 10;
            p1_score += p1 + 1;
        } else {
            p2 += moves;
            p2 %= 10;
            p2_score += p2 + 1;
        }
        p1_turn = !p1_turn;
    }

    let losing_score = if p1_score >= 1000 {
        println!("Player 1 wins with {}", p1_score);
        p2_score
    } else {
        println!("Player 2 wins with {}", p2_score);
        p1_score
    };

    println!("{} * {} = {}", losing_score, die.count, losing_score * die.count);

    let mut rolls = HashMap::<u8, u64>::new();

    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                let mut val = rolls.entry(i + j + k).or_insert(0);
                *val += 1;
            }
        }
    }

    let mut cache = HashMap::<(States, Scores, bool), Wins>::new();
    let wins = play([3, 2], [0, 0], true, &rolls, &mut cache);

    println!("Player 1 wins: {}. Player 2 wins: {}", wins[0], wins[1]);
}

type States = [u8; 2];
type Scores = [u8; 2];
type Wins = [u64; 2];

fn play(states: States, scores: Scores, p1_turn: bool, rolls: &HashMap<u8, u64>, cache: &mut HashMap<(States, Scores, bool), Wins>) -> Wins {
    if let Some(wins) = cache.get(&(states, scores, p1_turn)) {
        return *wins
    }

    let mut new_wins = [0, 0];
    for (roll, count) in rolls.iter() {
        if p1_turn {
            let mut states = states;
            let mut scores = scores;
            states[0] = (states[0] + roll) % 10;
            scores[0] = scores[0] + states[0] + 1;
            if scores[0] >= 21 {
                new_wins[0] += count;
            } else {
                let rec_wins = play(states, scores, false, rolls, cache);
                new_wins[0] += rec_wins[0] * count;
                new_wins[1] += rec_wins[1] * count;
            }
        } else {
            let mut states = states;
            let mut scores = scores;
            states[1] = (states[1] + roll) % 10;
            scores[1] = scores[1] + states[1] + 1;
            if scores[1] >= 21 {
                new_wins[1] += count;
            } else {
                let rec_wins = play(states, scores, true, rolls, cache);
                new_wins[0] += rec_wins[0] * count;
                new_wins[1] += rec_wins[1] * count;
            }
        }
    }
    cache.insert((states, scores, p1_turn), new_wins);
    new_wins
    // if scores[0] >= 15 {
    //     *p1_wins += 1;
    //     // if *p1_wins % 1000000 == 0 {
    //     //     println!("Player 1 at {} wins", *p1_wins);
    //     // }
    // } else if p2_score >= 15 {
    //     *p2_wins += 1;
    //     // if *p2_wins % 1000000 == 0 {
    //     //     println!("Player 2 at {} wins", *p2_wins);
    //     // }
    // } else {
    //     if p1_turn {
    //         for i in 1..=3 {
    //             for j in 1..=3 {
    //                 for k in 1..=3 {
    //                     let turn = i + j + k;
    //                     let new_pos = (p1 + turn) % 10;
    //                     play(new_pos, p2, p1_score + new_pos + 1, p2_score, false, p1_wins, p2_wins);
    //                 }
    //             }
    //         }
    //     } else {
    //         for i in 1..=3 {
    //             for j in 1..=3 {
    //                 for k in 1..=3 {
    //                     let turn = i + j + k;
    //                     let new_pos = (p2 + turn) % 10;
    //                     play(p1, new_pos, p1_score, p2_score + new_pos + 1, true, p1_wins, p2_wins);
    //                 }
    //             }
    //         }
    //     }
    // }
}

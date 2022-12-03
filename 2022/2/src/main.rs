use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Hand {
    Unknown,
    Rock,
    Paper,
    Scissors
}

#[derive(Copy, Clone, Debug)]
struct Game {
    opponent: Hand,
    you: Hand
}

fn game_score(game: &Game) -> i32 {
    if game.opponent == game.you {
        // tie
        3
    } else if game.opponent == Hand::Rock && game.you == Hand::Scissors {
        0
    } else if game.opponent == Hand::Paper && game.you == Hand::Rock {
        0
    } else if game.opponent == Hand::Scissors && game.you == Hand::Paper {
        0
    } else {
        6
    }
}

fn get_hand(game: &Game) -> Hand {
    // Treat "you" as the win/tie/lose instead
    // Rock = lose, paper = tie, scissors = win
    match game.opponent {
        Hand::Rock => match game.you {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
            Hand::Unknown => Hand::Unknown
        }
        Hand::Paper => match game.you {
            Hand::Rock => Hand::Rock,
            Hand::Paper => Hand::Paper,
            Hand::Scissors => Hand::Scissors,
            Hand::Unknown => Hand::Unknown
        }
        Hand::Scissors => match game.you {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
            Hand::Unknown => Hand::Unknown
        }
        Hand::Unknown => Hand::Unknown
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut games: Vec<Game> = Vec::new();
    for line in lines.iter() {
        let mut hands = line.split(" ").into_iter();

        let op_hand = match hands.next().unwrap() {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            // Should never happen, don't know how to handle this case
            _ => Hand::Unknown
        };

        let your_hand = match hands.next().unwrap() {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            // Should never happen, don't know how to handle this case
            _ => Hand::Unknown
        };

        games.push(Game { opponent: op_hand, you: your_hand });
    }

    let mut total_score: i32 = 0;
    for game in games.iter() {
        println!("{:?}", game);
        let win_score = game_score(game);
        let hand_score = match game.you {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
            Hand::Unknown => 0
        };

        total_score += win_score + hand_score;
    }
    println!("{}", total_score);

    let mut second_score: i32 = 0;
    for game in games.iter() {
        let win_score = match game.you {
            Hand::Rock => 0,
            Hand::Paper => 3,
            Hand::Scissors => 6,
            Hand::Unknown => 0
        };
        let hand_score = match get_hand(game) {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
            Hand::Unknown => 0
        };
        second_score += win_score + hand_score;
    }
    println!("{}", second_score);
}

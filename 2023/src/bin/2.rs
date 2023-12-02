use std::fs;

struct Grab {
    red: u32,
    green: u32,
    blue: u32,
}

impl From<&str> for Grab {
    fn from(value: &str) -> Self {
        let colors = value.split(", ");

        let mut grab = Grab {
            red: 0,
            green: 0,
            blue: 0,
        };

        for block in colors {
            let count = block.split(' ').next().unwrap().parse::<u32>().unwrap();
            let color = block.split(' ').last().unwrap();
            match color {
                "red" => grab.red = count,
                "green" => grab.green = count,
                "blue" => grab.blue = count,
                _ => panic!(),
            }
        }

        grab
    }
}

struct Game {
    id: u32,
    grabs: Vec<Grab>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let pattern = regex::Regex::new(r"^Game (\d+): (.*)$").unwrap();

        let capture = pattern.captures(value).unwrap();

        let id = capture[1].parse::<u32>().unwrap();
        let grabs: Vec<Grab> = capture[2].split("; ").map(Grab::from).collect();

        Game { id, grabs }
    }
}

impl Game {
    fn power(&self) -> u32 {
        let red = self.grabs.iter().map(|g| g.red).max().unwrap();
        let green = self.grabs.iter().map(|g| g.green).max().unwrap();
        let blue = self.grabs.iter().map(|g| g.blue).max().unwrap();
        red * green * blue
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/2.txt").unwrap();

    let games: Vec<Game> = contents.lines().map(Game::from).collect();

    let mut sum = 0u32;
    for game in games.iter() {
        if game
            .grabs
            .iter()
            .all(|grab| grab.red <= 12 && grab.green <= 13 && grab.blue <= 14)
        {
            sum += game.id;
        }
    }
    println!("{}", sum);

    sum = 0;
    for game in games.iter() {
        sum += game.power();
    }
    println!("{}", sum);
}

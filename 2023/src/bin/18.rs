use std::collections::HashSet;

use geo::{Contains, Coord, LineString, Polygon};

struct Step {
    direction: String,
    length: u32,
    color: String,
}

impl Step {
    fn length_from_color(&self) -> u32 {
        u32::from_str_radix(&self.color[0..5], 16).unwrap()
    }

    fn dir_from_color(&self) -> &str {
        match self.color.chars().nth(5).unwrap() {
            '0' => "R",
            '1' => "D",
            '2' => "L",
            '3' => "U",
            _ => panic!(),
        }
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/18-sample.txt").unwrap();

    let pattern = regex::Regex::new(r"(\w) (\d+) \(#(\w{6})\)").unwrap();

    let mut steps: Vec<Step> = Vec::new();
    for line in contents.lines() {
        let captures = pattern.captures(line).unwrap();
        steps.push(Step {
            direction: captures[1].to_string(),
            length: captures[2].parse().unwrap(),
            color: captures[3].to_string(),
        })
    }

    let mut grid: HashSet<(i64, i64)> = Default::default();
    let mut pos = (0, 0);
    let mut points: Vec<(i64, i64)> = Vec::new();
    points.push(pos);
    for step in steps.iter() {
        let dir = match step.direction.as_ref() {
            "R" => (1, 0),
            "D" => (0, 1),
            "L" => (-1, 0),
            "U" => (0, -1),
            _ => panic!(),
        };
        for _ in 0..step.length {
            pos.0 += dir.0;
            pos.1 += dir.1;
            grid.insert(pos);
        }
        points.push(pos);
    }

    let line: LineString<f64> = LineString::new(
        points
            .into_iter()
            .map(|p| Into::into((p.0 as f64, p.1 as f64)))
            .collect(),
    );
    let polygon = Polygon::new(line, Vec::new());

    let min_x = grid.iter().map(|v| v.0).min().unwrap();
    let max_x = grid.iter().map(|v| v.0).max().unwrap();
    let min_y = grid.iter().map(|v| v.1).min().unwrap();
    let max_y = grid.iter().map(|v| v.1).max().unwrap();

    println!("{} {} {} {}", min_x, max_x, min_y, max_y);

    let mut total = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if grid.contains(&(x, y))
                || polygon.contains(&Into::<Coord>::into((x as f64, y as f64)))
            {
                total += 1;
            }
        }
    }

    println!("{}", total);

    pos = (0, 0);
    grid.clear();
    points = Vec::new();

    for step in steps.iter() {
        let dir = match step.dir_from_color() {
            "R" => (1, 0),
            "D" => (0, 1),
            "L" => (-1, 0),
            "U" => (0, -1),
            _ => panic!(),
        };
        for _ in 0..step.length_from_color() {
            pos.0 += dir.0;
            pos.1 += dir.1;
            grid.insert(pos);
        }
        points.push(pos);
    }

    let line: LineString<f64> = LineString::new(
        points
            .into_iter()
            .map(|p| Into::into((p.0 as f64, p.1 as f64)))
            .collect(),
    );
    let polygon = Polygon::new(line, Vec::new());

    let min_x = grid.iter().map(|v| v.0).min().unwrap();
    let max_x = grid.iter().map(|v| v.0).max().unwrap();
    let min_y = grid.iter().map(|v| v.1).min().unwrap();
    let max_y = grid.iter().map(|v| v.1).max().unwrap();

    println!("{} {} {} {}", min_x, max_x, min_y, max_y);

    let mut total = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if grid.contains(&(x, y))
                || polygon.contains(&Into::<Coord>::into((x as f64, y as f64)))
            {
                total += 1;
            }
        }
    }

    println!("{}", total);
}

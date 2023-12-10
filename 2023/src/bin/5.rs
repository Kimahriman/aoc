struct SeedIdMap {
    source_start: u64,
    dest_start: u64,
    length: u64,
}

struct SeedMap {
    source_type: String,
    dest_type: String,
    id_maps: Vec<SeedIdMap>,
}

impl SeedMap {
    fn resolve(&self, value: u64) -> u64 {
        for id_map in self.id_maps.iter() {
            if (id_map.source_start..(id_map.source_start + id_map.length)).contains(&value) {
                return value - id_map.source_start + id_map.dest_start;
            }
        }

        value
    }
}

fn resolve_seed(mut value: u64, seed_maps: &[SeedMap]) -> u64 {
    let mut state = "seed";

    while state != "location" {
        let seed_map = seed_maps.iter().find(|m| m.source_type == state).unwrap();
        state = seed_map.dest_type.as_ref();
        value = seed_map.resolve(value);
    }

    value
}

fn main() {
    let contents = std::fs::read_to_string("inputs/5.txt").unwrap();

    let mut lines = contents.lines();

    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    lines.next().unwrap();

    let mut seed_maps: Vec<SeedMap> = Vec::new();

    let map_pattern = regex::Regex::new(r"(\w+)-to-(\w+) map:").unwrap();
    let num_pattern = regex::Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
    while let Some(map_line) = lines.next() {
        let captures = map_pattern.captures(map_line).unwrap();
        let source_type = captures[1].to_string();
        let dest_type = captures[2].to_string();
        let mut id_maps: Vec<SeedIdMap> = Vec::new();

        for id_line in lines.by_ref() {
            if id_line.is_empty() {
                break;
            }

            let captures = num_pattern.captures(id_line).unwrap();
            let dest_start: u64 = captures[1].parse().unwrap();
            let source_start: u64 = captures[2].parse().unwrap();
            let length: u64 = captures[3].parse().unwrap();

            id_maps.push(SeedIdMap {
                source_start,
                dest_start,
                length,
            });
        }

        seed_maps.push(SeedMap {
            source_type,
            dest_type,
            id_maps,
        })
    }

    let mut lowest = u64::MAX;
    for seed in seeds.iter() {
        let value = resolve_seed(*seed, &seed_maps);
        if value < lowest {
            lowest = value;
        }
    }

    println!("{}", lowest);

    lowest = u64::MAX;
    for chunks in seeds.chunks(2) {
        let start = chunks[0];
        let length = chunks[1];

        for seed in start..(start + length) {
            let value = resolve_seed(seed, &seed_maps);
            if value < lowest {
                lowest = value;
            }
        }
    }

    println!("{}", lowest);
}

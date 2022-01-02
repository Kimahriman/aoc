use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};

#[derive(Copy, Clone, PartialEq, Debug)]
enum AmphipodType {
    A,
    B,
    C,
    D
}

impl AmphipodType {
    fn get(c: char) -> Self {
        if c == 'A' {
            Self::A
        } else if c == 'B' {
            Self::B
        } else if c == 'C' {
            Self::C
        } else {
            Self::D
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D'
        }
    }

    fn cost(&self) -> u32 {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000
        }
    }

    fn room(self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3
        }
    }

    fn encode(&self) -> u32 {
        self.room() as u32
    }
}

#[derive(Copy, Clone, Debug)]
struct Amphipod {
    typ: AmphipodType,
    finished: bool
}

impl Amphipod {
    fn new(typ: char) -> Self {
        Amphipod { typ: AmphipodType::get(typ), finished: false }
    }
}

#[derive(Clone, Debug)]
struct State {
    hallway: [Option<Amphipod>; 11],
    rooms: [(usize, Vec<Amphipod>); 4],
    energy: u32
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.energy == other.energy
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let res = if self.energy < other.energy {
            Ordering::Less
        } else if self.energy > other.energy {
            Ordering::Greater
        } else {
            Ordering::Equal
        };
        Some(res)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.energy < other.energy {
            Ordering::Less
        } else if self.energy > other.energy {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}



impl State {

    fn encode(&self) -> u128 {
        let mut state = 1u128;
        for h in self.hallway.iter() {
            state <<= 1;
            if let Some(amp) = h {
                state += 1;
                state <<= 2;
                state += amp.typ.encode() as u128;
            }
        }
        for (_, room) in self.rooms.iter() {
            state <<= 4 - room.len();
            // if room.len() == 0 {
            //     state <<= 2;
            // } else if room.len() == 1 {
            //     state <<= 1;
            // }
            for amp in room.iter() {
                state <<= 1;
                state += 1;
                state <<= 2;
                state += amp.typ.encode() as u128;
            }
        }
        state
    }

    fn print(&self) {
        println!("XXXXXXXXXXXXX");
        print!("#");
        for h in self.hallway.iter() {
            if let Some(amp) = h {
                print!("{}", amp.typ.to_char());
            } else {
                print!(".");
            }
        }
        print!("#");
        println!();

        print!("###");
        for (_, r) in self.rooms.iter() {
            if r.len() == 4 {
                print!("{}", r[3].typ.to_char());
            } else {
                print!(".");
            }
            print!("#");
        }
        print!("##");
        println!();

        for i in (0..3).rev() {
            print!("  #");
            for (_, r) in self.rooms.iter() {
                if r.len() >= i + 1 {
                    print!("{}", r[i].typ.to_char());
                } else {
                    print!(".");
                }
                print!("#");
            }
            print!("  ");
            println!();
        }
        println!("XXXXXXXXXXXXX");
    }

    fn validate(&self) -> bool {
        // Nothing can stop in front of a hallway
        for i in [2, 4, 6, 8] {
            if self.hallway[i].is_some() {
                return false;
            }
        }
        true
    }

    fn finished(&self) -> bool {
        for h in self.hallway.iter() {
            if h.is_some() {
                return false
            }
        }
        for (_, room) in self.rooms.iter() {
            for a in room.iter() {
                if !a.finished {
                    return false
                }
            }
        }
        true
    }

    fn branch(&self) -> Vec<Self> {
        let mut branches = Vec::<Self>::new();
        for (hx, h) in self.hallway.iter().enumerate().filter(|(_, x)| x.is_some()) {
            let amp = h.as_ref().unwrap();
            // Only option is to move into a room
            let (dx, dst_room) = &self.rooms[amp.typ.room()];
            if dst_room.len() != 0 && dst_room.iter().any(|x| !x.finished) {
                continue;
            }

            let mut blocked = false;
            let mut dist;
            if hx < *dx {
                dist = *dx - hx;
                for i in (hx + 1)..=*dx {
                    if self.hallway[i].is_some() {
                        // Something in the way
                        blocked = true;
                        break;
                    }
                }
            } else {
                dist = hx - *dx;
                for i in *dx..hx {
                    if self.hallway[i].is_some() {
                        // Something in the way
                        blocked = true;
                        break;
                    }
                }
            }

            if !blocked {
                dist += 4 - dst_room.len();
                let new_energy = self.energy + (dist as u32 * amp.typ.cost());
                let mut new_state = self.clone();
                new_state.energy = new_energy;
                
                let mut moved_amp = new_state.hallway[hx].unwrap();
                moved_amp.finished = true;
                new_state.hallway[hx] = None;
                new_state.rooms[moved_amp.typ.room()].1.push(moved_amp);
                branches.push(new_state);
            }
        }
        for (i, (rx, room)) in self.rooms.iter().enumerate() {
            // println!("Checking room {}", rx);
            let mut new_room = room.clone();
            if let Some(amp) = new_room.pop() {
                if amp.finished { continue }
                let get_out_dist = 4 - new_room.len();
                let mut x = *rx - 1;
                while self.hallway[x].is_none() {
                    // println!("{}", x);
                    let mut new_state = self.clone();
                    new_state.energy = self.energy + (get_out_dist + *rx - x) as u32 * amp.typ.cost();
                    new_state.rooms[i] = (*rx, new_room.clone());
                    new_state.hallway[x] = Some(amp);
                    branches.push(new_state);
                    if x == 0 { break; }
                    x -= 1;
                }

                let mut x = *rx + 1;
                while x < 11 && self.hallway[x].is_none() {
                    // println!("{}", x);
                    let mut new_state = self.clone();
                    new_state.energy = self.energy + (get_out_dist + x - *rx) as u32 * amp.typ.cost();
                    new_state.rooms[i] = (*rx, new_room.clone());
                    new_state.hallway[x] = Some(amp);
                    branches.push(new_state);
                    x += 1;
                }
            }
        }
        branches.into_iter().filter(|b| b.validate()).collect()
    }
}

fn main() {
    let initial_state = State {
        hallway: [None; 11],
        rooms: [
            (2, [Amphipod::new('C'), Amphipod::new('D'), Amphipod::new('D'), Amphipod::new('B')].to_vec()),
            (4, [Amphipod::new('D'), Amphipod::new('B'), Amphipod::new('C'), Amphipod::new('C')].to_vec()),
            (6, [Amphipod::new('D'), Amphipod::new('A'), Amphipod::new('B'), Amphipod::new('A')].to_vec()),
            (8, [Amphipod::new('A'), Amphipod::new('C'), Amphipod::new('A'), Amphipod::new('B')].to_vec())
            // (2, [Amphipod::new('A'), Amphipod::new('D'), Amphipod::new('D'), Amphipod::new('B')].to_vec()),
            // (4, [Amphipod::new('D'), Amphipod::new('B'), Amphipod::new('C'), Amphipod::new('C')].to_vec()),
            // (6, [Amphipod::new('C'), Amphipod::new('A'), Amphipod::new('B'), Amphipod::new('B')].to_vec()),
            // (8, [Amphipod::new('A'), Amphipod::new('C'), Amphipod::new('A'), Amphipod::new('D')].to_vec())
        ],
        energy: 0
    };

    let mut heap = BinaryHeap::<Reverse<State>>::new();
    let mut visited = HashSet::<u128>::new();
    heap.push(Reverse(initial_state));
    while let Some(Reverse(state)) = heap.pop() {
        let encoding = state.encode();
        if visited.contains(&encoding) {
            println!("Already visited state {}", state.energy);
            // state.print();
            continue;
        }
        visited.insert(encoding);
        println!("Checking state {:?}", state.energy);
        // state.print();
        if state.finished() {
            println!("Found the final state with {} energy", state.energy);
            break;
        }
        for new_state in state.branch().into_iter() {
            // println!("Adding branch {:?}", new_state.energy);
            heap.push(Reverse(new_state));
        }
    }
}


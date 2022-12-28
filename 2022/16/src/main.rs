use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use regex::Regex;

#[derive(Clone, Debug)]
struct Valve {
    id: String,
    flow_rate: i32,
    tunnels: Vec::<(String, i32)>
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct State {
    valve_id: String,
    elephant_valve_id: Option<String>,
    minutes_left: i32,
    pressure_released: i32,
    valves_open: Vec<String>,
    last_valve: Option<String>,
    elephant_last_valve: Option<String>
}

fn get_max_released(state: &State, valve_map: &HashMap<String, Valve>) -> i32 {
    let mut max_pressure = state.pressure_released;
    let mut remaining_valves: Vec<&Valve> = valve_map.iter().map(|(_, v)| v).filter(|v| !state.valves_open.contains(&v.id) && v.flow_rate > 0).collect();
    remaining_valves.sort_by(|a, b| b.flow_rate.cmp(&a.flow_rate));

    // Next round we open the biggest, then spend one minute moving to the next biggest, then open that, etc.
    for (i, valve) in remaining_valves.iter().enumerate() {
        max_pressure += std::cmp::max(state.minutes_left - (2 * (i / 2) as i32) - 1, 0) * valve.flow_rate;
    }

    max_pressure
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();

    let mut valve_map: HashMap<String, Valve> = HashMap::new();

    for line in lines.iter() {
        println!("{}", line);
        let cap = re.captures(line).unwrap();
        let id = cap.get(1).unwrap().as_str().to_string();
        let flow_rate: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
        let tunnels: Vec::<String> = cap.get(3).unwrap().as_str().split(", ").into_iter().map(|x| x.to_string()).collect();

        let valve = Valve { id, flow_rate, tunnels: tunnels.into_iter().map(|x| (x, 1)).collect() };
        valve_map.insert(valve.id.clone(), valve);
    }

    let mut resolved_valves: HashMap<String, Valve> = HashMap::new();

    for (_, valve) in valve_map.iter() {
        if valve.flow_rate == 0 && valve.id != "AA" {
            continue;
        }
        let mut new_tunnels: Vec<(String, i32)> = Vec::new();
        let mut visited_valves: HashSet<String> = HashSet::new();
        visited_valves.insert(valve.id.clone());
        let mut tunnel_queue: Vec<(String, i32)> = valve.tunnels.iter().map(|x| x.clone()).collect();

        while !tunnel_queue.is_empty() {
            let tunnel = tunnel_queue.remove(0);
            if visited_valves.contains(&tunnel.0) {
                continue;
            }
            visited_valves.insert(tunnel.0.clone());
            // println!("Working tunnel {:?}", tunnel);
            if (valve_map[&tunnel.0].flow_rate > 0 || valve_map[&tunnel.0].id == "AA") && tunnel.0 != valve.id {
                new_tunnels.push((tunnel.0.clone(), tunnel.1));
            } else {
                for t in valve_map[&tunnel.0].tunnels.iter() {
                    tunnel_queue.push((t.0.clone(), tunnel.1 + 1));
                }
            }
        }
        let mut new_valve = valve.clone();
        new_valve.tunnels = new_tunnels;
        resolved_valves.insert(valve.id.clone(), new_valve);
    }

    for r in resolved_valves.iter() {
        println!("{:?}", r);
    }
    // valve_map = resolved_valves;

    let mut state_queue: Vec<State> = Vec::new();

    state_queue.push(State {
        valve_id: "AA".to_string(),
        elephant_valve_id: None,
        minutes_left: 30,
        pressure_released: 0,
        valves_open: Vec::new(),
        last_valve: Some("AA".to_string()),
        elephant_last_valve: None,
    });

    let mut max_released = 0;

    let mut states_proccessed = 0;

    while !state_queue.is_empty() {
        states_proccessed += 1;
        if states_proccessed % 1000 == 0 {
            // println!("{} states processed", states_proccessed);
        }
        let next_state = state_queue.pop().unwrap();
        if get_max_released(&next_state, &valve_map) < max_released {
            continue;
        }
        // println!("{:?}", next_state);
        if next_state.minutes_left <= 0 || next_state.valves_open.len() == valve_map.len() - 1 {
            if next_state.pressure_released > max_released {
                max_released = next_state.pressure_released;
            }
            continue;
        }

        if valve_map[&next_state.valve_id].flow_rate > 0 && !next_state.valves_open.contains(&next_state.valve_id) {
            let mut valves_open = next_state.valves_open.clone();
            valves_open.push(next_state.valve_id.clone());

            let minutes_left = next_state.minutes_left - 1;
            let pressure_released = next_state.pressure_released + minutes_left * valve_map[&next_state.valve_id].flow_rate;

            let new_state = State {
                valve_id: next_state.valve_id.clone(),
                elephant_valve_id: None,
                minutes_left,
                pressure_released,
                valves_open,
                last_valve: None,
                elephant_last_valve: None,
            };
            state_queue.push(new_state);
        }

        for (tunnel, dist) in valve_map[&next_state.valve_id].tunnels.iter() {
            if Some(tunnel) == next_state.last_valve.as_ref() {
                continue;
            }
            let mut new_state = next_state.clone();
            new_state.last_valve = Some(new_state.valve_id.clone());
            new_state.valve_id = tunnel.clone();
            new_state.minutes_left -= dist;
            state_queue.push(new_state);
        }
    }

    println!("{} in {} states", max_released, states_proccessed);

    state_queue.push(State {
        valve_id: "AA".to_string(),
        elephant_valve_id: Some("AA".to_string()),
        minutes_left: 26,
        pressure_released: 0,
        valves_open: Vec::new(),
        last_valve: Some("AA".to_string()),
        elephant_last_valve: Some("AA".to_string())
    });

    let mut max_released = 0;

    let mut states_proccessed = 0;



    while !state_queue.is_empty() {
        states_proccessed += 1;
        let next_state = state_queue.pop().unwrap();
        if get_max_released(&next_state, &valve_map) < max_released {
            continue;
        }
        // println!("{:?}", next_state);
        if next_state.minutes_left <= 0 || next_state.valves_open.len() == valve_map.len() - 1 {
            if next_state.pressure_released > max_released {
                max_released = next_state.pressure_released;
                println!("New max released {}", max_released);
            }
            continue;
        }

        // 4 cases:
        // - Open both mine and elephants valve
        // - Open only my valve and elephant moves
        // - Open only elephant's valve and I move
        // - We both move
        if valve_map[&next_state.valve_id].flow_rate > 0 && !next_state.valves_open.contains(&next_state.valve_id) {
            let mut valves_open = next_state.valves_open.clone();
            valves_open.push(next_state.valve_id.clone());
            let minutes_left = next_state.minutes_left - 1;
            let mut pressure_released = next_state.pressure_released + minutes_left * valve_map[&next_state.valve_id].flow_rate;

            if valve_map[next_state.elephant_valve_id.as_ref().unwrap()].flow_rate > 0 && !valves_open.contains(next_state.elephant_valve_id.as_ref().unwrap()) {
                // First case, both opening valves
                valves_open.push(next_state.elephant_valve_id.as_ref().unwrap().clone());
                pressure_released += minutes_left * valve_map[next_state.elephant_valve_id.as_ref().unwrap()].flow_rate;

                let new_state = State {
                    valve_id: next_state.valve_id.clone(),
                    elephant_valve_id: Some(next_state.elephant_valve_id.as_ref().unwrap().clone()),
                    minutes_left,
                    pressure_released,
                    valves_open,
                    last_valve: None,
                    elephant_last_valve: None
                };
                state_queue.push(new_state);
            } else {
                // Second case, just me opening valves
                for (elephant_tunnel, _) in valve_map[next_state.elephant_valve_id.as_ref().unwrap()].tunnels.iter() {
                    if Some(elephant_tunnel) == next_state.elephant_last_valve.as_ref() {
                        continue;
                    }
                    let new_state = State {
                        valve_id: next_state.valve_id.clone(),
                        elephant_valve_id: Some(elephant_tunnel.clone()),
                        minutes_left,
                        pressure_released,
                        valves_open: valves_open.clone(),
                        last_valve: None,
                        elephant_last_valve: next_state.elephant_last_valve.clone()
                    };
                    state_queue.push(new_state);
                }
            }
        } else if valve_map[next_state.elephant_valve_id.as_ref().unwrap()].flow_rate > 0 && !next_state.valves_open.contains(next_state.elephant_valve_id.as_ref().unwrap()) {
            // Third case, just elephant opening valave
            let mut valves_open = next_state.valves_open.clone();
            valves_open.push(next_state.elephant_valve_id.as_ref().unwrap().clone());
            let minutes_left = next_state.minutes_left - 1;
            let pressure_released = next_state.pressure_released + minutes_left * valve_map[next_state.elephant_valve_id.as_ref().unwrap()].flow_rate;

            for (tunnel, _) in valve_map[&next_state.valve_id].tunnels.iter() {
                if Some(tunnel) == next_state.last_valve.as_ref() {
                    continue;
                }
                let new_state = State {
                    valve_id: tunnel.clone(),
                    elephant_valve_id: next_state.elephant_valve_id.clone(),
                    minutes_left,
                    pressure_released,
                    valves_open: valves_open.clone(),
                    last_valve: Some(next_state.valve_id.clone()),
                    elephant_last_valve: None
                };
                state_queue.push(new_state);
            }
        }

        // Final case, don't open either valve and move both
        for (tunnel, _) in valve_map[&next_state.valve_id].tunnels.iter() {
            if Some(tunnel) == next_state.last_valve.as_ref() {
                continue;
            }
            for (elephant_tunnel, _) in valve_map[next_state.elephant_valve_id.as_ref().unwrap()].tunnels.iter() {
                if Some(elephant_tunnel) == next_state.elephant_last_valve.as_ref() {
                    continue;
                }
                let mut new_state = next_state.clone();
                new_state.last_valve = Some(new_state.valve_id.clone());
                new_state.elephant_last_valve = new_state.elephant_valve_id.clone();
                new_state.valve_id = tunnel.clone();
                new_state.elephant_valve_id = Some(elephant_tunnel.clone());
                new_state.minutes_left -= 1;
                state_queue.push(new_state);
            }
        }
    }

    println!("{} in {} states", max_released, states_proccessed);
}

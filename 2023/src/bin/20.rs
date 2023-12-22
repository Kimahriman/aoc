use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Pulse {
    Low,
    High,
}

struct PulseInfo {
    source: String,
    target: String,
    pulse: Pulse,
}

trait Module {
    fn create(_inputs: Vec<String>) -> Box<dyn Module>
    where
        Self: Default + 'static,
    {
        Box::<Self>::default()
    }

    fn process_pulse(&mut self, source: String, pulse: Pulse) -> Option<Pulse>;
}

#[derive(Default)]
struct FlipFlop {
    on: bool,
}

impl Module for FlipFlop {
    fn process_pulse(&mut self, _source: String, pulse: Pulse) -> Option<Pulse> {
        if let Pulse::Low = pulse {
            self.on = !self.on;
            if self.on {
                Some(Pulse::High)
            } else {
                Some(Pulse::Low)
            }
        } else {
            None
        }
    }
}

#[derive(Default)]
struct Conjunction {
    memory: HashMap<String, Pulse>,
}

impl Module for Conjunction {
    fn create(inputs: Vec<String>) -> Box<dyn Module> {
        let mut memory: HashMap<String, Pulse> = Default::default();
        for input in inputs.into_iter() {
            memory.insert(input, Pulse::Low);
        }
        Box::new(Self { memory })
    }

    fn process_pulse(&mut self, source: String, pulse: Pulse) -> Option<Pulse> {
        self.memory.insert(source, pulse);
        if self.memory.values().all(|p| *p == Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }
}

#[derive(Default)]
struct Broadcast;

impl Module for Broadcast {
    fn process_pulse(&mut self, _source: String, pulse: Pulse) -> Option<Pulse> {
        Some(pulse)
    }
}

type ModuleCreate = fn(Vec<String>) -> Box<dyn Module>;

fn main() {
    let contents = std::fs::read_to_string("inputs/20.txt").unwrap();
    let pattern = regex::Regex::new(r"(\S+) -> (.*)$").unwrap();

    let mut module_infos: Vec<(String, ModuleCreate, Vec<String>)> = vec![];
    let mut sources: HashMap<String, Vec<String>> = Default::default();

    for line in contents.lines() {
        let captures = pattern.captures(line).unwrap();

        let (name, module_create): (String, ModuleCreate) = if captures[1].starts_with('%') {
            (captures[1][1..].to_string(), FlipFlop::create)
        } else if captures[1].starts_with('&') {
            (captures[1][1..].to_string(), Conjunction::create)
        } else {
            (captures[1].to_string(), Broadcast::create)
        };

        let targets: Vec<String> = captures[2].split(", ").map(Into::into).collect();
        for target in targets.iter() {
            if !sources.contains_key(target) {
                sources.insert(target.to_string(), vec![]);
            }
            sources.get_mut(target).unwrap().push(name.clone());
        }

        module_infos.push((name, module_create, targets));
    }

    let mut modules: HashMap<String, (Box<dyn Module>, Vec<String>)> = module_infos
        .into_iter()
        .map(|info| {
            let module = info.1(sources.remove(&info.0).unwrap_or(vec![]));
            (info.0.clone(), (module, info.2))
        })
        .collect();

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    let mut pulse_queue: VecDeque<PulseInfo> = Default::default();
    let mut min_rx_low = 0;

    let mut push_button = |track: bool| {
        pulse_queue.push_back(PulseInfo {
            source: "".to_string(),
            target: "broadcaster".to_string(),
            pulse: Pulse::Low,
        });

        if track {
            low_pulses += 1;
        }

        let mut min_rx_low = false;

        while let Some(pulse_info) = pulse_queue.pop_front() {
            // if pulse_info.target == "rx" {
            //     println!("Targeted rx: {:?}", pulse_info.pulse);
            // }
            if pulse_info.target == "rx" && pulse_info.pulse == Pulse::Low {
                min_rx_low = true;
            }
            if let Some((module, targets)) = modules.get_mut(&pulse_info.target) {
                if let Some(next_pulse) = module.process_pulse(pulse_info.source, pulse_info.pulse)
                {
                    for target in targets.iter() {
                        if track {
                            match next_pulse {
                                Pulse::Low => low_pulses += 1,
                                Pulse::High => high_pulses += 1,
                            }
                        }
                        pulse_queue.push_back(PulseInfo {
                            source: pulse_info.target.clone(),
                            target: target.clone(),
                            pulse: next_pulse,
                        });
                    }
                }
            }
        }

        min_rx_low
    };

    for i in 1..=1000 {
        if push_button(true) && min_rx_low == 0 {
            min_rx_low = i;
        }
    }

    let mut i = 1001;
    while min_rx_low == 0 {
        if i % 100000 == 0 {
            println!("At cycle {}", i);
        }
        if push_button(false) {
            min_rx_low = i;
            break;
        }
        i += 1;
    }
    println!("{}", low_pulses * high_pulses);
    println!("{}", min_rx_low);
}

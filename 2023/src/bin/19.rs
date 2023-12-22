use std::{collections::HashMap, convert::Infallible, ops::RangeInclusive, str::FromStr};

#[derive(Debug)]
enum Op {
    Gt(String, u32),
    Lt(String, u32),
    Default,
}

#[derive(Debug)]
struct Rule {
    op: Op,
    target: String,
}

impl Rule {
    fn matches(&self, p: &Part) -> bool {
        match &self.op {
            Op::Gt(var, val) => p.vals.get(var).unwrap() > val,
            Op::Lt(var, val) => p.vals.get(var).unwrap() < val,
            Op::Default => true,
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn process(&self, p: &Part) -> String {
        for rule in self.rules.iter() {
            if rule.matches(p) {
                return rule.target.clone();
            }
        }
        panic!()
    }
}

impl FromStr for Workflow {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = regex::Regex::new(r"^(\w+)\{(.*)\}").unwrap();
        let rule_pattern = regex::Regex::new(r"(\w)([><])(\d+):(\w+)").unwrap();

        let captures = pattern.captures(s).unwrap();

        let name = captures[1].to_string();
        let rules: Vec<Rule> = captures[2]
            .split(',')
            .map(|rule_str| {
                if let Some(rule) = rule_pattern.captures(rule_str) {
                    let var = rule[1].to_string();
                    let op = match &rule[2] {
                        "<" => Op::Lt(var, rule[3].parse().unwrap()),
                        ">" => Op::Gt(var, rule[3].parse().unwrap()),
                        _ => panic!(),
                    };
                    Rule {
                        op,
                        target: rule[4].to_string(),
                    }
                } else {
                    Rule {
                        op: Op::Default,
                        target: rule_str.to_string(),
                    }
                }
            })
            .collect();

        Ok(Workflow { name, rules })
    }
}

struct Part {
    vals: HashMap<String, u32>,
}

impl FromStr for Part {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s[1..s.len() - 1]
            .split(',')
            .map(|val| {
                let mut split = val.split('=');
                let var = split.next().unwrap();
                let val: u32 = split.next().unwrap().parse().unwrap();
                (var.to_string(), val)
            })
            .collect();

        Ok(Part { vals })
    }
}

#[derive(Clone, Debug)]
struct PartRange {
    workflow: String,
    ranges: HashMap<String, RangeInclusive<u32>>,
}

impl Default for PartRange {
    fn default() -> Self {
        Self {
            workflow: "in".to_string(),
            ranges: ["x", "m", "a", "s"]
                .into_iter()
                .map(|c| (c.to_string(), 1..=4000))
                .collect(),
        }
    }
}

impl PartRange {
    fn split(mut self, workflow: &Workflow) -> Vec<PartRange> {
        let mut splits: Vec<PartRange> = vec![];

        for rule in workflow.rules.iter() {
            match &rule.op {
                Op::Gt(var, val) => {
                    let cur_var_range = self.ranges.get(var).unwrap().clone();
                    let mut new_range = self.clone();

                    new_range
                        .ranges
                        .insert(var.to_string(), *val + 1..=*cur_var_range.end());
                    new_range.workflow = rule.target.clone();

                    splits.push(new_range);

                    // Update existing range for not matching
                    self.ranges
                        .insert(var.to_string(), *cur_var_range.start()..=*val);
                }
                Op::Lt(var, val) => {
                    let cur_var_range = self.ranges.get(var).unwrap().clone();
                    let mut new_range = self.clone();

                    new_range
                        .ranges
                        .insert(var.to_string(), *cur_var_range.start()..=*val - 1);
                    new_range.workflow = rule.target.clone();

                    splits.push(new_range);

                    self.ranges
                        .insert(var.to_string(), *val..=*cur_var_range.end());
                }
                Op::Default => {
                    self.workflow = rule.target.clone();
                    splits.push(self);
                    break;
                }
            }
        }

        splits
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/19.txt").unwrap();

    let mut lines = contents.lines();

    let mut workflows: HashMap<String, Workflow> = Default::default();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let workflow: Workflow = line.parse().unwrap();
        workflows.insert(workflow.name.clone(), workflow);
    }

    let parts: Vec<Part> = lines.map(|l| l.parse().unwrap()).collect();

    let mut sum: u32 = 0;
    for part in parts.iter() {
        let mut wf = workflows.get("in").unwrap();
        let result = loop {
            match wf.process(part).as_str() {
                "A" => break true,
                "R" => break false,
                w => wf = workflows.get(w).unwrap(),
            }
        };

        if result {
            sum += part.vals.values().cloned().sum::<u32>();
        }
    }

    println!("{}", sum);

    let mut part_ranges: Vec<PartRange> = vec![PartRange::default()];
    let mut final_ranges: Vec<PartRange> = vec![];

    while let Some(part_range) = part_ranges.pop() {
        let workflow = workflows.get(&part_range.workflow).unwrap();

        for new_range in part_range.split(workflow) {
            if new_range.workflow == "A" || new_range.workflow == "R" {
                final_ranges.push(new_range);
            } else {
                part_ranges.push(new_range);
            }
        }
    }

    let sum: u64 = final_ranges
        .into_iter()
        .filter_map(|port_range| {
            if port_range.workflow == "A" {
                Some(
                    port_range
                        .ranges
                        .into_values()
                        .map(|r| r.count() as u64)
                        .product::<u64>(),
                )
            } else {
                None
            }
        })
        .sum();

    println!("{}", sum);
}

// 167409079868000
//  78720000000000
// 256000000000000

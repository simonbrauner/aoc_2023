use std::collections::{HashMap, VecDeque};

use regex::Regex;

pub fn solve(input: &[String]) -> String {
    let mut modules = HashMap::new();
    let mut inputs_of_names = HashMap::new();

    let before_arrow_re = Regex::new(r"([%&]?)(\w+)").unwrap();
    let after_arrow_re = Regex::new(r"\w+").unwrap();

    for line in input {
        let [r#type, name] = before_arrow_re.captures(line).unwrap().extract().1;
        let outputs: Vec<_> = after_arrow_re
            .find_iter(line)
            .map(|m| m.as_str().to_string())
            .skip(1)
            .collect();

        modules.insert(name.to_string(), Module::new(r#type, outputs.clone()));

        for output in outputs {
            inputs_of_names
                .entry(output)
                .or_insert_with(HashMap::new)
                .insert(name.to_string(), Pulse::Low);
        }
    }

    for (name, name_inputs) in inputs_of_names {
        if !modules.contains_key(&name) {
            modules.insert(
                name.clone(),
                Module {
                    r#type: ModuleType::Broadcast,
                    output_names: Vec::new(),
                },
            );
        }

        if let ModuleType::Conjunction(ref mut inner_inputs) =
            modules.get_mut(&name).unwrap().r#type
        {
            inner_inputs.extend(name_inputs);
        }
    }

    format!("{}\n{}\n", part_1(&modules), part_2())
}

fn part_1(modules: &HashMap<String, Module>) -> usize {
    let mut modules = modules.clone();
    let mut counter = HashMap::new();

    for _ in 0..PUSH_COUNT {
        let mut queue = VecDeque::new();
        queue.push_back((BROADCASTER_NAME.to_string(), Pulse::Low));
        *counter.entry(Pulse::Low).or_insert(0) += 1;

        while let Some((sender_name, pulse)) = queue.pop_front() {
            let sender = modules.get(&sender_name).unwrap().clone();
            *counter.entry(pulse.clone()).or_insert(0) += sender.output_names.len();

            for receiver_name in sender.output_names.iter() {
                let receiver = modules.get_mut(receiver_name).unwrap();
                receiver.accept_pulse_from(receiver_name, &sender_name, pulse.clone(), &mut queue);
            }
        }
    }

    counter.values().product()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone, PartialEq)]
enum FlipFlopState {
    Off,
    On,
}

#[derive(Clone)]
struct Module {
    r#type: ModuleType,
    output_names: Vec<String>,
}

#[derive(Clone)]
enum ModuleType {
    Broadcast,
    FlipFlop(FlipFlopState),
    Conjunction(HashMap<String, Pulse>),
}

impl Module {
    fn new(symbol: &str, output_names: Vec<String>) -> Self {
        let r#type = match symbol {
            "" => ModuleType::Broadcast,
            "%" => ModuleType::FlipFlop(FlipFlopState::Off),
            "&" => ModuleType::Conjunction(HashMap::new()),
            _ => unreachable!(),
        };

        Module {
            r#type,
            output_names,
        }
    }
}

impl Module {
    fn accept_pulse_from(
        &mut self,
        self_name: &String,
        sender_name: &String,
        pulse: Pulse,
        queue: &mut VecDeque<(String, Pulse)>,
    ) {
        let pulse_to_send_option = match self.r#type {
            ModuleType::Broadcast => Some(pulse),
            ModuleType::FlipFlop(ref mut state) => {
                if pulse == Pulse::Low {
                    if *state == FlipFlopState::Off {
                        *state = FlipFlopState::On;
                        Some(Pulse::High)
                    } else {
                        *state = FlipFlopState::Off;
                        Some(Pulse::Low)
                    }
                } else {
                    None
                }
            }
            ModuleType::Conjunction(ref mut memory) => {
                *memory.get_mut(sender_name).unwrap() = pulse.clone();

                if memory.values().all(|remembered| *remembered == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        };

        if let Some(pulse_to_send) = pulse_to_send_option {
            queue.push_back((self_name.to_string(), pulse_to_send.clone()));
        }
    }
}

const BROADCASTER_NAME: &str = "broadcaster";
const PUSH_COUNT: usize = 1000;

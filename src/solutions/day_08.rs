use std::collections::HashMap;

use regex::Regex;

pub fn solve(input: &[String]) -> String {
    let mut network = HashMap::new();
    let node_re = Regex::new(r"(\w+)\s+=\s+\((\w+),\s+(\w+)\)").unwrap();

    for line in input.iter().skip(2) {
        let nodes = node_re.captures(line).unwrap().extract::<3>().1;
        network.insert(nodes[0], vec![nodes[1], nodes[2]]);
    }

    format!("{}\n{}\n", part_1(&input[0], &network), part_2())
}

fn part_1(instructions: &str, network: &HashMap<&str, Vec<&str>>) -> u32 {
    let mut instruction = instructions.chars().cycle();
    let mut steps = 0;

    let mut current = START;
    while current != END {
        current = network.get(current).unwrap()[instruction_index(&instruction.next().unwrap())];
        steps += 1;
    }

    steps
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

fn instruction_index(instruction: &char) -> usize {
    match instruction {
        'L' => 0,
        'R' => 1,
        _ => unreachable!(),
    }
}

const START: &str = "AAA";
const END: &str = "ZZZ";

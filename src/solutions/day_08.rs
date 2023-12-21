use std::collections::HashMap;

use num::integer::lcm;
use regex::Regex;

pub fn solve(input: &[String]) -> String {
    let mut network = HashMap::new();
    let node_re = Regex::new(r"(\w+)\s+=\s+\((\w+),\s+(\w+)\)").unwrap();

    for line in input.iter().skip(2) {
        let nodes = node_re.captures(line).unwrap().extract::<3>().1;
        network.insert(nodes[0], vec![nodes[1], nodes[2]]);
    }

    format!(
        "{}\n{}\n",
        part_1(&input[0], &network),
        part_2(&input[0], &network)
    )
}

fn part_1(instructions: &str, network: &HashMap<&str, Vec<&str>>) -> u64 {
    steps_required(instructions, network, START, &|node| node == END)
}

fn part_2(instructions: &str, network: &HashMap<&str, Vec<&str>>) -> u64 {
    network
        .keys()
        .filter(|node| is_start(node))
        .map(|node| steps_required(instructions, network, node, &is_end))
        .reduce(lcm)
        .unwrap()
}

fn steps_required(
    instructions: &str,
    network: &HashMap<&str, Vec<&str>>,
    start: &str,
    is_end: &dyn Fn(&str) -> bool,
) -> u64 {
    let mut instruction = instructions.chars().cycle();
    let mut steps = 0;

    let mut current = start;
    while !is_end(current) {
        current = network.get(current).unwrap()[instruction_index(&instruction.next().unwrap())];
        steps += 1;
    }

    steps
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

fn is_start(node: &str) -> bool {
    node.chars().nth(2) == START.chars().nth(2)
}

fn is_end(node: &str) -> bool {
    node.chars().nth(2) == END.chars().nth(2)
}

use std::collections::HashMap;

use rayon::prelude::*;
use regex::Regex;

pub fn solve(input: &[String]) -> String {
    let workflow_re = Regex::new(r"^(\w+)\{(.*)\}$").unwrap();
    let part_re = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();

    let mut input_iterator = input.iter();
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();

    for line in input_iterator.by_ref() {
        if line.is_empty() {
            break;
        }

        let [name, rules] = workflow_re
            .captures(line)
            .unwrap()
            .extract::<2>()
            .1
            .map(|s| s.to_string());
        workflows.insert(name, rules);
    }

    for line in input_iterator {
        let [x, m, a, s] = part_re
            .captures(line)
            .unwrap()
            .extract::<4>()
            .1
            .map(|s| s.parse::<u64>().unwrap());
        parts.push(Part { x, m, a, s });
    }

    format!("{}\n{}\n", part_1(&workflows, &parts), part_2(&workflows))
}

fn part_1(workflows: &HashMap<String, String>, parts: &[Part]) -> u64 {
    parts
        .iter()
        .filter(|part| part.eval(workflows, "in"))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

fn part_2(workflows: &HashMap<String, String>) -> u64 {
    let edge_cases = Part::compute_edge_cases(workflows);
    let x_edges = edge_cases.get("x").unwrap();
    let m_edges = edge_cases.get("m").unwrap();
    let a_edges = edge_cases.get("a").unwrap();
    let s_edges = edge_cases.get("s").unwrap();

    x_edges
        .par_iter()
        .zip(x_edges.par_iter().skip(1))
        .map(|(&x, &next_x)| {
            let mut subresult = 0;

            for (&m, &next_m) in m_edges.iter().zip(m_edges.iter().skip(1)) {
                for (&a, &next_a) in a_edges.iter().zip(a_edges.iter().skip(1)) {
                    for (&s, &next_s) in s_edges.iter().zip(s_edges.iter().skip(1)) {
                        let part = Part { x, m, a, s };
                        if part.eval(workflows, "in") {
                            subresult += (next_x - x) * (next_m - m) * (next_a - a) * (next_s - s);
                        }
                    }
                }
            }

            subresult
        })
        .sum()
}

struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn eval(&self, workflows: &HashMap<String, String>, current_name: &str) -> bool {
        if current_name == "A" {
            return true;
        }
        if current_name == "R" {
            return false;
        }

        for rule in workflows.get(current_name).unwrap().split(',') {
            if let Some(operator) = Self::extract_operator(rule) {
                let (condition, next_name) = rule.split_once(':').unwrap();
                let (component, number) = condition.split_once(operator).unwrap();
                let number = number.parse::<u64>().unwrap();

                if (operator == '<' && self.get(component) < number)
                    || (operator == '>' && self.get(component) > number)
                {
                    return self.eval(workflows, next_name);
                }
            } else {
                return self.eval(workflows, rule);
            };
        }

        unreachable!();
    }

    fn compute_edge_cases(workflows: &HashMap<String, String>) -> HashMap<String, Vec<u64>> {
        let mut edge_cases: HashMap<String, Vec<u64>> = "xmas"
            .chars()
            .map(|ch| (ch.to_string(), vec![1, 4001]))
            .collect();

        for rules in workflows.values() {
            for rule in rules.split(',') {
                if let Some(operator) = Self::extract_operator(rule) {
                    let (condition, _) = rule.split_once(':').unwrap();
                    let (component, number) = condition.split_once(operator).unwrap();
                    let mut number = number.parse::<u64>().unwrap();

                    if operator == '>' {
                        number += 1;
                    }

                    edge_cases.get_mut(component).unwrap().push(number);
                }
            }
        }

        for numbers in edge_cases.values_mut() {
            numbers.sort();
            numbers.dedup();
        }

        edge_cases
    }

    fn get(&self, component: &str) -> u64 {
        match component {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => unreachable!(),
        }
    }

    fn extract_operator(rule: &str) -> Option<char> {
        if rule.contains('<') {
            Some('<')
        } else if rule.contains('>') {
            Some('>')
        } else {
            None
        }
    }
}

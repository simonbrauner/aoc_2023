use std::collections::HashMap;

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
            .map(|s| s.parse::<usize>().unwrap());
        parts.push(Part { x, m, a, s });
    }

    format!("{}\n{}\n", part_1(&workflows, &parts), part_2())
}

fn part_1(workflows: &HashMap<String, String>, parts: &[Part]) -> usize {
    parts
        .iter()
        .filter(|part| part.eval(workflows, "in"))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
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
            let operator = if rule.contains('<') {
                Some('<')
            } else if rule.contains('>') {
                Some('>')
            } else {
                None
            };

            if let Some(operator) = operator {
                let (condition, next_name) = rule.split_once(':').unwrap();
                let (component, number) = condition.split_once(operator).unwrap();
                let number = number.parse::<usize>().unwrap();

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

    fn get(&self, component: &str) -> usize {
        match component {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => unreachable!(),
        }
    }
}

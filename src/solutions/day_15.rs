use indexmap::IndexMap;
use itertools::Itertools;

pub fn solve(input: &[String]) -> String {
    let init_sequence: Vec<_> = input[0].split(',').collect();

    format!("{}\n{}\n", part_1(&init_sequence), part_2(&init_sequence))
}

fn part_1(init_sequence: &[&str]) -> usize {
    init_sequence.iter().map(hash).sum()
}

fn part_2(init_sequence: &[&str]) -> usize {
    let mut boxes: Vec<_> = (0..256).map(|_| IndexMap::<&str, usize>::new()).collect();

    for operation in init_sequence {
        if operation.contains('-') {
            let label = operation.split('-').next().unwrap();
            boxes[hash(&label)].shift_remove(label);
        } else {
            let (label, focal_length) = operation.split('=').collect_tuple().unwrap();
            let focal_length = focal_length.parse::<usize>().unwrap();

            boxes[hash(&label)].insert(label, focal_length);
        }
    }

    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_index, r#box)| {
            r#box
                .iter()
                .enumerate()
                .map(move |(lens_index, (_, focal_length))| {
                    (box_index + 1) * (lens_index + 1) * focal_length
                })
        })
        .sum()
}

fn hash(step: &&str) -> usize {
    let mut current_value: usize = 0;

    for ascii_value in step.as_bytes() {
        current_value += *ascii_value as usize;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

pub fn solve(input: &[String]) -> String {
    let init_sequence: Vec<_> = input[0].split(',').collect();

    format!("{}\n{}\n", part_1(&init_sequence), part_2())
}

fn part_1(init_sequence: &[&str]) -> usize {
    init_sequence.iter().map(hash).sum()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
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

pub fn solve(input: &[String]) -> String {
    let sequences: Vec<Vec<i64>> = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();

    format!("{}\n{}\n", part_1(&sequences), part_2())
}

fn part_1(sequences: &[Vec<i64>]) -> i64 {
    sequences.iter().cloned().map(next_value).sum()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

fn next_value(sequence: Vec<i64>) -> i64 {
    let mut sequences = vec![sequence];

    while let Some(current) = sequences.last() {
        if current.iter().all(|number| number == &0) {
            break;
        }

        sequences.push(
            current
                .iter()
                .zip(current.iter().skip(1))
                .map(|(left, right)| right - left)
                .collect(),
        );
    }

    sequences
        .iter()
        .map(|sequence| sequence.last().unwrap())
        .sum()
}

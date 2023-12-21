pub fn solve(input: &[String]) -> String {
    let sequences: Vec<Vec<i64>> = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();

    format!("{}\n{}\n", part_1(&sequences), part_2(&sequences))
}

fn part_1(sequences: &[Vec<i64>]) -> i64 {
    sum_of_predicted_values(
        sequences,
        &|sequence| *sequence.last().unwrap(),
        &|accumulator, number| accumulator + number,
    )
}

fn part_2(sequences: &[Vec<i64>]) -> i64 {
    sum_of_predicted_values(
        sequences,
        &|sequence| *sequence.first().unwrap(),
        &|accumulator, number| -accumulator + number,
    )
}

fn sum_of_predicted_values(
    sequences: &[Vec<i64>],
    location: &dyn Fn(&Vec<i64>) -> i64,
    extrapolation: &dyn Fn(i64, i64) -> i64,
) -> i64 {
    sequences
        .iter()
        .cloned()
        .map(|sequence| predict_value(sequence, location, extrapolation))
        .sum()
}

fn predict_value(
    sequence: Vec<i64>,
    location: &dyn Fn(&Vec<i64>) -> i64,
    extrapolation: &dyn Fn(i64, i64) -> i64,
) -> i64 {
    sequence_with_differences(sequence)
        .iter()
        .rev()
        .map(location)
        .fold(0, extrapolation)
}

fn sequence_with_differences(sequence: Vec<i64>) -> Vec<Vec<i64>> {
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
}

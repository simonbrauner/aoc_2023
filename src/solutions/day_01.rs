pub fn solve(input: &[String]) -> String {
    format!("{}\n{}\n", part_1(input), part_2())
}

fn part_1(document: &[String]) -> u32 {
    calibration_sum(document)
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

fn calibration_sum(document: &[String]) -> u32 {
    document
        .iter()
        .map(|line| {
            let mut digits = line.chars().filter_map(|letter| letter.to_digit(10));

            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);

            10 * first + last
        })
        .sum()
}

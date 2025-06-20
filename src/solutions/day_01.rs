pub fn solve(input: &[String]) -> String {
    format!("{}\n{}\n", part_1(input), part_2(input))
}

fn part_1(document: &[String]) -> u32 {
    calibration_sum(document)
}

fn part_2(document: &[String]) -> u32 {
    let modified_document: Vec<String> = document
        .iter()
        .map(|line| {
            let mut new_line = line.to_string();

            for (word, digit) in WORDS_AND_DIGITS {
                new_line = new_line.replace(word, &format!("{}{}{}", word, digit, word));
            }

            new_line
        })
        .collect();

    calibration_sum(&modified_document)
}

fn calibration_sum(document: &[String]) -> u32 {
    document
        .iter()
        .map(|line| {
            let mut digits = line.chars().filter_map(|letter| letter.to_digit(10));

            let first = digits.next().unwrap();
            let last = digits.next_back().unwrap_or(first);

            10 * first + last
        })
        .sum()
}

const WORDS_AND_DIGITS: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

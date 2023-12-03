pub fn solve(input: &[String]) -> String {
    let mut engine = Vec::new();

    for line in input {
        engine.push(line.chars().collect())
    }

    format!("{}\n{}\n", part_1(&engine), part_2())
}

fn part_1(engine: &[Vec<char>]) -> u32 {
    let mut part_number_sum = 0;

    for (row, line) in engine.iter().enumerate() {
        let mut number = 0;
        let mut valid = false;

        for (col, letter) in line.iter().enumerate() {
            if let Some(digit) = letter.to_digit(10) {
                number *= 10;
                number += digit;

                valid = valid || validity(engine, col, row);

                if !line.get(col + 1).unwrap_or(&'.').is_ascii_digit() {
                    if valid {
                        part_number_sum += number;
                    }

                    number = 0;
                    valid = false;
                }
            }
        }
    }

    part_number_sum
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

fn is_symbol(letter: &char) -> bool {
    !letter.is_ascii_digit() && letter != &'.'
}

fn validity(engine: &[Vec<char>], col_index: usize, row_index: usize) -> bool {
    (-1..=1)
        .filter_map(|row_change| row_index.checked_add_signed(row_change))
        .filter_map(|row_index| engine.get(row_index))
        .flat_map(|row| {
            (-1..=1)
                .filter_map(|col_change| col_index.checked_add_signed(col_change))
                .filter_map(|col_index| row.get(col_index))
        })
        .any(is_symbol)
}

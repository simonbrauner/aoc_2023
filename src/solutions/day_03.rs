use std::collections::{HashMap, HashSet};

pub fn solve(input: &[String]) -> String {
    let mut part_numbers: Vec<u32> = Vec::new();
    let mut part_numbers_indices: HashMap<(usize, usize), usize> = HashMap::new();
    let mut symbols: HashMap<(usize, usize), char> = HashMap::new();

    for (row, line) in input.iter().enumerate() {
        let mut number = 0;
        let mut length = 0;
        let line: Vec<char> = line.chars().collect();

        for (col, letter) in line.iter().enumerate() {
            if let Some(digit) = letter.to_digit(10) {
                number *= 10;
                number += digit;
                length += 1;

                if !line.get(col + 1).unwrap_or(&'.').is_ascii_digit() {
                    for col_decrement in 0..length {
                        part_numbers_indices.insert((col - col_decrement, row), part_numbers.len());
                    }

                    part_numbers.push(number);
                    number = 0;
                    length = 0;
                }
            } else if is_symbol(letter) {
                symbols.insert((col, row), *letter);
            }
        }
    }

    let engine = Engine {
        part_numbers,
        part_numbers_indices,
        symbols,
    };

    format!("{}\n{}\n", part_1(&engine), part_2(&engine))
}

fn part_1(engine: &Engine) -> u32 {
    let mut indices: HashSet<usize> = HashSet::new();

    for (col, row) in engine.symbols.keys() {
        indices.extend(get_adjacent_indices(col, row, engine));
    }

    indices
        .iter()
        .map(|index| engine.part_numbers.get(*index).unwrap())
        .sum()
}

fn part_2(engine: &Engine) -> u32 {
    engine
        .symbols
        .iter()
        .filter_map(|((col, row), symbol)| {
            let adjacent_indices = get_adjacent_indices(col, row, engine);

            if symbol == &'*' && adjacent_indices.len() == 2 {
                Some(
                    adjacent_indices
                        .iter()
                        .map(|index| engine.part_numbers.get(*index).unwrap())
                        .product::<u32>(),
                )
            } else {
                None
            }
        })
        .sum()
}

struct Engine {
    part_numbers: Vec<u32>,
    part_numbers_indices: HashMap<(usize, usize), usize>,
    symbols: HashMap<(usize, usize), char>,
}

fn is_symbol(letter: &char) -> bool {
    !letter.is_ascii_digit() && letter != &'.'
}

fn get_adjacent_indices(col: &usize, row: &usize, engine: &Engine) -> HashSet<usize> {
    (-1..=1)
        .filter_map(|col_change| col.checked_add_signed(col_change))
        .flat_map(|col| {
            (-1..=1)
                .filter_map(|row_change| row.checked_add_signed(row_change))
                .filter_map(move |row| engine.part_numbers_indices.get(&(col, row)))
                .copied()
        })
        .collect()
}

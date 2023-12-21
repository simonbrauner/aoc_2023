use std::{
    cmp::{max, min},
    collections::HashSet,
};

pub fn solve(input: &[String]) -> String {
    let image = expanded_image(input);

    format!("{}\n{}\n", part_1(&image), part_2())
}

fn part_1(image: &[Vec<char>]) -> usize {
    let mut galaxies = HashSet::new();

    for (row_index, row) in image.iter().enumerate() {
        for (col_index, pixel) in row.iter().enumerate() {
            if pixel == &'#' {
                galaxies.insert((col_index, row_index));
            }
        }
    }

    let mut lengths = 0;
    for (col, row) in &galaxies {
        for (other_col, other_row) in &galaxies {
            lengths += max(col, other_col) - min(col, other_col);
            lengths += max(row, other_row) - min(row, other_row);
        }
    }

    lengths / 2
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

fn expanded_image(image: &[String]) -> Vec<Vec<char>> {
    let mut expanded: Vec<Vec<char>> = Vec::new();

    for row in image {
        expanded.push(row.chars().collect());
        if !row.contains('#') {
            expanded.push(row.chars().collect());
        }
    }

    let mut col_index = 0;
    while col_index < expanded[0].len() {
        if expanded.iter().all(|row| row[col_index] == '.') {
            for row in expanded.iter_mut() {
                row.insert(col_index, '.');
            }

            col_index += 1;
        }

        col_index += 1;
    }

    expanded
}

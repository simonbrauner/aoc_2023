use std::{
    cmp::{max, min},
    collections::HashSet,
};

pub fn solve(input: &[String]) -> String {
    let mut galaxies = HashSet::new();
    let mut expanded_rows = Vec::new();

    for (row_index, row) in input.iter().enumerate() {
        let mut galaxy_found = false;

        for (col_index, letter) in row.char_indices() {
            if letter == '#' {
                galaxies.insert((col_index, row_index));
                galaxy_found = true;
            }
        }

        expanded_rows.push(!galaxy_found);
    }

    let mut expanded_cols = Vec::new();

    for col_index in 0..input[0].len() {
        expanded_cols.push(
            input
                .iter()
                .all(|row| row.chars().nth(col_index).unwrap() == '.'),
        );
    }

    let image = Image {
        galaxies,
        expanded_cols,
        expanded_rows,
    };

    format!("{}\n{}\n", part_1(&image), part_2(&image))
}

fn part_1(image: &Image) -> usize {
    sum_of_lenghts(image, 2)
}

fn part_2(image: &Image) -> usize {
    sum_of_lenghts(image, 1_000_000)
}

fn sum_of_lenghts(image: &Image, expansion: usize) -> usize {
    let mut lengths = 0;

    for (col, row) in &image.galaxies {
        for (other_col, other_row) in &image.galaxies {
            lengths += (*min(col, other_col)..*max(col, other_col))
                .map(|index| {
                    if image.expanded_cols[index] {
                        expansion
                    } else {
                        1
                    }
                })
                .sum::<usize>();

            lengths += (*min(row, other_row)..*max(row, other_row))
                .map(|index| {
                    if image.expanded_rows[index] {
                        expansion
                    } else {
                        1
                    }
                })
                .sum::<usize>();
        }
    }

    lengths / 2
}

struct Image {
    galaxies: HashSet<(usize, usize)>,
    expanded_cols: Vec<bool>,
    expanded_rows: Vec<bool>,
}

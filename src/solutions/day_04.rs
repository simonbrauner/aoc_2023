use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

pub fn solve(input: &[String]) -> String {
    let mut cards = Vec::new();
    let card_re = Regex::new(r"Card\s+\d+:([\s|\d]*)\|([\s\d]*)").unwrap();

    for line in input {
        let (winning_numbers, my_numbers) = card_re
            .captures(line)
            .unwrap()
            .extract::<2>()
            .1
            .into_iter()
            .map(|numbers| {
                numbers
                    .split_whitespace()
                    .map(|number| number.parse().unwrap())
                    .collect()
            })
            .collect_tuple()
            .unwrap();

        cards.push(Card {
            winning_numbers,
            my_numbers,
        });
    }
    format!("{}\n{}\n", part_1(&cards), part_2())
}

fn part_1(cards: &[Card]) -> u32 {
    cards.iter().map(card_points).sum()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

struct Card {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

fn card_points(card: &Card) -> u32 {
    match card
        .my_numbers
        .iter()
        .filter(|number| card.winning_numbers.contains(number))
        .count() as u32
    {
        0 => 0,
        number => 2_u32.pow(number - 1),
    }
}

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

    format!("{}\n{}\n", part_1(&cards), part_2(&cards))
}

fn part_1(cards: &[Card]) -> u32 {
    cards
        .iter()
        .map(|card| match card.winning_count() {
            0 => 0,
            number => 2_u32.pow(number as u32 - 1),
        })
        .sum()
}

fn part_2(cards: &[Card]) -> u32 {
    let mut card_counts: Vec<u32> = (0..cards.len()).map(|_| 1).collect();

    for (index, card) in cards.iter().enumerate() {
        for new_card_index in index + 1..index + 1 + card.winning_count() {
            card_counts[new_card_index] += card_counts[index];
        }
    }

    card_counts.iter().sum()
}

struct Card {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl Card {
    fn winning_count(&self) -> usize {
        self.my_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }
}

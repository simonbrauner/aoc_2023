use itertools::Itertools;
use regex::Regex;

pub fn solve(input: &[String]) -> String {
    let hand_re = Regex::new(r"(\w+) (\d+)").unwrap();

    let hands: Vec<_> = input
        .iter()
        .map(|line| {
            let hand = hand_re.captures(line).unwrap().extract::<2>().1;
            let cards = hand[0].chars().map(Card::new).collect();
            let bid = hand[1].parse::<u32>().unwrap();

            Hand { cards, bid }
        })
        .collect();

    format!("{}\n{}\n", part_1(&hands), part_2(&hands))
}

fn part_1(hands: &[Hand]) -> usize {
    total_winnings(hands)
}

fn part_2(hands: &[Hand]) -> usize {
    let mut hands_with_jokers = hands.to_vec();
    for hand in hands_with_jokers.iter_mut() {
        hand.cards.iter_mut().for_each(|card| {
            if *card == Card::Jack {
                *card = Card::Joker;
            }
        });
    }

    total_winnings(&hands_with_jokers)
}

fn total_winnings(hands: &[Hand]) -> usize {
    hands
        .iter()
        .sorted_by(|a, b| {
            a.compute_type()
                .cmp(&b.compute_type())
                .then_with(|| a.cards.cmp(&b.cards))
        })
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bid as usize)
        .sum()
}

#[derive(Clone)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn compute_type(&self) -> Type {
        let joker_count = self
            .cards
            .iter()
            .filter(|&card| *card == Card::Joker)
            .count();

        let counts: Vec<_> = self
            .cards
            .iter()
            .filter(|&card| *card != Card::Joker)
            .counts()
            .values()
            .cloned()
            .sorted()
            .rev()
            .collect();

        match (
            *counts.first().unwrap_or(&0) + joker_count,
            *counts.get(1).unwrap_or(&0),
        ) {
            (5, _) => Type::FiveOfAKind,
            (4, _) => Type::FourOfAKind,
            (3, 2) => Type::FullHouse,
            (3, 1) => Type::ThreeOfAKind,
            (2, 2) => Type::TwoPairs,
            (2, 1) => Type::OnePair,
            _ => Type::HighCard,
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Number(u32),
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn new(c: char) -> Self {
        match c {
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            'T' => Card::Number(10),
            _ => Card::Number(c.to_digit(10).unwrap()),
        }
    }
}

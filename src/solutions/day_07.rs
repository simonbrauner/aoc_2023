use itertools::Itertools;
use regex::Regex;

pub fn solve(input: &[String]) -> String {
    let hand_re = Regex::new(r"(\w+) (\d+)").unwrap();

    let mut hands: Vec<_> = input
        .iter()
        .map(|line| {
            let hand = hand_re.captures(line).unwrap().extract::<2>().1;
            let cards = hand[0].chars().into_iter().map(Card::new).collect();
            let bid = hand[1].parse::<u32>().unwrap();

            Hand { cards, bid }
        })
        .collect();

    hands.sort_by(|a, b| {
        a.compute_type()
            .cmp(&b.compute_type())
            .then_with(|| a.cards.cmp(&b.cards))
    });

    format!("{}\n{}\n", part_1(&hands), part_2())
}

fn part_1(hands: &[Hand]) -> usize {
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bid as usize)
        .sum()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
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
        let counts: Vec<_> = self
            .cards
            .iter()
            .counts()
            .values()
            .cloned()
            .sorted()
            .rev()
            .collect();

        match (counts.get(0), counts.get(1)) {
            (Some(5), _) => Type::FiveOfAKind,
            (Some(4), _) => Type::FourOfAKind,
            (Some(3), Some(2)) => Type::FullHouse,
            (Some(3), Some(1)) => Type::ThreeOfAKind,
            (Some(2), Some(2)) => Type::TwoPairs,
            (Some(2), Some(1)) => Type::OnePair,
            _ => Type::HighCard,
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
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

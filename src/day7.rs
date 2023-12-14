use once_cell::unsync::Lazy;
use std::{collections::HashMap, str::FromStr};

use crate::utils;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&[Card; 5]> for Type {
    fn from(value: &[Card; 5]) -> Self {
        let mut count = vec![0; VALID_CARDS.len()];
        for card in value {
            count[card.card as usize - 1] += 1;
        }
        let jokers = count[0];
        let count = &count[1..];

        // AA2J4
        match *count.iter().max().unwrap() {
            5 => Type::FiveOfAKind,
            4 if jokers == 1 => Type::FiveOfAKind,
            4 => Type::FourOfAKind,
            3 if jokers == 1 => Type::FourOfAKind,
            3 if jokers == 2 => Type::FiveOfAKind,
            3 if count.iter().any(|x| *x == 2) => Type::FullHouse,
            3 => Type::ThreeOfAKind,
            2 if jokers == 2 => Type::FourOfAKind,
            2 if jokers == 3 => Type::FiveOfAKind,
            2 if count.iter().filter(|x| **x == 2).count() > 1 => {
                if jokers == 0 {
                    Type::TwoPair
                } else {
                    Type::FullHouse
                }
            }
            2 if jokers == 1 => Type::ThreeOfAKind,
            2 => Type::OnePair,
            1 if jokers == 1 => Type::OnePair,
            1 if jokers == 2 => Type::ThreeOfAKind,
            1 if jokers == 3 => Type::FourOfAKind,
            1 if jokers == 4 => Type::FiveOfAKind,
            1 => Type::HighCard,
            0 => {
                debug_assert!(jokers == 5);
                Type::FiveOfAKind
            }
            m => unreachable!("Jokers: {jokers}, max: {m}"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
struct Card {
    card: u8,
}

impl Default for Card {
    fn default() -> Self {
        Card { card: 1 }
    }
}

const VALID_CARDS: Lazy<HashMap<char, u8>> = Lazy::new(|| {
    HashMap::from([
        ('J', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ])
});

const CARD_TO_CHAR: Lazy<HashMap<u8, char>> = Lazy::new(|| {
    HashMap::from([
        (1, 'J'),
        (2, '2'),
        (3, '3'),
        (4, '4'),
        (5, '5'),
        (6, '6'),
        (7, '7'),
        (8, '8'),
        (9, '9'),
        (10, 'T'),
        (11, 'Q'),
        (12, 'K'),
        (13, 'A'),
    ])
});

impl TryFrom<char> for Card {
    type Error = utils::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match VALID_CARDS.get(&value) {
            Some(value) => Ok(Card { card: *value }),
            None => Err(format!("Invalid card: {value}").into()),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Hand {
    cards: [Card; 5],
    typ: Type,
}

impl FromStr for Hand {
    type Err = utils::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards: [Card; 5] = [Card::default(); 5];
        for (i, c) in s.chars().enumerate() {
            if let Ok(card) = c.try_into() {
                cards[i] = card;
            }
        }

        let typ: Type = (&cards).into();

        Ok(Hand { cards, typ })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.typ.cmp(&other.typ) {
            c @ std::cmp::Ordering::Less => Some(c),
            c @ std::cmp::Ordering::Greater => Some(c),
            std::cmp::Ordering::Equal => {
                for i in 0..5 {
                    let c1 = &self.cards[i];
                    let c2 = &other.cards[i];

                    match c1.cmp(c2) {
                        c @ std::cmp::Ordering::Less => {
                            return Some(c);
                        }
                        c @ std::cmp::Ordering::Greater => {
                            return Some(c);
                        }
                        std::cmp::Ordering::Equal => (),
                    }
                }

                Some(std::cmp::Ordering::Equal)
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("Hand::partial_cmp should be complete")
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.cards {
            write!(f, "{}", CARD_TO_CHAR[&c.card])?;
        }
        write!(f, " {:?}", &self.typ)
    }
}

pub fn main() -> utils::ACResult {
    let mut hands: Vec<_> = utils::read_lines(7)?
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand: Hand = hand.parse().expect("Failed to parse hand.");
            let bid: u32 = bid.parse().expect("Failed to parse bid.");
            (hand, bid)
        })
        .collect();
    hands.sort();

    let ans2: usize = hands
        .iter()
        .enumerate()
        .map(|(i, (_, b))| *b as usize * (i + 1))
        .sum();

    println!("Day 7, Answer 2: {ans2}");

    Ok(())
}

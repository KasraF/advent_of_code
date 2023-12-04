use std::str::FromStr;

use crate::utils;

struct Card {
    winners: [u32; 10],
    numbers: [u32; 25],
}

impl FromStr for Card {
    type Err = utils::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, rest) = s.split_once(':').ok_or("Card number not found.")?;
        let (wins, nums) = rest.split_once('|').ok_or("Vertical bar not found.")?;

        let mut winners = [0; 10];
        let mut numbers = [0; 25];
        let mut i = 0;

        for win in wins.split_whitespace() {
            if i >= 10 {
                return Err("Too many winning numbers.".into());
            }

            winners[i] = win.parse()?;
            i += 1;
        }

        if i != 10 {
            return Err("Not enough winning numbers".into());
        }

        i = 0;
        for num in nums.split_whitespace() {
            if i >= 25 {
                return Err("Too many numbers on card.".into());
            }

            numbers[i] = num.parse()?;
            i += 1;
        }

        if i != 25 {
            return Err("Not enough numbers on card".into());
        }

        Ok(Card { winners, numbers })
    }
}

fn find<const S: usize, T: Eq>(arr: &[T; S], elem: &T) -> Option<usize> {
    for i in 0..S {
        if &arr[i] == elem {
            return Some(i);
        }
    }
    None
}

pub fn main() -> Result<(), utils::Error> {
    let cards: Vec<Card> = utils::read_lines(4)?
        .map(|line| line.parse().expect("Failed to parse card."))
        .collect();
    let mut indices: Vec<usize> = (0..cards.len()).collect();
    let mut count = 0;

    while let Some(id) = indices.pop() {
        count += 1;
        let card = &cards[id];
        let matches = card
            .winners
            .iter()
            .filter(|n| find(&card.numbers, *n).is_some())
            .count();

        if matches > 0 {
            indices.extend(id + 1..=id + matches);
        }
    }

    println!("Day 4, Answer 2: {count}");

    Ok(())
}

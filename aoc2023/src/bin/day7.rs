use aoc2023::read_input_lines;

use std::{cmp::Ordering, collections::HashMap};

fn card_from_char(c: char) -> u8 {
    match c {
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as u8,
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Hand {
    cards: Vec<u8>,
    type_score: u8,
}

impl Hand {
    fn parse<S: AsRef<str>>(input: S) -> Self {
        let cards = input
            .as_ref()
            .chars()
            .map(card_from_char)
            .collect::<Vec<_>>();

        let cards_by_value =
            cards
                .iter()
                .fold(HashMap::new(), |mut acc: HashMap<u8, u32>, card| {
                    *acc.entry(*card).or_insert(0) += 1;
                    acc
                });

        let type_score = match cards_by_value.len() {
            1 => 7, // Five of a kind
            2 => {
                if cards_by_value.values().any(|&v| v == 4) {
                    6 // Four of a kind
                } else {
                    5 // Full house
                }
            }
            3 => {
                if cards_by_value.values().any(|&v| v == 3) {
                    4 // Three of a kind
                } else {
                    3 // Two pairs
                }
            }
            4 => 2, // One pair
            _ => 1, // High card
        };
        Self { cards, type_score }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.type_score.cmp(&other.type_score) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for i in 0..self.cards.len() {
                    match self.cards[i].cmp(&other.cards[i]) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => continue,
                    }
                }

                unreachable!("Two hands cannot be equal")
            }
        }
    }
}

#[derive(Debug)]
struct Bet {
    bet: usize,
    hand: Hand,
}

impl Bet {
    fn parse<S: AsRef<str>>(input: S) -> Self {
        let mut parts = input.as_ref().split(' ');
        let hand = Hand::parse(parts.next().unwrap().trim());
        let bet = parts.next().unwrap().trim().parse::<usize>().unwrap();
        Self { bet, hand }
    }
}

fn main() {
    let mut bets = read_input_lines()
        .iter()
        .map(Bet::parse)
        .collect::<Vec<_>>();
    bets.sort_by(|a, b| a.hand.cmp(&b.hand));
    let res = bets
        .iter()
        .enumerate()
        .map(|(i, bet)| bet.bet * (i + 1))
        .sum::<usize>();
    println!("{}", res);
}

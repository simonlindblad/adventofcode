use aoc2023::read_input_lines;

use std::{cmp::Ordering, collections::HashMap};

fn card_from_char(c: char, with_joker: bool) -> u8 {
    match c {
        'J' => {
            if with_joker {
                1
            } else {
                11
            }
        }
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
    fn parse<S: AsRef<str>>(input: S, with_joker: bool) -> Self {
        let cards = input
            .as_ref()
            .chars()
            .map(|c| card_from_char(c, with_joker))
            .collect::<Vec<_>>();

        let mut cards_by_value =
            cards
                .iter()
                .fold(HashMap::new(), |mut acc: HashMap<u8, u32>, card| {
                    *acc.entry(*card).or_insert(0) += 1;
                    acc
                });

        // If we have the joker, upgrade the most common card
        if with_joker {
            if let Some(joker_count) = cards_by_value.get(&1) {
                let mut max_count = 0;
                let mut max_card = 0;
                for (&card, &count) in cards_by_value.iter() {
                    if card != 1 && count > max_count {
                        max_count = count;
                        max_card = card;
                    }
                }

                *cards_by_value.entry(max_card).or_insert(0) += *joker_count;
                cards_by_value.remove(&1);
            }
        }

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
    fn parse<S: AsRef<str>>(input: S, with_joker: bool) -> Self {
        let mut parts = input.as_ref().split(' ');
        let hand = Hand::parse(parts.next().unwrap().trim(), with_joker);
        let bet = parts.next().unwrap().trim().parse::<usize>().unwrap();
        Self { bet, hand }
    }
}

fn find_score(with_joker: bool) -> usize {
    let mut bets = read_input_lines()
        .iter()
        .map(|l| Bet::parse(l, with_joker))
        .collect::<Vec<_>>();
    bets.sort_by(|a, b| a.hand.cmp(&b.hand));
    bets.iter()
        .enumerate()
        .map(|(i, bet)| bet.bet * (i + 1))
        .sum::<usize>()
}

fn main() {
    println!("Part 1: {}", find_score(false));
    println!("Part 2: {}", find_score(true));
}

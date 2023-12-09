use aoc2023::read_input_lines;

use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<usize>,
    scratched_numbers: HashSet<usize>,
}

impl Card {
    fn parse<S: AsRef<str>>(line: S) -> Self {
        let (_id, numbers) = line
            .as_ref()
            .strip_prefix("Card ")
            .unwrap()
            .trim()
            .split_once(": ")
            .unwrap();

        let (winning_numbers_raw, scratched_numbers_raw) = numbers.split_once(" | ").unwrap();
        let winning_numbers = winning_numbers_raw
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let scratched_numbers = scratched_numbers_raw
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        Self {
            winning_numbers,
            scratched_numbers,
        }
    }

    fn win_count(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|n| self.scratched_numbers.contains(n))
            .count()
    }
}

fn part1() {
    let total = read_input_lines()
        .iter()
        .map(Card::parse)
        .map(|c| c.win_count())
        .filter(|&c| c > 0)
        .map(|c| 2usize.pow(c as u32 - 1))
        .sum::<usize>();
    println!("Total points: {:?}", total);
}

fn part2() {
    let cards = read_input_lines()
        .iter()
        .map(Card::parse)
        .collect::<Vec<_>>();

    let mut card_counts = vec![1; cards.len()];
    cards.iter().enumerate().for_each(|(pos, card)| {
        for point in 1..=card.win_count() {
            card_counts[pos + point] += card_counts[pos];
        }
    });

    let total_cards = card_counts.iter().sum::<usize>();

    println!("Total cards: {:?}", total_cards);
}

fn main() {
    part1();

    part2();
}

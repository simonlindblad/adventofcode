use std::fs::read_to_string;
use std::collections::HashSet;

use crate::BatchedIteratorExt;

fn get_point(c: char) -> u32 {
    let value = c as u32;
    if c.is_uppercase() {
        value - 38 // Starts at 65, we want it to start at 27
    } else {
        value - 96 // Starts at 97, we want it to start at 1
    }
}

fn find_common_item(backpacks: &[&str]) -> char {
    backpacks.iter()
        .map(|b| HashSet::<char>::from_iter(b.chars()))
        .reduce(|accum, e| {
            accum.intersection(&e).copied().collect()
        })
        .expect("No backpack provided")
        .into_iter().next().expect("No items in common")
}

fn part1(file: &str) -> u32 {
    read_to_string(file).unwrap()
        .lines()
        .map(|l| l.split_at(l.len()/2))
        .map(|b| find_common_item(&[b.0, b.1]))
        .map(get_point)
        .sum::<u32>()
}

fn part2(file: &str) -> u32 {
    read_to_string(file).unwrap()
        .lines()
        .batch(3)
        .map(|b| find_common_item(&b))
        .map(get_point)
        .sum()
}

pub fn run(file: &str) {
    println!("Part 1: {}", part1(file));
    println!("Part 2: {}", part2(file));
}

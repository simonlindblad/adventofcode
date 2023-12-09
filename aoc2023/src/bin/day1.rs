use std::collections::HashMap;

use aoc2023::{map, read_input_lines};
use lazy_static::lazy_static;

lazy_static! {
    static ref WORDS: HashMap<&'static str, i32> = map! {
        "one" => 1,
        "1" => 1,
        "two" => 2,
        "2" => 2,
        "three" => 3,
        "3" => 3,
        "four" => 4,
        "4" => 4,
        "five" => 5,
        "5" => 5,
        "six" => 6,
        "6" => 6,
        "seven" => 7,
        "7" => 7,
        "eight" => 8,
        "8" => 8,
        "nine" => 9,
        "9" => 9
    };
}

fn extract_number(line: String) -> i32 {
    let first = WORDS
        .keys()
        .min_by_key(|word| line.find(**word).unwrap_or(usize::MAX))
        .map(|word| WORDS[word]);

    // Need to map to i32 to get a negative "wrong" base value
    let last = WORDS
        .keys()
        .max_by_key(|word| line.rfind(**word).map(|u| u as i32).unwrap_or(i32::MIN))
        .map(|word| WORDS[word]);

    format!("{}{}", first.unwrap(), last.unwrap())
        .parse::<i32>()
        .unwrap()
}

fn main() {
    let total = read_input_lines()
        .into_iter()
        .map(extract_number)
        .sum::<i32>();
    println!("{}", total);
}

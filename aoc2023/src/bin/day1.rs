use std::collections::HashMap;

use aoc2023::{map, read_input_file};
use lazy_static::lazy_static;

lazy_static! {
    static ref WORDS_TO_DIGITS: HashMap<String, char> = map! {
        String::from("one") => '1',
        String::from("two") => '2',
        String::from("three") => '3',
        String::from("four") => '4',
        String::from("five") => '5',
        String::from("six") => '6',
        String::from("seven") => '7',
        String::from("eight") => '8',
        String::from("nine") => '9'
    };
}

fn extract_number(line: String) -> i32 {
    let mut first = line.find(|c: char| c.is_ascii_digit());
    let mut first_digit = line.chars().nth(first.unwrap_or_default()).unwrap_or('0');
    for digit in WORDS_TO_DIGITS.keys().clone() {
        if let Some(position) = line.find(digit) {
            if first.is_none() || position < first.unwrap() {
                first = Some(position);
                first_digit = *WORDS_TO_DIGITS.get(digit).unwrap();
            }
        }
    }

    let mut last = line.rfind(|c: char| c.is_ascii_digit());
    let mut last_digit = line.chars().nth(last.unwrap_or_default()).unwrap_or('0');
    for digit in WORDS_TO_DIGITS.keys().clone() {
        if let Some(position) = line.rfind(digit) {
            if last.is_none() || position > last.unwrap() {
                last = Some(position);
                last_digit = *WORDS_TO_DIGITS.get(digit).unwrap();
            }
        }
    }

    format!("{}{}", first_digit, last_digit)
        .parse::<i32>()
        .unwrap()
}

fn main() {
    let total = read_input_file()
        .into_iter()
        .map(extract_number)
        .sum::<i32>();
    println!("{}", total);
}

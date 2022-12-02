use std::fs::read_to_string;

// No need to be fancy - we can just use a lookup table
fn score_part2(line: &str) -> i32 {
    match line {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        l => panic!("Invalid input: {}", l)
    }
}

// No need to be fancy - we can just use a lookup table
fn score_part1(line: &str) -> i32 {
    match line {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        l => panic!("Invalid input: {}", l)
    }
}

fn total_score(file: &str, scoring: fn(&str) -> i32) -> i32 {
    read_to_string(file).unwrap()
        .lines()
        .map(scoring)
        .sum()
}

pub fn run(file: &str) {
    println!("Part 1: {}", total_score(file, score_part1));
    println!("Part 2: {}", total_score(file, score_part2));
}

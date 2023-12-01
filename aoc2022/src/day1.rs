use std::fs::read_to_string;

pub fn sum_top_n(file: &str, n: usize) -> i32 {
    let mut elves = read_to_string(file)
        .unwrap()
        .split("\n\n")
        .map(|elf| elf.lines().map(|i| i.parse::<i32>().unwrap()).sum::<i32>())
        .collect::<Vec<_>>();
    elves.sort();

    elves.into_iter().rev().take(n).sum::<i32>()
}

pub fn run(file: &str) {
    println!("Part 1: {}", sum_top_n(file, 1));
    println!("Part 2: {}", sum_top_n(file, 3));
}

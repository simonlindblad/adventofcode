use std::collections::HashSet;
use std::fs::read_to_string;

fn find_marker(line: &str, count: usize) -> usize {
    let window_start = line
        .as_bytes()
        .windows(count)
        .position(|w| HashSet::<&u8>::from_iter(w.iter()).len() == count)
        .expect("No marker found");
    window_start + count
}

pub fn run(file: &str) {
    let content = read_to_string(file).unwrap();
    // The actual input is only one line, but this makes validating
    // the example input easier.
    for line in content.lines() {
        println!("Part 1: {}", find_marker(line, 4));
        println!("Part 2: {}", find_marker(line, 14));
    }
}

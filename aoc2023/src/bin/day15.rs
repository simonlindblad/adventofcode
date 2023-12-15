use aoc2023::read_input_content;

fn hash(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, c| ((acc + (c as u32)) * 17) % 256)
}

fn main() {
    let raw = read_input_content();
    let sum = raw
        .split(",")
        .map(|s| s.trim())
        .map(|s| hash(s))
        .sum::<u32>();

    println!("{:?}", sum);
}

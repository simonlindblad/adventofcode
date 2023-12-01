use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    aoc2022::day18::run(&args[1])
}

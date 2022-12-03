use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    adventofcode2022::day3::run(&args[1]);
}

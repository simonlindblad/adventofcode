use std::env;

use adventofcode2022::day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    day1::run(&args[1]);
}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    adventofcode2022::day7::run(&args[1]);
}

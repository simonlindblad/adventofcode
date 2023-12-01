use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    adventofcode::day18::run(&args[1])
}

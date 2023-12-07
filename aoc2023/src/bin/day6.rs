use aoc2023::{read_input_lines, solve_quadratic};

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }
}

#[derive(Debug)]
struct Paper {
    races: Vec<Race>,
}

impl Paper {
    fn parse(input: Vec<String>) -> Self {
        let times = input[0]
            .strip_prefix("Time: ")
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let distances = input[1]
            .strip_prefix("Distance: ")
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        Self {
            races: (0..(times.len()))
                .map(|i| Race::new(times[i], distances[i]))
                .collect(),
        }
    }
}

fn number_of_ways(time: f64, distance: f64) -> u64 {
    let (lower, upper) = solve_quadratic(1.0, -time, distance);
    (upper - 1.0).ceil() as u64 - lower.floor() as u64
}

fn main() {
    let paper = Paper::parse(read_input_lines());
    let res = paper
        .races
        .iter()
        .map(|r| number_of_ways(r.time as f64, r.distance as f64))
        .product::<u64>();

    println!("Result: {}", res);
}

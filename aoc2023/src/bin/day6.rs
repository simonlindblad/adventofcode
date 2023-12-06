use aoc2023::read_input_lines;

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

fn number_of_ways_to_win(time: u64, distance: u64, current_speed: u64) -> u64 {
    if time == 0 {
        0
    } else if distance == 0 {
        time
    } else if current_speed * time > distance {
        1 + number_of_ways_to_win(time - 1, distance, current_speed + 1)
    } else {
        number_of_ways_to_win(time - 1, distance, current_speed + 1)
    }
}

fn part1() {
    let paper = Paper::parse(read_input_lines());
    let res = paper
        .races
        .iter()
        .map(|r| number_of_ways_to_win(r.time, r.distance, 0))
        .product::<u64>();
    println!("Part 1: {}", res);
}

fn main() {
    part1();
}

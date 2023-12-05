use aoc2023::read_input_content;
use std::collections::HashMap;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, len: u64) -> Self {
        Self {
            start,
            end: start + len - 1,
        }
    }

    fn contains(&self, value: u64) -> bool {
        self.start <= value && value <= self.end
    }
}

#[derive(Debug)]
struct RangeMapping {
    source: Range,
    destination: Range,
}

impl RangeMapping {
    fn parse<S: AsRef<str>>(input: S) -> Self {
        let numbers = input
            .as_ref()
            .split(' ')
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        Self {
            destination: Range::new(numbers[0], numbers[2]),
            source: Range::new(numbers[1], numbers[2]),
        }
    }

    fn map(&self, value: u64) -> Option<u64> {
        if self.source.contains(value) {
            let offset = value - self.source.start;
            Some(self.destination.start + offset)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map {
    source: String,
    target: String,
    mappings: Vec<RangeMapping>,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<String, Map>,
}

impl Almanac {
    fn parse(input: String) -> Self {
        let components = input.split("\n\n").collect::<Vec<_>>();
        let seeds = components
            .first()
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let maps = components
            .iter()
            .skip(1)
            .map(|component| {
                let mut lines = component.lines();
                let names: Vec<&str> = lines
                    .next()
                    .unwrap()
                    .split(' ')
                    .nth(0)
                    .unwrap()
                    .split("-to-")
                    .collect();
                let (source, target) = (names[0], names[1]);

                let ranges = lines.map(RangeMapping::parse).collect::<Vec<_>>();
                let map = Map {
                    source: source.to_string(),
                    target: target.to_string(),
                    mappings: ranges,
                };
                (map.source.clone(), map)
            })
            .collect();

        Self { seeds, maps }
    }

    fn get_location(&self, seed: u64) -> u64 {
        let mut value = seed;
        let mut current = "seed";

        while current != "location" {
            let map = self.maps.get(current).unwrap();
            value = map
                .mappings
                .iter()
                .flat_map(|mapping| mapping.map(value))
                .next()
                .unwrap_or(value);
            current = &map.target;
        }
        value
    }
}

fn part1() {
    let almanac = Almanac::parse(read_input_content());

    let result = almanac
        .seeds
        .iter()
        .map(|s| almanac.get_location(*s))
        .min()
        .unwrap();
    println!("Part 1: {}", result);
}

fn part2() {
    let almanac = Almanac::parse(read_input_content());

    let result = almanac
        .seeds
        .chunks_exact(2)
        .flat_map(|chunk| (chunk[0]..=chunk[0] + chunk[1]).collect::<Vec<_>>())
        .map(|s| almanac.get_location(s))
        .min()
        .unwrap();
    println!("Part 2: {}", result);
}

fn main() {
    part1();
    part2();
    //let almanac = Almanac::parse(read_input_content());

    //for map in almanac.maps.iter() {
    //    println!("Name: {}", map.0);
    //    for range in map.1.iter() {
    //        println!("  {:?}", range);
    //    }
    //    println!();
    //}
}

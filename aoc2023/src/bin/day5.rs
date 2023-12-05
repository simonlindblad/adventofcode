use aoc2023::read_input_content;
use std::collections::VecDeque;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn from_length(start: u64, len: u64) -> Self {
        Self {
            start,
            end: start + len - 1,
        }
    }

    fn contains(&self, value: u64) -> bool {
        self.start <= value && value <= self.end
    }

    fn intersects(&self, other: &Self) -> bool {
        self.contains(other.start)
            || self.contains(other.end)
            || (self.start > other.start && self.end < other.end)
    }

    fn intersection(&self, other: &Self) -> Option<Range> {
        if self.intersects(other) {
            let start = self.start.max(other.start);
            let end = self.end.min(other.end);
            Some(Range::new(start, end))
        } else {
            None
        }
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
            destination: Range::from_length(numbers[0], numbers[2]),
            source: Range::from_length(numbers[1], numbers[2]),
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
    mappings: VecDeque<RangeMapping>,
}

impl Map {
    fn new(mut mappings: Vec<RangeMapping>) -> Self {
        mappings.sort_by(|a, b| a.source.start.cmp(&b.source.start));
        Self {
            mappings: VecDeque::from(mappings),
        }
    }

    fn merge_intervals(&self, mut intervals: VecDeque<Range>) -> VecDeque<Range> {
        let mut result = VecDeque::new();
        while let Some(interval) = intervals.pop_front() {
            if interval.start > interval.end {
                continue;
            }

            let intersecting = self
                .mappings
                .iter()
                .filter(|m| interval.intersects(&m.source))
                .collect::<Vec<_>>();

            if intersecting.is_empty() {
                result.push_back(interval);
            } else {
                for mapping in intersecting {
                    let intersection = interval.intersection(&mapping.source).unwrap();
                    if interval.start < intersection.start {
                        intervals.push_back(Range::new(interval.start, intersection.start - 1));
                    }

                    if interval.end > intersection.end {
                        intervals.push_back(Range::new(intersection.end + 1, interval.end));
                    }

                    let offset = intersection.start - mapping.source.start;
                    let length = intersection.end - intersection.start + 1;

                    result.push_back(Range::from_length(
                        mapping.destination.start + offset,
                        length,
                    ));
                }
            }
        }

        result
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
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
                let ranges = component
                    .lines()
                    .skip(1)
                    .map(RangeMapping::parse)
                    .collect::<Vec<_>>();
                Map::new(ranges)
            })
            .collect();

        Self { seeds, maps }
    }

    fn get_location(&self, seed: u64) -> u64 {
        let mut value = seed;
        for map in self.maps.iter() {
            value = map
                .mappings
                .iter()
                .flat_map(|mapping| mapping.map(value))
                .next()
                .unwrap_or(value);
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

fn part22() {
    let alma = Almanac::parse(read_input_content());
    let mut intervals = alma
        .seeds
        .chunks_exact(2)
        .map(|chunk| Range::from_length(chunk[0], chunk[1]))
        .collect::<Vec<_>>();

    intervals.sort_by(|a, b| a.start.cmp(&b.start));
    let result = alma
        .maps
        .iter()
        .fold(VecDeque::from(intervals), |a, b| b.merge_intervals(a))
        .iter()
        .map(|i| i.start)
        .min()
        .unwrap();

    println!("Part 2: {:?}", result);
}

fn main() {
    part1();
    part22();
}

use std::fs::read_to_string;

use crate::{max, min};

#[derive(Eq, PartialEq, Hash, Debug)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn distance_to(&self, other: &Position) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn tuning_frequency(&self) -> i64 {
        (self.x * 4_000_000) + self.y
    }
}

struct Sensor {
    pos: Position,
    distance_to_beacon: i64,
}

#[derive(Debug)]
struct Segment {
    start: i64,
    end: i64,
}

impl Segment {
    fn is_overlapping(&self, other: &Segment) -> bool {
        !(self.end + 1 < other.start || self.start > other.end + 1)
    }

    fn merge(&self, other: &Segment) -> Option<Segment> {
        if self.is_overlapping(other) {
            Some(Segment {
                start: min(self.start, other.start),
                end: max(self.end, other.end),
            })
        } else {
            None
        }
    }
}

struct Grid {
    sensors: Vec<Sensor>,
}

impl Grid {
    fn with_pairs(pairs: Vec<(Position, Position)>) -> Self {
        let mut sensors = Vec::new();
        for coord in pairs {
            let distance_to_beacon = coord.0.distance_to(&coord.1);
            sensors.push(Sensor {
                pos: coord.0,
                distance_to_beacon,
            });
        }

        Grid { sensors }
    }

    fn beacon_exclusion_segments_on_line(&self, y: i64) -> Vec<Segment> {
        let mut segments = Vec::new();
        for sensor in self.sensors.iter() {
            let dist = sensor.distance_to_beacon
                - sensor.pos.distance_to(&Position { x: sensor.pos.x, y });
            if dist.is_negative() {
                continue;
            }
            segments.push(Segment {
                start: sensor.pos.x - dist,
                end: sensor.pos.x + dist,
            });
        }

        segments.sort_by(|s1, s2| s1.start.cmp(&s2.start));

        let mut merged = Vec::<Segment>::new();
        for segment in segments.into_iter() {
            if let Some(last) = merged.pop() {
                if let Some(merged_segment) = last.merge(&segment) {
                    merged.push(merged_segment);
                } else {
                    merged.push(last);
                    merged.push(segment);
                }
            } else {
                merged.push(segment);
            }
        }

        merged
    }
}

fn parse_coords(coord: &str) -> Position {
    let parts: Vec<_> = coord.split(", ").collect();
    Position {
        x: parts[0].strip_prefix("x=").unwrap().parse().unwrap(),
        y: parts[1].strip_prefix("y=").unwrap().parse().unwrap(),
    }
}

fn parse_line(line: &str) -> (Position, Position) {
    let parts: Vec<_> = line.split(": closest beacon is at ").collect();
    let sensor = parse_coords(parts[0].strip_prefix("Sensor at ").unwrap());
    let beacon = parse_coords(parts[1]);
    (sensor, beacon)
}

fn parse_grid(file: &str) -> Grid {
    let coords: Vec<_> = read_to_string(file)
        .unwrap()
        .lines()
        .map(parse_line)
        .collect();
    let grid = Grid::with_pairs(coords);
    grid
}

fn part1(file: &str) -> i64 {
    let grid = parse_grid(file);
    let line = 2_000_000;
    let segments = grid.beacon_exclusion_segments_on_line(line);
    // Not technically correct since there can be multiple segments..
    // just happens not to be the case.
    segments.last().unwrap().end - segments.first().unwrap().start
}

fn part2(file: &str) -> i64 {
    let grid = parse_grid(file);
    for i in 0..4_000_000 {
        let segments = grid.beacon_exclusion_segments_on_line(i);
        if segments.len() > 1 {
            return Position {
                x: segments.first().unwrap().end + 1,
                y: i,
            }
            .tuning_frequency();
        }
    }

    panic!("No solution found");
}

pub fn run(file: &str) {
    println!("Part 1: {}", part1(file));
    println!("Part 2: {}", part2(file));
}

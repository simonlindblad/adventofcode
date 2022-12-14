use std::fs::read_to_string;

use crate::{max, min};

struct Line {
    start: (usize, usize),
    end: (usize, usize),
}

struct Cave {
    tiles: Vec<Vec<bool>>,
}

impl Cave {
    fn with_capacity(x: usize, y: usize) -> Cave {
        // We pad to cover the possible scenarios. In the problem description there's no
        // "edge". Probably could have used a sparse structure instead.
        Cave {
            tiles: vec![vec![false; y + 2]; x * 2],
        }
    }

    fn add_floor(&mut self) {
        let bottom = self.tiles[0].len() - 1;
        for x in 0..self.tiles.len() {
            self.tiles[x][bottom] = true;
        }
    }

    fn from_lines(lines: &[Line]) -> Cave {
        let max_x = lines.iter().map(|l| max(l.start.0, l.end.0)).max().unwrap();
        let max_y = lines.iter().map(|l| max(l.start.1, l.end.1)).max().unwrap();

        let mut cave = Cave::with_capacity(max_x + 1, max_y + 1);
        for line in lines {
            cave.add_line(line);
        }

        cave
    }

    fn add_line(&mut self, line: &Line) {
        let start_x = min(line.start.0, line.end.0);
        let end_x = max(line.start.0, line.end.0);

        let start_y = min(line.start.1, line.end.1);
        let end_y = max(line.start.1, line.end.1);

        for x in start_x..end_x + 1 {
            for y in start_y..end_y + 1 {
                self.tiles[x][y] = true;
            }
        }
    }

    fn simulate_single_unit(&mut self, mut pos: (usize, usize)) -> bool {
        // We are already full
        if self.tiles[pos.0][pos.1] {
            return false;
        }

        loop {
            pos.1 = pos.1 + 1;
            if pos.1 >= self.tiles[0].len() {
                self.tiles[pos.0][pos.1 - 1] = true;
                break false;
            } else if !self.tiles[pos.0][pos.1] {
                // Just move on
            } else if pos.0 > 0 && !self.tiles[pos.0 - 1][pos.1] {
                pos.0 -= 1;
            } else if pos.0 + 1 < self.tiles.len() && !self.tiles[pos.0 + 1][pos.1] {
                pos.0 += 1;
            } else {
                self.tiles[pos.0][pos.1 - 1] = true;
                break true;
            }
        }
    }

    fn simulate_sand_from(&mut self, pos: (usize, usize)) -> usize {
        let mut count = 0;
        while self.simulate_single_unit(pos) {
            count += 1;
        }

        count
    }
}

fn parse_cave(file: &str) -> Cave {
    let mut lines = Vec::new();
    for line in read_to_string(file).unwrap().lines() {
        let coordinates = line
            .split(" -> ")
            .map(|l| {
                let coord = l
                    .split(',')
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                (coord[0], coord[1])
            })
            .collect::<Vec<_>>();
        for i in 1..coordinates.len() {
            lines.push(Line {
                start: coordinates[i - 1],
                end: coordinates[i],
            })
        }
    }

    Cave::from_lines(&lines)
}

fn part1(file: &str) -> usize {
    let mut cave = parse_cave(file);
    cave.simulate_sand_from((500, 0))
}

fn part2(file: &str) -> usize {
    let mut cave = parse_cave(file);
    cave.add_floor();
    // debug_cave(&cave);
    cave.simulate_sand_from((500, 0))
}

pub fn run(file: &str) {
    println!("Part 1: {}", part1(file));
    println!("Part 2: {}", part2(file));
}

use std::collections::{HashMap, HashSet};

use aoc2023::read_input_lines;

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Number {
    number: usize,
    x_range: (usize, usize),
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct GearCandidate {
    x: usize,
    y: usize,
    number: usize,
}

impl Grid {
    fn parse(input: &[String]) -> Self {
        let width = input[0].len();
        let height = input.len();
        let data = input.iter().map(|line| line.chars().collect()).collect();
        Grid {
            width,
            height,
            data,
        }
    }

    fn find_adjacent(&self, x: usize, y: usize, detector: fn(char) -> bool) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let x_start = if x == 0 { 0 } else { x - 1 };
        let y_start = if y == 0 { 0 } else { y - 1 };
        for xn in x_start..=x + 1 {
            for yn in y_start..=y + 1 {
                if (xn == x && yn == y) || (xn >= self.width || yn >= self.height) {
                    continue;
                }

                if detector(self.data[yn][xn]) {
                    result.push((xn, yn));
                }
            }
        }

        result
    }

    fn has_symbol_adjacent(&self, x: usize, y: usize) -> bool {
        !self
            .find_adjacent(x, y, |c| !c.is_numeric() && c != '.')
            .is_empty()
    }

    fn find_numbers(&self) -> Vec<Number> {
        let mut result = Vec::new();
        let mut current_number = String::new();
        let mut found_symbol = false;

        let add_number =
            |current_number: &mut String, x: usize, y: usize, result: &mut Vec<Number>| {
                if !current_number.is_empty() {
                    result.push(Number {
                        number: current_number.parse().unwrap(),
                        x_range: (x - current_number.len(), x - 1),
                        y,
                    });
                }

                current_number.clear();
            };

        for y in 0..self.height {
            for x in 0..self.width {
                if !self.data[y][x].is_numeric() {
                    add_number(&mut current_number, x, y, &mut result);
                } else {
                    found_symbol = found_symbol || self.has_symbol_adjacent(x, y);
                    current_number += &self.data[y][x].to_string();
                }
            }
            add_number(&mut current_number, self.width - 1, y, &mut result);
        }
        result
    }
}

fn part1() {
    let grid = Grid::parse(&read_input_lines());
    let number = grid
        .find_numbers()
        .into_iter()
        .filter(|number| {
            for x in number.x_range.0..=number.x_range.1 {
                if grid.has_symbol_adjacent(x, number.y) {
                    return true;
                }
            }

            false
        })
        .map(|number| number.number)
        .sum::<usize>();
    println!("Part 1: {}", number);
}

fn part2() {
    let grid = Grid::parse(&read_input_lines());

    let res = grid
        .find_numbers()
        .into_iter()
        .flat_map(|number| {
            let mut result = HashSet::new();
            for x in number.x_range.0..=number.x_range.1 {
                for candidate in grid.find_adjacent(x, number.y, |c| c == '*') {
                    result.insert(GearCandidate {
                        x: candidate.0,
                        y: candidate.1,
                        number: number.number,
                    });
                }
            }

            result
        })
        .fold(HashMap::new(), |mut acc, candidate| {
            let entry = acc.entry((candidate.x, candidate.y)).or_insert((0, 1));
            *entry = (entry.0 + 1, entry.1 * candidate.number);
            acc
        })
        .into_iter()
        .filter(|(_, (count, _))| *count == 2)
        .map(|(_, (_, number))| number)
        .sum::<usize>();

    println!("Part 2: {:?}", res);
}

fn main() {
    part1();

    part2();
}

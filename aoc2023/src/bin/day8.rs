use std::collections::HashMap;

use aoc2023::{lcm, read_input_lines};

#[derive(Debug)]
struct Document {
    moves: Vec<char>,
    // Node => left, right
    nodes: HashMap<String, (String, String)>,
}

impl Document {
    fn parse(input: Vec<String>) -> Self {
        let moves = input.first().unwrap().chars().collect::<Vec<_>>();
        let nodes = input
            .iter()
            .skip(2)
            .map(|line| {
                let mut parts = line.split(" = ");
                let key = parts.next().unwrap().to_string();
                let (left, right) = parts
                    .next()
                    .unwrap()
                    .strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .split_once(", ")
                    .unwrap();
                (key, (left.to_string(), right.to_string()))
            })
            .collect::<HashMap<_, _>>();
        Document { moves, nodes }
    }

    fn next_node(&self, current: &str, number_of_moves: u64) -> &str {
        let idx = number_of_moves % (self.moves.len() as u64);
        match self.moves[idx as usize] {
            'L' => &self.nodes[current].0,
            'R' => &self.nodes[current].1,
            _ => panic!("Invalid move"),
        }
    }

    fn moves_to_navigate<'a>(&'a self, mut current: &'a str) -> u64 {
        let mut moves = 0;
        while !current.ends_with('Z') {
            current = self.next_node(current, moves);
            moves += 1;
        }

        moves
    }
}

pub fn part1() {
    let document = Document::parse(read_input_lines());
    println!("{}", document.moves_to_navigate("AAA"));
}

pub fn part2() {
    let document = Document::parse(read_input_lines());
    let moves = document
        .nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| document.moves_to_navigate(k))
        .reduce(lcm)
        .unwrap();
    println!("{:?}", moves);
}

fn main() {
    part1();
    part2();
}

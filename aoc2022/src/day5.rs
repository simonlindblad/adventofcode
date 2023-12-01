use std::collections::VecDeque;
use std::fs::read_to_string;

#[derive(Debug)]
struct Move {
    from_crate: usize,
    to_crate: usize,
    count: usize,
}

#[derive(Debug)]
struct Ship {
    stacks: Vec<VecDeque<char>>,
}

impl Ship {
    fn with_capacity(capacity: usize) -> Self {
        Ship {
            stacks: (0..capacity).map(|_| VecDeque::<char>::new()).collect(),
        }
    }

    fn add_to_bottom(&mut self, stack: usize, c: char) {
        self.stacks[stack].push_front(c)
    }

    fn move_crate(&mut self, crate_move: Move) {
        let range = (self.stacks[crate_move.from_crate - 1].len() - crate_move.count)..;
        let crates: Vec<_> = self.stacks[crate_move.from_crate - 1]
            .drain(range)
            .collect();
        self.stacks[crate_move.to_crate - 1].extend(crates);
    }

    fn top_crates(&self) -> String {
        self.stacks
            .iter()
            .map(|s| s.back().unwrap_or(&' '))
            .collect()
    }
}

fn parse_stacks(content: &str) -> Ship {
    let capacity = (content.lines().next().unwrap().len() + 1) / 4;
    let mut ship = Ship::with_capacity(capacity);

    content
        .lines()
        .take_while(|l| l.chars().nth(1) != Some('1'))
        .for_each(|l| {
            l.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| c.is_alphabetic())
                .for_each(|(i, c)| ship.add_to_bottom(i, c));
        });
    ship
}

fn parse_moves(content: &str) -> impl Iterator<Item = Move> + '_ {
    content
        .lines()
        .skip_while(|l| !matches!(l.chars().next(), Some('m')))
        .map(|l| {
            let parts = l.split(' ').collect::<Vec<_>>();
            Move {
                count: parts[1].parse().unwrap(),
                from_crate: parts[3].parse().unwrap(),
                to_crate: parts[5].parse().unwrap(),
            }
        })
}

pub fn part1(content: &str) -> String {
    let mut ship = parse_stacks(content);
    parse_moves(content).for_each(|m| {
        // Do a single move
        (0..m.count).for_each(|_| {
            let single_move = Move {
                from_crate: m.from_crate,
                to_crate: m.to_crate,
                count: 1,
            };
            ship.move_crate(single_move);
        })
    });

    ship.top_crates()
}

fn part2(content: &str) -> String {
    let mut ship = parse_stacks(content);
    parse_moves(content).for_each(|m| ship.move_crate(m));
    ship.top_crates()
}

pub fn run(file: &str) {
    let content = read_to_string(file).unwrap();
    println!("Part 1: {}", part1(&content));
    println!("Part 2: {}", part2(&content));
}

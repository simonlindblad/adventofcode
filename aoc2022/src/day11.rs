use std::collections::VecDeque;
use std::fs::read_to_string;

enum Operation {
    Add(Option<u128>),
    Multiply(Option<u128>),
}

impl Operation {
    fn execute(&self, other: u128) -> u128 {
        // println!("Multiplying: {}", other);
        match self {
            Operation::Add(Some(i)) => other + i,
            Operation::Add(None) => other + other,
            Operation::Multiply(Some(i)) => other * i,
            Operation::Multiply(None) => other * other,
        }
    }
}

struct Monkey {
    items: VecDeque<u128>,
    operation: Operation,
    divisible_test: u128,
    // If true, false
    targets: (usize, usize),
    throws: usize,
}

impl Monkey {
    fn throw(&mut self, worry_factor: u128, common_divisible: u128) -> Option<(usize, u128)> {
        if let Some(item) = self.items.pop_front() {
            self.throws += 1;
            let item = (self.operation.execute(item) / worry_factor) % common_divisible;
            if item % self.divisible_test == 0 {
                Some((self.targets.0, item))
            } else {
                Some((self.targets.1, item))
            }
        } else {
            None
        }
    }

    fn add(&mut self, item: u128) {
        self.items.push_back(item);
    }
}

fn parse_starting_items(line: &str) -> VecDeque<u128> {
    line.strip_prefix("  Starting items: ")
        .unwrap()
        .split(", ")
        .map(|s| s.parse::<u128>().unwrap())
        .collect()
}

fn parse_operation(line: &str) -> Operation {
    let parts: Vec<_> = line
        .strip_prefix("  Operation: new = ")
        .unwrap()
        .split(' ')
        .collect();

    match (parts[0], parts[1], parts[2]) {
        ("old", "+", "old") => Operation::Add(None),
        ("old", "*", "old") => Operation::Multiply(None),
        ("old", "+", item) => Operation::Add(Some(item.parse::<u128>().unwrap())),
        ("old", "*", item) => Operation::Multiply(Some(item.parse::<u128>().unwrap())),
        _ => panic!("Unsupported operation"),
    }
}

fn parse_monkey(content: &str) -> Monkey {
    let lines = content.lines().collect::<Vec<_>>();
    let items = parse_starting_items(lines[1]);
    let operation = parse_operation(lines[2]);
    let divisible_test = lines[3]
        .strip_prefix("  Test: divisible by ")
        .unwrap()
        .parse::<u128>()
        .unwrap();
    let targets = (
        lines[4]
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap(),
        lines[5]
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap(),
    );

    Monkey {
        items,
        operation,
        divisible_test,
        targets,
        throws: 0,
    }
}

fn parse_monkeys(file: &str) -> Vec<Monkey> {
    read_to_string(file)
        .unwrap()
        .split("\n\n")
        .map(|m| parse_monkey(m))
        .collect()
}

fn run_rounds(mut monkeys: Vec<Monkey>, rounds: u128, worry_factor: u128, prod: u128) -> usize {
    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();
            let mut targets = Vec::new();
            while let Some(target) = monkey.throw(worry_factor, prod) {
                targets.push(target);
            }

            for target in targets.iter() {
                let monkey = monkeys.get_mut(target.0).unwrap();
                monkey.add(target.1);
            }
        }
    }

    let mut throws = monkeys.iter().map(|m| m.throws).collect::<Vec<_>>();
    throws.sort();
    let num_monkeys = monkeys.len();
    throws[num_monkeys - 2] * throws[num_monkeys - 1]
}

pub fn run(file: &str) {
    let monkeys = parse_monkeys(file);
    let prod = monkeys.iter().map(|m| m.divisible_test).product();
    println!("Part 1: {}", run_rounds(monkeys, 20, 3, prod));

    let monkeys = parse_monkeys(file);
    println!("Part 2: {}", run_rounds(monkeys, 10000, 1, prod))
}

use std::collections::HashMap;

use aoc2023::{read_input_lines, repeat};

fn search(
    dp: &mut HashMap<(usize, u64, usize), u64>,
    line: &[char],
    current: u64,
    blocks: &[u64],
) -> u64 {
    if line.is_empty() {
        return match (current, blocks.len() as u64) {
            (0, 0) => 1,
            (x, 1) if x == blocks[0] => 1,
            _ => 0,
        };
    }

    if current > 0 && blocks.is_empty() {
        return 0;
    }

    let cache_key = (line.len(), current, blocks.len());
    if let Some(&v) = dp.get(&cache_key) {
        return v;
    }

    let res = match (line[0], current) {
        ('.', 0) => search(dp, &line[1..], 0, blocks),
        ('.', x) if x == blocks[0] => search(dp, &line[1..], 0, &blocks[1..]),
        ('.', _) => 0,
        ('#', x) => search(dp, &line[1..], x + 1, blocks),
        ('?', 0) => search(dp, &line[1..], 1, blocks) + search(dp, &line[1..], 0, blocks),
        ('?', x) if x == blocks[0] => search(dp, &line[1..], 0, &blocks[1..]),
        ('?', x) => search(dp, &line[1..], x + 1, blocks),
        _ => unreachable!(),
    };

    dp.insert(cache_key, res);
    res
}

fn parse<S: AsRef<str>>(line: S) -> (Vec<char>, Vec<u64>) {
    let (line, blocks) = line.as_ref().split_once(' ').unwrap();
    let line = line.chars().collect::<Vec<_>>();
    let blocks = blocks
        .split(',')
        .map(|i| i.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    (line, blocks)
}

fn main() {
    println!(
        "Part 1: {}",
        read_input_lines()
            .iter()
            .map(parse)
            .map(|(line, blocks)| {
                let mut dp = HashMap::new();
                search(&mut dp, &line, 0, &blocks)
            })
            .sum::<u64>()
    );

    println!(
        "Part 2: {}",
        read_input_lines()
            .iter()
            .map(parse)
            .map(|(mut line, blocks)| {
                // Nasty... should have kept it as a string
                line.push('?');
                line = repeat(line, 5);
                line.pop().unwrap();
                (line, repeat(blocks, 5))
            })
            .map(|(line, blocks)| {
                let mut dp = HashMap::new();
                search(&mut dp, &line, 0, &blocks)
            })
            .sum::<u64>()
    );
}

use std::fs::read_to_string;

fn parse_trees(file: &str) -> Vec<Vec<u32>> {
    let content = read_to_string(file).unwrap();
    if content.is_empty() {
        println!("0");
    }

    content
        .lines()
        .map(|l| {
            l.chars()
                .map(|p| p.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn find_max(trees: &[Vec<u32>], iter: impl Iterator<Item = (usize, usize)>) -> u32 {
    iter.map(|(row, col)| trees[row][col]).max().unwrap()
}

fn part1(file: &str) {
    let trees = parse_trees(file);
    let mut visable = trees.len() * 2 + trees[0].len() * 2 - 4;
    let rows = trees.len();
    let columns = trees[0].len();
    for row in 1..rows - 1 {
        for col in 1..columns - 1 {
            let current = trees[row][col];

            if find_max(&trees, (0..col).map(|c| (row, c))) < current || // left
                find_max(&trees, (col+1..columns).map(|c| (row, c))) < current ||  // right
                find_max(&trees, (0..row).map(|r| (r, col))) < current || // up
                find_max(&trees, (row+1..rows).map(|r| (r, col))) < current
            {
                // down
                visable += 1;
            }
        }
    }

    println!("Part 1: {}", visable);
}

fn find_visible(
    trees: &[Vec<u32>],
    current: u32,
    total_count: usize,
    iter: impl Iterator<Item = (usize, usize)>,
) -> usize {
    let count = iter
        .take_while(|(row, col)| trees[*row][*col] < current)
        .count();

    // If the end is not visible, add one (we see the tree that we couldn't see past).
    if total_count == count {
        count
    } else {
        count + 1
    }
}

fn part2(file: &str) {
    let trees = parse_trees(file);
    let mut max_score = 0;
    let rows = trees.len();
    let columns = trees[0].len();
    for row in 0..trees.len() {
        for col in 0..trees[0].len() {
            let current = trees[row][col];

            let (left, left_count) = ((0..col).map(|c| (row, c)).rev(), col);
            let (right, right_count) = ((col+1..columns).map(|c| (row, c)), columns-(col+1));
            let (top, top_count) = ((0..row).rev().map(|r| (r, col)), row);
            let (bottom, bottom_count) = ((row+1..rows).map(|r| (r, col)), rows - (row+1));

            let score = find_visible(&trees, current, left_count, left) *
                find_visible(&trees, current, right_count, right) *
                find_visible(&trees, current, top_count, top) *
                find_visible(&trees, current, bottom_count, bottom);
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("Part 2: {}", max_score);
}

pub fn run(file: &str) {
    part1(file);
    part2(file);
}

use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

type Grid = Vec<Vec<char>>;
type Position = (usize, usize);

fn parse_grid(file: &str) -> Grid {
    read_to_string(file)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect()
}

fn neighbors(position: Position, grid: &Grid) -> Vec<Position> {
    let mut result = Vec::new();
    for change in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
        if (position.0 == 0 && change.0 == -1) || (position.1 == 0 && change.1 == -1) {
            continue;
        }

        let new = (
            (position.0 as i32 + change.0) as usize,
            (position.1 as i32 + change.1) as usize,
        );
        if new.0 < grid.len() && new.1 < grid[0].len() {
            result.push(new);
        }
    }

    result
}

fn get_elevation(position: Position, grid: &Grid) -> u32 {
    let elevation = grid[position.0][position.1];
    if elevation == 'E' {
        'z' as u32
    } else if elevation == 'S' {
        'a' as u32
    } else {
        elevation as u32
    }
}

fn bfs(position: Position, grid: &Grid) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((position, 0));

    while let Some(front) = queue.pop_front() {
        let (position, dist) = front;

        if grid[position.0][position.1] == 'E' {
            return Some(dist);
        }

        for nb in neighbors(position, grid) {
            if !visited.contains(&nb)
                && get_elevation(nb, grid) <= get_elevation(position, grid) + 1
            {
                queue.push_back((nb, dist + 1));
                visited.insert(nb);
            }
        }
    }

    None
}

fn find_starts(grid: &Grid, allow_a: bool) -> Vec<Position> {
    let mut starts = Vec::new();
    for i in 0..grid.len() {
        for k in 0..grid[0].len() {
            if grid[i][k] == 'S' || (allow_a && grid[i][k] == 'a') {
                starts.push((i, k));
            }
        }
    }
    starts
}

fn part1(file: &str) -> usize {
    let grid = parse_grid(file);
    let start = find_starts(&grid, false);
    bfs(start[0], &grid).unwrap()
}

fn part2(file: &str) -> usize {
    let grid = parse_grid(file);
    let mut min = grid.len() * grid[0].len();
    // Ideally we'd store the calculations from the first
    // run and reuse those. However, this runs fast enough
    // to solve the problem.
    for start in find_starts(&grid, true) {
        match bfs(start, &grid) {
            Some(n) if n < min => min = n,
            _ => (),
        }
    }
    min
}

pub fn run(file: &str) {
    println!("Part 1: {}", part1(file));
    println!("Part 2: {}", part2(file));
}

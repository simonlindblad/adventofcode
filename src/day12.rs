use std::collections::VecDeque;
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

fn neighbors<'a>(pos: &'a Position, grid: &'a Grid) -> impl Iterator<Item = Position> + 'a {
    [(-1, 0), (1, 0), (0, 1), (0, -1)]
        .iter()
        .filter(|c| (pos.0 != 0 || c.0 != -1) || (pos.1 != 0 || c.1 != -1))
        .map(|c| ((pos.0 as i32 + c.0) as usize, (pos.1 as i32 + c.1) as usize))
        .filter(|p| p.0 < grid.len() && p.1 < grid[0].len())
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

fn bfs(position: Position, grid: &Grid, allow_a: bool) -> Vec<usize> {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    queue.push_back((position, 0));

    let mut results = Vec::new();
    while let Some(front) = queue.pop_front() {
        let (position, dist) = front;

        let label = grid[position.0][position.1];
        if label == 'S' || (label == 'a' && allow_a) {
            results.push(dist);
        }

        for nb in neighbors(&position, grid) {
            if !visited[nb.0][nb.1] && get_elevation(nb, grid) + 1 >= get_elevation(position, grid)
            {
                queue.push_back((nb, dist + 1));
                visited[nb.0][nb.1] = true;
            }
        }
    }

    results
}

fn find_start(grid: &Grid) -> Position {
    for i in 0..grid.len() {
        for k in 0..grid[0].len() {
            if grid[i][k] == 'E' {
                return (i, k);
            }
        }
    }

    panic!("No goal found")
}

fn part1(file: &str) -> usize {
    let grid = parse_grid(file);
    let start = find_start(&grid);
    bfs(start, &grid, false)[0]
}

fn part2(file: &str) -> usize {
    let grid = parse_grid(file);
    // Ideally we'd store the calculations from the first
    // run and reuse those. However, this runs fast enough
    // to solve the problem.
    let start = find_start(&grid);
    bfs(start, &grid, true).into_iter().min().unwrap()
}

pub fn run(file: &str) {
    println!("Part 1: {}", part1(file));
    println!("Part 2: {}", part2(file));
}

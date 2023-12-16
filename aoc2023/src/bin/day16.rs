use std::collections::{HashSet, VecDeque};

use aoc2023::{Coordinate, Direction, Grid};

fn traverse(grid: &Grid, start: (Direction, Coordinate)) -> (HashSet<Coordinate>, Grid) {
    let mut energized = HashSet::new();
    let mut vgrid = Grid::from_size(grid.cols(), grid.rows(), '.');
    let mut visited = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while !queue.is_empty() {
        let (direction, coordinate) = queue.pop_front().unwrap();
        if visited.contains(&(direction, coordinate)) {
            continue;
        }

        visited.insert((direction, coordinate));
        if coordinate.x >= 0
            && coordinate.y >= 0
            && coordinate.x < grid.cols() as i64
            && coordinate.y < grid.rows() as i64
        {
            energized.insert(coordinate);
            vgrid.update(coordinate.x, coordinate.y, '#');
        }

        let next = match coordinate.navigate(
            &direction,
            (grid.cols() - 1) as i64,
            (grid.rows() - 1) as i64,
        ) {
            Some(next) => next,
            None => continue,
        };

        match grid.get(next.x, next.y) {
            '.' => queue.push_back((direction, next)),
            '/' => match direction {
                Direction::Up => queue.push_back((Direction::Right, next)),
                Direction::Down => queue.push_back((Direction::Left, next)),
                Direction::Left => queue.push_back((Direction::Down, next)),
                Direction::Right => queue.push_back((Direction::Up, next)),
            },
            '\\' => match direction {
                Direction::Up => queue.push_back((Direction::Left, next)),
                Direction::Down => queue.push_back((Direction::Right, next)),
                Direction::Left => queue.push_back((Direction::Up, next)),
                Direction::Right => queue.push_back((Direction::Down, next)),
            },
            '|' => match direction {
                Direction::Up | Direction::Down => queue.push_back((direction, next)),
                Direction::Left | Direction::Right => {
                    queue.push_back((Direction::Up, next));
                    queue.push_back((Direction::Down, next));
                }
            },
            '-' => match direction {
                Direction::Left | Direction::Right => queue.push_back((direction, next)),
                Direction::Up | Direction::Down => {
                    queue.push_back((Direction::Left, next));
                    queue.push_back((Direction::Right, next));
                }
            },
            _ => unreachable!(),
        }
    }

    (energized, vgrid)
}

fn main() {
    let grid = Grid::from_input();
    let (energized, _) = traverse(&grid, (Direction::Right, Coordinate::new(-1, 0)));
    println!("Part 1: {}", energized.len());

    let mut res = 0;
    for row in 0..grid.rows() {
        let (energized, _) = traverse(&grid, (Direction::Right, Coordinate::new(-1, row as i64)));
        res = aoc2023::max(res, energized.len());
        let (energized, _) = traverse(
            &grid,
            (
                Direction::Left,
                Coordinate::new(grid.cols() as i64, row as i64),
            ),
        );
        res = aoc2023::max(res, energized.len());
    }

    for col in 0..grid.cols() {
        let (energized, _) = traverse(&grid, (Direction::Down, Coordinate::new(col as i64, -1)));
        res = aoc2023::max(res, energized.len());
        let (energized, _) = traverse(
            &grid,
            (
                Direction::Up,
                Coordinate::new(col as i64, grid.rows() as i64),
            ),
        );
        res = aoc2023::max(res, energized.len());
    }

    println!("Part 2: {}", res);
}

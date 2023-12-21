use std::collections::HashSet;

use aoc2023::{Coordinate, Direction, Grid};

fn find_next_coordinates(grid: &Grid, current: &HashSet<Coordinate>) -> HashSet<Coordinate> {
    let mut result = HashSet::new();
    for c in current {
        for direction in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(next) =
                c.navigate(direction, grid.cols() as i64 - 1, grid.rows() as i64 - 1)
            {
                if grid.get(next.x, next.y) == '.' {
                    result.insert(next);
                }
            }
        }
    }

    result
}

fn main() {
    let mut grid = Grid::from_input();
    let (x, y) = grid.find('S').unwrap();
    grid.update(x as i64, y as i64, '.');
    let iterations = 64;


    let mut current = HashSet::new();
    current.insert(Coordinate::new(x as i64, y as i64));
    for _ in 0..iterations {
        current = find_next_coordinates(&grid, &current);
    }

    println!("Part 1: {}", current.len());
}

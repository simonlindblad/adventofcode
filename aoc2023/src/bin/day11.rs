use std::collections::HashSet;

use aoc2023::{max, min, within_range, Grid};

pub struct Expanded {
    columns: Vec<usize>,
    rows: Vec<usize>,
}

impl Expanded {
    fn from_grid(grid: &Grid) -> Self {
        let mut occupied_columns = HashSet::new();
        let mut occupied_rows = HashSet::new();
        for (column, row) in grid.find_all('#').iter() {
            occupied_columns.insert(*column);
            occupied_rows.insert(*row);
        }

        let mut rows = vec![];
        let mut columns = vec![];
        for row in 0..grid.rows() {
            if !occupied_rows.contains(&row) {
                rows.push(row);
            }
        }

        for column in 0..grid.cols() {
            if !occupied_columns.contains(&column) {
                columns.push(column);
            }
        }

        Expanded { columns, rows }
    }
}

fn solve(expansion_ratio: usize) -> usize {
    let grid = Grid::from_input();
    let expanded = Expanded::from_grid(&grid);

    let galaxies = grid.find_all('#');
    let mut distances = vec![];
    for (x, y) in galaxies.iter() {
        for (x2, y2) in galaxies.iter() {
            if x == x2 && y == y2 {
                continue;
            }
            let mut num_expanded = 0;
            for row in expanded.rows.iter() {
                if within_range(*row as i64, *y as i64, *y2 as i64) {
                    num_expanded += 1;
                }
            }

            for column in expanded.columns.iter() {
                if within_range(*column as i64, *x as i64, *x2 as i64) {
                    num_expanded += 1;
                }
            }

            distances.push(
                max(y2, y) - min(y2, y) + max(x2, x) - min(x2, x)
                    + num_expanded * (expansion_ratio - 1),
            );
        }
    }

    distances.iter().sum::<usize>() / 2
}

fn main() {
    println!("Part 1: {}", solve(2));
    println!("Part 2: {}", solve(1000000));
}

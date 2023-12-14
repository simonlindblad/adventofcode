use std::collections::HashMap;

use aoc2023::Grid;

fn roll_north(grid: &mut Grid) {
    for col in 0..grid.cols() {
        let mut current_top = 0;
        for row in 0..grid.rows() {
            if grid.get(col as i64, row as i64) == 'O' {
                grid.update(col as i64, row as i64, '.');
                grid.update(col as i64, current_top as i64, 'O');
                current_top = current_top + 1;
            } else if grid.get(col as i64, row as i64) == '#' {
                current_top = row + 1;
            }
        }
    }
}

fn roll_west(grid: &mut Grid) {
    for row in 0..grid.rows() {
        let mut current_left = 0;
        for col in 0..grid.cols() {
            if grid.get(col as i64, row as i64) == 'O' {
                grid.update(col as i64, row as i64, '.');
                grid.update(current_left as i64, row as i64, 'O');
                current_left = current_left + 1;
            } else if grid.get(col as i64, row as i64) == '#' {
                current_left = col + 1;
            }
        }
    }
}

fn roll_south(grid: &mut Grid) {
    for col in 0..grid.cols() {
        let mut current_bottom = grid.rows() - 1;
        for row in (0..grid.rows()).rev() {
            if grid.get(col as i64, row as i64) == 'O' {
                grid.update(col as i64, row as i64, '.');
                grid.update(col as i64, current_bottom as i64, 'O');
                if current_bottom > 0 {
                    current_bottom = current_bottom - 1;
                }
            } else if grid.get(col as i64, row as i64) == '#' && row > 0 {
                current_bottom = row - 1;
            }
        }
    }
}

fn roll_east(grid: &mut Grid) {
    for row in 0..grid.rows() {
        let mut current_right = grid.cols() - 1;
        for col in (0..grid.cols()).rev() {
            if grid.get(col as i64, row as i64) == 'O' {
                grid.update(col as i64, row as i64, '.');
                grid.update(current_right as i64, row as i64, 'O');
                if current_right > 0 {
                    current_right = current_right - 1;
                }
            } else if grid.get(col as i64, row as i64) == '#' && col > 0 {
                current_right = col - 1;
            }
        }
    }
}

fn cycle(grid: &mut Grid) {
    roll_north(grid);
    roll_west(grid);
    roll_south(grid);
    roll_east(grid)
}

fn score(grid: &Grid) -> u64 {
    let max_score = grid.rows() as u64;
    let mut score = 0;
    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            if grid.get(j as i64, i as i64) == 'O' {
                score += max_score - i as u64;
            }
        }
    }

    score
}

fn part2() {
    let mut grid = Grid::from_input();
    let mut cache = HashMap::new();
    let cycles = 1000000000;
    for i in 0..cycles {
        cycle(&mut grid);
        if let Some(key) = cache.get(&format!("{}", grid)[..]) {
            let repeat_len = i - key;
            let remaining = ((cycles - i) % repeat_len) - 1;
            for _ in 0..remaining {
                cycle(&mut grid);
            }
            break;
        }

        cache.insert(format!("{}", grid), i);
    }
    println!("{}", score(&grid));
}

fn part1() {
    let mut grid = Grid::from_input();
    roll_north(&mut grid);
    println!("Part 1: {}", score(&grid));
}

fn main() {
    part1();
    part2();
}

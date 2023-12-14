use aoc2023::{min, read_input_content, Grid};

fn row_diffs(idx1: usize, idx2: usize, grid: &Grid) -> usize {
    let mut diffs = 0;
    for i in 0..grid.cols() {
        if grid.get(i as i64, idx1 as i64) != grid.get(i as i64, idx2 as i64) {
            diffs += 1;
        }
    }
    diffs
}

fn find_matches(grid: Grid, allowed_diffs: usize, multiplier: usize) -> Option<usize> {
    for i in 1..grid.rows() {
        let mut diff_buffer = allowed_diffs;
        let diffs = row_diffs(i - 1, i, &grid);
        if diffs <= diff_buffer {
            diff_buffer -= diffs;
            let mut found = true;
            let m = min(i, grid.rows() - i);
            for j in 1..m {
                let diffs = row_diffs(i - j - 1, i + j, &grid);
                if diffs > diff_buffer {
                    found = false;
                    break;
                } else {
                    diff_buffer -= diffs;
                }
            }

            if found && diff_buffer == 0 {
                return Some(i * multiplier);
            }
        }
    }

    None
}

fn main() {
    let input = read_input_content();
    let res = input
        .split("\n\n")
        .map(Grid::from_raw)
        .map(|g| {
            find_matches(g.clone(), 0, 100)
                .or_else(|| find_matches(g.transpose(), 0, 1))
                .unwrap()
        })
        .sum::<usize>();
    println!("Part 1: {}", res);

    let res = input
        .split("\n\n")
        .map(Grid::from_raw)
        .map(|g| {
            let m = find_matches(g.clone(), 1, 100)
                .or_else(|| find_matches(g.transpose(), 1, 1))
                .unwrap();
            m
        })
        .sum::<usize>();
    println!("Part 2: {}", res)
}

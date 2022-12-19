use std::fs::read_to_string;

type Droplet = (usize, usize, usize);
type Grid = Vec<Vec<Vec<State>>>;

fn parse_droplets(file: &str) -> Vec<Droplet> {
    read_to_string(file).unwrap().lines()
        .map(|l| {
            let parts: Vec<_> = l.split(',').collect();
            let droplet: (usize, usize, usize) = (
                parts[0].parse().unwrap(),
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap()
            );
            droplet
        }).collect()
}

fn neighbors(pos: &Droplet) -> impl Iterator<Item = Droplet> + '_ {
    [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)]
        .iter()
        .filter(|c| (pos.0 != 0 || c.0 != -1) && (pos.1 != 0 || c.1 != -1) && (pos.2 != 0 || c.2 != -1))
        .map(|c| ((pos.0 as i32 + c.0) as usize, (pos.1 as i32 + c.1) as usize, (pos.2 as i32 + c.2) as usize))
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    Lava,
    Air,
    Steam,
}

fn flood(grid: &mut Vec<Vec<Vec<State>>>, drop: &Droplet) {
    if drop.0 >= grid.len() || drop.1 >= grid[0].len() || drop.2 >= grid[0][0].len() || grid[drop.0][drop.1][drop.2] != State::Air {
        return;
    }

    grid[drop.0][drop.1][drop.2] = State::Steam;

    for nb in neighbors(drop) {
        flood(grid, &nb);
    }
}

fn count_surfaces_with_state(grid: &Grid, state: State, droplets: &[Droplet]) -> usize {
    droplets.iter().map(|d| {
        let mut count = 0;

        for i in [d.0, d.1, d.2] {
            if i == 0 {
                count += 1;
            }
        }

        for nb in neighbors(d) {
            if grid[nb.0][nb.1][nb.2] == state {
                count += 1;
            }
        }
        count
    }).sum()
}

fn part1(file: &str) {
    let droplets = parse_droplets(file);
    let mut grid = vec![vec![vec![State::Air; 25]; 25]; 25];
    droplets.iter().for_each(|d| grid[d.0][d.1][d.2] = State::Lava);
    println!("Part 1: {}", count_surfaces_with_state(&grid, State::Air, &droplets));
}

fn part2(file: &str) {
    let droplets = parse_droplets(file);
    let mut grid = vec![vec![vec![State::Air; 25]; 25]; 25];
    droplets.iter().for_each(|d| grid[d.0][d.1][d.2] = State::Lava);

    // Flood the air filled bubbles with "Steam"
    for x in 0..25 {
        flood(&mut grid, &(x, 0, 0));
        flood(&mut grid, &(0, x, 0));
        flood(&mut grid, &(0, 0, x));
    }
    println!("Part 1: {}", count_surfaces_with_state(&grid, State::Air, &droplets));
}

pub fn run(file: &str) {
    part1(file);
    part2(file);
}

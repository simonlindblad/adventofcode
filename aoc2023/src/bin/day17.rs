use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc2023::{Coordinate, Direction, Grid};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    coordinate: Coordinate,
    direction: Direction,
    distance: usize,
    num_straight: usize,
}

impl State {
    fn new(
        coordinate: Coordinate,
        direction: Direction,
        distance: usize,
        num_straight: usize,
    ) -> Self {
        Self {
            coordinate,
            direction,
            distance,
            num_straight,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn allowed_directions_crucible(
    num_straight: usize,
    previous_direction: Direction,
) -> Vec<Direction> {
    match (num_straight, previous_direction) {
        (3, Direction::Down | Direction::Up) => vec![Direction::Left, Direction::Right],
        (3, Direction::Left | Direction::Right) => vec![Direction::Up, Direction::Down],
        (_, Direction::Down) => vec![Direction::Down, Direction::Left, Direction::Right],
        (_, Direction::Up) => vec![Direction::Up, Direction::Left, Direction::Right],
        (_, Direction::Left) => vec![Direction::Left, Direction::Up, Direction::Down],
        (_, Direction::Right) => vec![Direction::Right, Direction::Up, Direction::Down],
    }
}

fn allowed_directions_ultra_crucible(
    num_straight: usize,
    previous_direction: Direction,
) -> Vec<Direction> {
    match (num_straight, previous_direction) {
        (10, Direction::Down | Direction::Up) => vec![Direction::Left, Direction::Right],
        (10, Direction::Left | Direction::Right) => vec![Direction::Up, Direction::Down],
        (x, direction) if x < 4 => vec![direction],
        (_, Direction::Down) => vec![Direction::Down, Direction::Left, Direction::Right],
        (_, Direction::Up) => vec![Direction::Up, Direction::Left, Direction::Right],
        (_, Direction::Left) => vec![Direction::Left, Direction::Up, Direction::Down],
        (_, Direction::Right) => vec![Direction::Right, Direction::Up, Direction::Down],
    }
}

fn shortest_path(
    grid: &Grid,
    start: (Direction, Coordinate),
    goal: Coordinate,
    get_allowed_directions: fn(usize, Direction) -> Vec<Direction>,
) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut distances = HashMap::new();

    // Different than regular dijkstra - we need to consider "slower" states if we haven't
    // visited them yet.
    let mut visited = HashSet::new();

    distances.insert(start.1, 0);
    heap.push(State::new(start.1, start.0, 0, 0));

    while let Some(State {
        coordinate,
        direction: previous_direction,
        distance,
        num_straight,
    }) = heap.pop()
    {
        if coordinate == goal {
            return Some(distance);
        }

        let visited_key = (coordinate, previous_direction, num_straight);

        if visited.contains(&visited_key)
            && distance > *distances.get(&coordinate).unwrap_or(&usize::MAX)
        {
            continue;
        }

        visited.insert(visited_key);

        for direction in get_allowed_directions(num_straight, previous_direction) {
            let next = match coordinate.navigate(
                &direction,
                (grid.cols() - 1) as i64,
                (grid.rows() - 1) as i64,
            ) {
                Some(next) => next,
                None => continue,
            };

            let next_distance = distance
                + grid
                    .get(next.x, next.y)
                    .to_string()
                    .parse::<usize>()
                    .unwrap();

            if next_distance < *distances.get(&next).unwrap_or(&usize::MAX) {
                distances.insert(next, next_distance);
            }
            heap.push(State::new(
                next,
                direction,
                next_distance,
                if direction == previous_direction {
                    num_straight + 1
                } else {
                    1
                },
            ));
        }
    }

    None
}

fn main() {
    let grid = Grid::from_input();
    let cost = shortest_path(
        &grid,
        (Direction::Down, Coordinate::new(0, 0)),
        Coordinate::new(grid.cols() as i64 - 1, grid.rows() as i64 - 1),
        allowed_directions_crucible,
    )
    .unwrap();
    println!("Part 1: {}", cost);

    let cost = shortest_path(
        &grid,
        (Direction::Down, Coordinate::new(0, 0)),
        Coordinate::new(grid.cols() as i64 - 1, grid.rows() as i64 - 1),
        allowed_directions_ultra_crucible,
    )
    .unwrap();

    let cost_right = shortest_path(
        &grid,
        (Direction::Right, Coordinate::new(0, 0)),
        Coordinate::new(grid.cols() as i64 - 1, grid.rows() as i64 - 1),
        allowed_directions_ultra_crucible,
    )
    .unwrap();
    println!("Part 2: {}", aoc2023::min(cost, cost_right));
}

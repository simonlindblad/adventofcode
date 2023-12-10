use aoc2023::Grid;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Start,
    North,
    East,
    West,
    South,
}

#[derive(Copy, Clone, Debug)]
struct Position {
    x: i64,
    y: i64,
    came_from: Direction,
}

impl Position {
    fn move_west(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y,
            came_from: Direction::East,
        }
    }

    fn move_east(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
            came_from: Direction::West,
        }
    }

    fn move_north(&self) -> Position {
        Position {
            x: self.x,
            y: self.y - 1,
            came_from: Direction::South,
        }
    }

    fn move_south(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
            came_from: Direction::North,
        }
    }
}

fn search_from(grid: &Grid, start_x: i64, start_y: i64) -> Option<Vec<(i64, i64)>> {
    let mut position = Position {
        came_from: Direction::Start,
        x: start_x,
        y: start_y,
    };
    let mut path = Vec::new();

    loop {
        let (x, y) = (position.x, position.y);

        if !grid.is_valid(x, y) {
            continue;
        }

        if !path.is_empty() && (x, y) == (start_x, start_y) {
            return Some(path);
        }

        path.push((x, y));
        position = match (position.came_from, grid.get(x, y)) {
            (Direction::North, 'L') => position.move_east(),
            (_, 'L') => position.move_north(),
            (Direction::East, 'F') => position.move_south(),
            (_, 'F') => position.move_east(),
            (Direction::South, '|') => position.move_north(),
            (_, '|') => position.move_south(),
            (Direction::East, '-') => position.move_west(),
            (_, '-') => position.move_east(),
            (Direction::North, 'J') => position.move_west(),
            (_, 'J') => position.move_north(),
            (Direction::South, '7') => position.move_west(),
            (_, '7') => position.move_south(),
            _ => return None,
        };
    }
}

fn search(grid: &mut Grid) -> Vec<(i64, i64)> {
    let start = grid.find('S').expect("No start found");
    // Let's search through all possible ones instead of making S a special case
    for n in "LF|-J7".chars() {
        grid.update(start.0 as i64, start.1 as i64, n);
        if let Some(res) = search_from(grid, start.0 as i64, start.1 as i64) {
            return res;
        }
    }
    panic!("No loop");
}

fn main() {
    let mut grid = Grid::from_input();
    let mut path = search(&mut grid);
    println!("Part 1: {}", path.len() / 2);

    // We use the shoelace theorem to calculate 2A
    path.push(path[0]);
    let mut double_area = 0;
    for i in 1..path.len() {
        double_area += path[i - 1].0 * path[i].1 - path[i].0 * path[i - 1].1;
    }

    // Picks gives us the number of points inside the polygon
    println!("Part 2: {}", double_area / 2 - path.len() as i64 / 2 + 1);
}

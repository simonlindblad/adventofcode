use std::{collections::VecDeque, env, path::Path};

fn read_file<P: AsRef<Path>>(path: P) -> Vec<String> {
    let contents = std::fs::read_to_string(path).expect("Something went wrong reading the file");
    contents.lines().map(|s| s.to_string()).collect()
}

pub fn read_input_lines() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    read_file(&args[1])
}

pub fn read_input_content() -> String {
    let args: Vec<String> = env::args().collect();
    std::fs::read_to_string(&args[1]).expect("Something went wrong reading the file")
}

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let sqrt_d = (b * b - 4.0 * a * c).sqrt();
    let denom = 2.0 * a;
    let x1 = (-b - sqrt_d) / denom;
    let x2 = (-b + sqrt_d) / denom;
    (x1, x2)
}

pub fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

#[derive(Debug, Clone)]
pub struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    pub fn from_input() -> Self {
        let grid = read_input_lines()
            .iter()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { grid }
    }

    pub fn from_raw(s: &str) -> Self {
        let grid = s
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { grid }
    }

    pub fn from_size(rows: usize, cols: usize, c: char) -> Self {
        let grid = vec![vec![c; cols]; rows];
        Self { grid }
    }

    pub fn rows(&self) -> usize {
        self.grid.len()
    }

    pub fn cols(&self) -> usize {
        self.grid[0].len()
    }

    pub fn is_valid(&self, x: i64, y: i64) -> bool {
        x < self.grid[0].len() as i64 && y < self.grid.len() as i64 && x >= 0 && y >= 0
    }

    pub fn get(&self, x: i64, y: i64) -> char {
        self.grid[y as usize][x as usize]
    }

    pub fn update(&mut self, x: i64, y: i64, c: char) {
        self.grid[y as usize][x as usize] = c;
    }

    pub fn find(&self, c: char) -> Option<(usize, usize)> {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &c2) in row.iter().enumerate() {
                if c == c2 {
                    return Some((x, y));
                }
            }
        }

        None
    }

    pub fn find_all(&self, c: char) -> Vec<(usize, usize)> {
        let mut res = vec![];
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &c2) in row.iter().enumerate() {
                if c == c2 {
                    res.push((x, y));
                }
            }
        }

        res
    }

    pub fn insert_row(&mut self, index: usize, c: char) {
        let v = vec![c; self.grid[0].len()];
        self.grid.insert(index, v);
    }

    pub fn insert_column(&mut self, index: usize, c: char) {
        for row in self.grid.iter_mut() {
            row.insert(index, c);
        }
    }

    pub fn bfs(&self, starting_point: (usize, usize)) -> Vec<Vec<usize>> {
        let mut visited = vec![vec![usize::MAX; self.cols()]; self.rows()];
        let mut queue = VecDeque::from(vec![(0, starting_point)]);
        while !queue.is_empty() {
            let (distance, (x, y)) = queue.pop_front().unwrap();
            if visited[y][x] == usize::MAX {
                visited[y][x] = distance;
                if self.is_valid(x as i64 - 1, y as i64) {
                    queue.push_back((distance + 1, (x - 1, y)));
                }
                if self.is_valid(x as i64 + 1, y as i64) {
                    queue.push_back((distance + 1, (x + 1, y)));
                }
                if self.is_valid(x as i64, y as i64 - 1) {
                    queue.push_back((distance + 1, (x, y - 1)));
                }
                if self.is_valid(x as i64, y as i64 + 1) {
                    queue.push_back((distance + 1, (x, y + 1)));
                }
            }
        }

        visited
    }

    pub fn transpose(&self) -> Self {
        let mut grid = vec![vec!['.'; self.rows()]; self.cols()];
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                grid[x][y] = c;
            }
        }

        Self { grid }
    }

    pub fn rotate_left(&self) -> Self {
        let mut grid = vec![vec!['.'; self.rows()]; self.cols()];
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                grid[self.rows() - x - 1][y] = c;
            }
        }

        Self { grid }
    }

    pub fn rotate_right(&self) -> Self {
        let mut grid = vec![vec!['.'; self.rows()]; self.cols()];
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                grid[x][self.cols() - y - 1] = c;
            }
        }

        Self { grid }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for c in row.iter() {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.y == other.y {
            self.x.cmp(&other.x)
        } else {
            self.y.cmp(&other.y)
        }
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Coordinate {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn navigate(&self, direction: &Direction, max_x: i64, max_y: i64) -> Option<Self> {
        match (self.x, self.y, direction) {
            (0, _, Direction::Left) => None,
            (_, 0, Direction::Up) => None,
            (_, y, Direction::Down) if y == max_y => None,
            (x, _, Direction::Right) if x == max_x => None,
            (x, y, Direction::Left) => Some(Self { x: x - 1, y }),
            (x, y, Direction::Up) => Some(Self { x, y: y - 1 }),
            (x, y, Direction::Down) => Some(Self { x, y: y + 1 }),
            (x, y, Direction::Right) => Some(Self { x: x + 1, y }),
        }
    }
}

pub fn min<T: PartialOrd>(lhs: T, rhs: T) -> T {
    if lhs < rhs {
        lhs
    } else {
        rhs
    }
}

pub fn max<T: PartialOrd>(lhs: T, rhs: T) -> T {
    if lhs > rhs {
        lhs
    } else {
        rhs
    }
}

pub fn within_range(value: i64, start: i64, end: i64) -> bool {
    value >= min(start, end) && value <= max(start, end)
}

pub fn repeat<T: Clone>(data: Vec<T>, times: usize) -> Vec<T> {
    (0..times).flat_map(|_| data.clone()).collect()
}

/// Macro to construct a HashMap
#[macro_export]
macro_rules! map {
    ($( $key: expr => $val: expr ),*) => {{
         let mut t = ::std::collections::HashMap::new();
         $( t.insert($key, $val); )*
         t
    }}
}

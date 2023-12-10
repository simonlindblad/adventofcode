use std::{env, path::Path};

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

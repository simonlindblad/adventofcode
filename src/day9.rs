use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
enum Move {
    Down,
    Up,
    Left,
    Right,
}

impl Move {
    fn from_letter(letter: &str) -> Self {
        match letter {
            "U" => Move::Up,
            "D" => Move::Down,
            "L" => Move::Left,
            "R" => Move::Right,
            _ => panic!("Invalid move"),
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn are_touching(&self, knot: &Knot) -> bool {
        let (x_diff, y_diff) = self.get_position_diff(knot);
        x_diff.abs() <= 1 && y_diff.abs() <= 1
    }

    fn get_position_diff(&self, knot: &Knot) -> (i32, i32) {
        ((knot.x - self.x), (knot.y - self.y))
    }

    fn move_compared_to(&mut self, knot: &Knot) {
        if !self.are_touching(knot) {
            let (x_diff, y_diff) = self.get_position_diff(knot);
            if x_diff == 0 {
                // Same x - move y closer
                self.y += y_diff.signum();
            } else if y_diff == 0 {
                self.x += x_diff.signum();
            } else {
                self.x += x_diff.signum();
                self.y += y_diff.signum();
            }
        }
    }

    fn move_knot(&mut self, m: &Move) {
        match m {
            Move::Up => self.y += 1,
            Move::Down => self.y -= 1,
            Move::Left => self.x -= 1,
            Move::Right => self.x += 1,
        }
    }
}

struct Rope {
    knots: Vec<Knot>,
    tail_history: HashSet<(i32, i32)>,
}

impl Default for Rope {
    fn default() -> Self {
        Rope::new(2)
    }
}

impl Rope {
    fn new(count: usize) -> Self {
        let mut tail_history = HashSet::new();
        tail_history.insert((0, 0));
        let knots = (0..count).map(|_| Knot::default()).collect::<Vec<_>>();
        Rope {
            knots,
            tail_history,
        }
    }

    fn move_head(&mut self, m: &Move) {
        self.knots[0].move_knot(m);
        for i in 1..self.knots.len() {
            let previous = self.knots[i - 1];
            self.knots[i].move_compared_to(&previous);
        }

        self.save_history();
    }

    fn save_history(&mut self) {
        self.tail_history
            .insert((self.knots.last().unwrap().x, self.knots.last().unwrap().y));
    }

    fn tail_positions(&self) -> usize {
        self.tail_history.len()
    }
}

fn parse_moves(file: &str) -> Vec<Move> {
    let mut result = Vec::new();
    for line in read_to_string(file).unwrap().lines() {
        let (m, count) = line.split_once(' ').unwrap();
        result.extend((0..count.parse().unwrap()).map(|_| Move::from_letter(m)));
    }

    result
}

fn do_moves(moves: &[Move], mut rope: Rope) -> usize {
    moves.iter().for_each(|m| rope.move_head(m));
    rope.tail_positions()
}

pub fn run(file: &str) {
    let moves = parse_moves(file);
    println!("Part 1: {}", do_moves(&moves, Rope::default()));
    println!("Part 2: {}", do_moves(&moves, Rope::new(10)));
}

use std::collections::HashMap;
use std::fs::read_to_string;

use crate::max;

#[derive(Debug)]
enum Push {
    Left,
    Right,
}

impl Push {
    fn parse(inp: char) -> Self {
        match inp {
            '<' => Push::Left,
            '>' => Push::Right,
            e => panic!("Unsupported move: {}", e)
        }
    }
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn from(x: i64, y: i64) -> Self {
        Position { x, y }
    }
}

/// Possible rocks:
///
///####
// 
// .#.
// ###
// .#.
// 
// ..#
// ..#
// ###
// 
// #
// #
// #
// #
// 
// ##
// ##
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    HorizontalLine,
    Plus,
    ReverseL,
    VerticalLine,
    Square,
}

impl Rock {
    fn need_width(&self) -> usize {
        match self {
            Rock::Plus => 1,
            Rock::HorizontalLine => 4,
            Rock::ReverseL => 3,
            Rock::VerticalLine => 1,
            Rock::Square => 2,
        }
    }

    fn start_position(&self, x: i64, y: i64) -> Vec<Position> {
        match self {
            Rock::HorizontalLine => vec![
                Position::from(x, y),
                Position::from(x+1, y),
                Position::from(x+2, y),
                Position::from(x+3, y),
            ],
            Rock::Plus => vec![
                Position::from(x+1, y+2),
                Position::from(x, y+1),
                Position::from(x+1, y+1),
                Position::from(x+2, y+1),
                Position::from(x+1, y),
            ],
            Rock::ReverseL => vec![
                Position::from(x+2, y+2),
                Position::from(x+2, y+1),
                Position::from(x, y),
                Position::from(x+1, y),
                Position::from(x+2, y),
            ],
            Rock::VerticalLine => vec![
                Position::from(x, y+3),
                Position::from(x, y+2),
                Position::from(x, y+1),
                Position::from(x, y),
            ],
            Rock::Square => vec![
                Position::from(x, y+1),
                Position::from(x+1, y+1),
                Position::from(x, y),
                Position::from(x+1, y),
            ],
        }
    }
}

struct Pushes {
    pushes: Vec<Push>,
    current: usize,
}

impl Pushes {
    fn new(pushes: Vec<Push>) -> Self {
        Pushes { pushes, current: 0 }
    }

    fn get_next(&mut self) -> &Push {
        if self.current >= self.pushes.len() {
            self.current = 0;
        }
        let push = &self.pushes[self.current];
        self.current+=1;
        push
    }
}

type HistoryEntry = (Rock, usize, bool);

#[derive(Default)]
struct Chamber {
    space: Vec<Vec<bool>>,
    tallest: usize,
}

impl Chamber {
    fn move_positions(&self, pos: &mut Vec<Position>, x: i64, y: i64) {
        for p in pos {
            p.x += x;
            p.y += y;
        }
    }

    fn is_occupied(&self, x: i64, y: i64) -> bool {
        let y: usize = y.try_into().unwrap();
        let x: usize = x.try_into().unwrap();
        self.space[y][x]
    }

    fn can_move(&self, pos: &Vec<Position>, x: i64, y: i64) -> bool {
        for p in pos {
            let (newx, newy) = (p.x+x, p.y+y);
            if !(0..7).contains(&newx) || newy < 0 || self.is_occupied(newx, newy) {
                return false;
            }
        }

        true
    }

    fn can_drop(&self, pos: &Vec<Position>) -> bool {
        let can_drop = self.can_move(pos, 0, -1);
        self.can_move(pos, 0, -1)
    }

    fn drop(&self, pos: &mut Vec<Position>) {
        self.move_positions(pos, 0, -1)
    }

    fn can_push(&self, pos: &Vec<Position>, push: &Push) -> bool {
        match push {
            Push::Left => self.can_move(pos, -1, 0),
            Push::Right => self.can_move(pos, 1, 0),
        }
    }

    fn push(&self, pos: &mut Vec<Position>, push: &Push) {
        match push {
            Push::Left => self.move_positions(pos, -1, 0),
            Push::Right => self.move_positions(pos, 1, 0),
        }
    }

    fn debug_chamber(&self, pos: &[Position]) {
        println!("Tallest: {}", self.tallest);
        for y in (0..self.tallest + 7).rev() {
            for x in 0..self.space[0].len() {
                if self.space[y][x] {
                    print!("#");
                } else if pos.iter().any(|p| p.x == x.try_into().unwrap() && p.y == y.try_into().unwrap()) {
                    print!("@");
                } else {
                    print!("*");
                }
            }
            println!();
        }
    }

    fn drop_rock(&mut self, rock: &Rock, pushes: &mut Pushes, debug: bool, history: &mut Vec<HistoryEntry>, heights: &mut Vec<usize>) {
        if self.space.len() < self.tallest+10 {
            self.space.extend(vec![vec![false; 7]; 20]);
        }

        let mut top_line: u32 = 0;
        let mut max_range = 0;
        let mut current = 0;
        for i in 0..7 {
            if self.tallest == 0 {
                continue;
            }

            if !self.space[self.tallest-1][i] {
                current += 1;
                max_range = max(current, max_range)
            } else {
                current = 0;
            }

            if self.space[self.tallest-1][i] {
                top_line |= 1 << i;
            }
        }
        history.push((*rock, pushes.current, max_range < rock.need_width()));
        heights.push(self.tallest);


        let mut rock_positions = rock.start_position(2, (self.tallest+3).try_into().unwrap());
        let mut should_push = true;
        let mut buffer = String::new();
        let stdin = std::io::stdin(); // We get `Stdin` here
        loop {
            if debug {
                stdin.read_line(&mut buffer).unwrap();
                self.debug_chamber(&rock_positions);
            }

            if should_push {
                let push = pushes.get_next();
                if self.can_push(&rock_positions, push) {
                    self.push(&mut rock_positions, push)
                }
                should_push = false;
            } else if self.can_drop(&rock_positions) {
                self.drop(&mut rock_positions);
                should_push = true;
            } else {
                for pos in rock_positions.iter() {
                    let y: usize = pos.y.try_into().unwrap();
                    let x: usize = pos.x.try_into().unwrap();
                    self.space[y][x] = true;
                }

                // Find the tallest.. will be in the current tallest +4
                for x in 0..7 {
                    for y in (self.tallest..self.tallest+4).rev() {
                        if self.space[y][x] {
                            self.tallest = y+1;
                            break;
                        }
                    }
                }

                break;
            }
        }
    }
}

fn parse_pushes(file: &str) -> Vec<Push> {
    read_to_string(file).unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(Push::parse)
        .collect()
}

fn part1(history: &[usize]) {
    println!("Part 1: {}", history[2022]);
}

fn part2(history: &[HistoryEntry], heights: &[usize]) {
    let mut previous = HashMap::new();
    let mut cycle = (0, 0);
    for (i, entry) in history.iter().enumerate().skip(150) {
        if !entry.2 {
            continue;
        }

        if let Some(prev) = previous.get(entry) {
            cycle = (*prev, i);
            break;
            //println!("Cycle: {} -> {} ({})", cycle.0, cycle.1, cycle.1-cycle.0);
            //println!("Height: {} -> {} ({})", heights[cycle.0], heights[cycle.1], heights[cycle.1]-heights[cycle.0]);
            //previous.insert(*entry, i);
        } else {
            previous.insert(*entry, i);
        }
    }

    if cycle == (0, 0) {
        panic!("No cycles detected");
    }

    let height_diff = heights[cycle.1] - heights[cycle.0];
    let cycle_len = cycle.1 - cycle.0;
    let iters_left = 1_000_000_000_000 - cycle.0;
    let cycles_left = iters_left/cycle_len;
    let leftover = iters_left%cycle_len;
    let leftover_height = heights[cycle.0+leftover] - heights[cycle.0];
    println!("Cycle: {} -> {} ({})", cycle.0, cycle.1, cycle_len);
    println!("Height difference: {} -> {} ({})", heights[cycle.0], heights[cycle.1], height_diff);
    println!("Iters left: {}", iters_left);
    println!("Cycles left: {}", cycles_left);
    println!("Leftover len: {} height: {}", leftover, leftover_height);
    println!("Height at 140: {}", heights[140]);
    println!("Cycle test: {} = {}", heights[cycle.0 + cycle_len] + height_diff*2, heights[cycle.1+cycle_len*2]);

    // how many cycles in 1000B?
    let iters_left = 1_000_000_000_000 - cycle.0;
    let height = heights[cycle.0] + (cycles_left * height_diff) + heights[cycle.0+(iters_left % cycle_len)] - heights[cycle.0];
    println!("Height: {}", height);
}

pub fn run(file: &str) {
    let mut pushes = Pushes::new(parse_pushes(file));
    let mut chamber = Chamber::default();
    let rocks = vec![Rock::HorizontalLine, Rock::Plus, Rock::ReverseL, Rock::VerticalLine, Rock::Square];
    let mut history = Vec::new();
    let mut heights = Vec::new();
    for i in 0..10_000 {
        let rock = &rocks[i % 5];
        chamber.drop_rock(rock, &mut pushes, false, &mut history, &mut heights);
    }

    part1(&heights);
    part2(&history, &heights);
}

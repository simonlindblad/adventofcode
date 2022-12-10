use std::collections::VecDeque;
use std::fs::read_to_string;

/// Represents a supported CPU instruction
enum Instruction {
    Add(i32),
    Noop,
}

impl Instruction {
    /// Converts the raw line representation of a instruction to our
    /// enum.
    fn from_line(line: &str) -> Self {
        if line == "noop" {
            Instruction::Noop
        } else {
            let num = line.strip_prefix("addx ").unwrap();
            Instruction::Add(num.parse().unwrap())
        }
    }
}

struct Cpu {
    x: i32,
    cycle: i32,
    program: VecDeque<Instruction>,
    ongoing_add: bool,
}

impl Cpu {
    fn new(program: VecDeque<Instruction>) -> Self {
        Cpu {
            x: 1,
            cycle: 0,
            program,
            ongoing_add: false,
        }
    }

    fn run_cycle(&mut self) {
        match self.program.front() {
            Some(Instruction::Noop) => {
                self.cycle += 1;
                self.program.pop_front();
            }
            Some(Instruction::Add(n)) => {
                self.cycle += 1;
                if self.ongoing_add {
                    self.x += n;
                    self.program.pop_front();
                    self.ongoing_add = false;
                } else {
                    self.ongoing_add = true;
                }
            }
            None => self.cycle += 1,
        }
    }
}

fn part1(file: &str) {
    let content = read_to_string(file).unwrap();
    let commands = content.lines().map(Instruction::from_line);

    let mut cpu = Cpu::new(commands.collect());
    let mut cycles = VecDeque::from_iter((20..221).step_by(40));
    let mut sum = 0;
    for cycle in 1..221 {
        if let Some(c) = cycles.front() {
            if &cycle == c {
                sum += cpu.x * cycle;
                cycles.pop_front();
            }
        }
        cpu.run_cycle();
    }

    println!("Part 1: {}", sum)
}

fn part2(file: &str) {
    let content = read_to_string(file).unwrap();
    let commands = content.lines().map(Instruction::from_line);
    let mut cpu = Cpu::new(commands.collect());
    println!("Part 2:");
    for _row in 0..6 {
        for col in 0..40 {
            if cpu.x <= col + 1 && cpu.x >= col - 1 {
                print!("#");
            } else {
                print!(".");
            }

            cpu.run_cycle();
        }
        println!()
    }
}

pub fn run(file: &str) {
    part1(file);
    println!();
    part2(file);
}

use std::cmp::{Ord, Ordering};
use std::fs::read_to_string;

#[derive(Debug, Eq, Clone)]
enum PacketPart {
    Int(i32),
    List(Vec<PacketPart>),
}

impl Ord for PacketPart {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self, other) {
            (PacketPart::Int(i), PacketPart::Int(j)) => i.cmp(j),
            (PacketPart::List(is), PacketPart::List(js)) => {
                for (i, j) in is.iter().zip(js.iter()) {
                    match i.cmp(j) {
                        Ordering::Equal => continue,
                        o => return o,
                    }
                }

                is.len().cmp(&js.len())
            }
            (PacketPart::List(_), PacketPart::Int(_)) => {
                self.cmp(&PacketPart::List(vec![other.clone()]))
            }
            (PacketPart::Int(_), PacketPart::List(_)) => {
                let new = PacketPart::List(vec![self.clone()]);
                new.cmp(other)
            }
        }
    }
}

impl PartialOrd for PacketPart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PacketPart {
    fn eq(&self, other: &Self) -> bool {
        match (&self, other) {
            (PacketPart::Int(i), PacketPart::Int(j)) => i == j,
            (PacketPart::List(i), PacketPart::List(j)) => i == j,
            (PacketPart::Int(_), PacketPart::List(_)) => {
                PacketPart::List(vec![self.clone()]).eq(other)
            }
            (PacketPart::List(_), PacketPart::Int(_)) => {
                self.eq(&PacketPart::List(vec![other.clone()]))
            }
        }
    }
}

impl PacketPart {
    fn from_line(l: &str) -> Self {
        let chars = l.chars().collect::<Vec<_>>();
        let (parts, _) = PacketPart::get_packets(&chars, 0);
        parts
    }

    fn get_packets(l: &[char], start: usize) -> (PacketPart, usize) {
        if l[start] != '[' {
            panic!("Packets need to start with a list.")
        }

        let mut components = Vec::new();
        let mut idx = start + 1;
        while idx < l.len() && l[idx] != ']' {
            if l[idx].is_numeric() {
                let mut int = String::new();
                while l[idx].is_numeric() {
                    int.push(l[idx]);
                    idx += 1;
                }
                components.push(PacketPart::Int(int.parse::<i32>().unwrap())); // TODO: Convert char
                continue;
            } else if l[idx] == '[' {
                let (part, nidx) = PacketPart::get_packets(l, idx);
                components.push(part);
                idx = nidx;
            }

            idx += 1;
        }

        (PacketPart::List(components), idx + 1)
    }
}

pub fn part1(file: &str) {
    let packets = read_to_string(file)
        .unwrap()
        .split("\n\n")
        .map(|s| s.lines().map(PacketPart::from_line).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{:?}", packets[1]);

    println!(
        "Part 1: {:?}",
        packets
            .iter()
            .enumerate()
            .filter(|(_, p)| p[0] <= p[1])
            .map(|(i, _)| i + 1)
            .sum::<usize>()
    );
}
pub fn part2(file: &str) {
    let mut packets = read_to_string(file)
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(PacketPart::from_line)
        .collect::<Vec<_>>();

    packets.push(PacketPart::List(vec![PacketPart::Int(2)]));
    packets.push(PacketPart::List(vec![PacketPart::Int(6)]));

    packets.sort();

    let first = packets
        .iter()
        .position(|p| p == &PacketPart::List(vec![PacketPart::Int(2)]))
        .unwrap();
    let second = packets
        .iter()
        .position(|p| p == &PacketPart::List(vec![PacketPart::Int(6)]))
        .unwrap();

    println!("Part 2: {:?}", (first + 1) * (second + 1));
}

pub fn run(file: &str) {
    part1(file);
    part2(file);
}

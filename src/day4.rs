use std::fs::read_to_string;

#[derive(Debug)]
struct SectionRange {
    start: i32,
    end: i32,
}

impl SectionRange {
    fn from_str(range: &str) -> Self {
        let (start, end) = range.split_once('-').unwrap();
        SectionRange {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }

    fn contains(&self, other: &SectionRange) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &SectionRange) -> bool {
        !(self.start > other.end || self.end < other.start)
    }
}

fn count_pairs(file: &str, predicate: fn(&(SectionRange, SectionRange)) -> bool) -> usize {
    read_to_string(file)
        .unwrap()
        .lines()
        .map(|l| {
            let pair = l.split_once(',').unwrap();
            (
                SectionRange::from_str(pair.0),
                SectionRange::from_str(pair.1),
            )
        })
        .filter(predicate)
        .count()
}

pub fn run(file: &str) {
    println!(
        "Part 1: {}",
        count_pairs(file, |p| p.0.contains(&p.1) || p.1.contains(&p.0))
    );
    println!("Part 2: {}", count_pairs(file, |p| p.0.overlaps(&p.1)));
}

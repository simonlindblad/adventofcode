use aoc2023::read_input_content;

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u32,
}

#[derive(Debug, Default)]
struct Box {
    lenses: Vec<Lens>,
}

fn hash_fn(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, c| ((acc + (c as u32)) * 17) % 256)
}

fn part1() {
    let raw = read_input_content();
    let sum = raw.split(',').map(|s| s.trim()).map(hash_fn).sum::<u32>();

    println!("Part 1: {}", sum);
}

fn part2() {
    let mut boxes = (0..256).map(|_| Box::default()).collect::<Vec<_>>();
    let raw = read_input_content();
    for s in raw.split(',').map(|s| s.trim()) {
        if let Some(label) = s.strip_suffix('-') {
            boxes[hash_fn(label) as usize]
                .lenses
                .retain(|l| l.label != label)
        } else if let Some((label, focal_length)) = s
            .split_once('=')
            .map(|(label, focal_length)| (label.to_string(), focal_length.parse::<u32>().unwrap()))
        {
            let b = hash_fn(&label) as usize;
            if let Some(pos) = boxes[b].lenses.iter().position(|l| l.label == label) {
                boxes[b].lenses[pos].focal_length = focal_length;
            } else {
                boxes[b].lenses.push(Lens {
                    label,
                    focal_length,
                });
            };
        };
    }

    let mut sum = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, lens) in b.lenses.iter().enumerate() {
            sum += (i + 1) * (j + 1) * lens.focal_length as usize;
        }
    }
    println!("Part 2: {}", sum);
}

fn main() {
    part1();
    part2();
}

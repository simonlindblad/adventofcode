use aoc2023::{read_input_lines, Coordinate};

fn parse_coordinates_part1() -> Vec<(char, i64)> {
    read_input_lines()
        .iter()
        .map(|line| {
            let components = line.split_whitespace().collect::<Vec<_>>();
            (
                components[0].chars().next().unwrap(),
                components[1].parse::<i64>().unwrap(),
            )
        })
        .collect()
}

fn parse_coordinates_part2() -> Vec<(char, i64)> {
    read_input_lines()
        .iter()
        .map(|line| {
            let components = line.split_whitespace().collect::<Vec<_>>();
            let number = components[2].chars().skip(2).take(5).collect::<String>();
            let direction = match components[2].chars().nth(7).unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => unreachable!(),
            };
            (direction, i64::from_str_radix(&number, 16).unwrap())
        })
        .collect()
}

fn solve(directions: Vec<(char, i64)>) -> i64 {
    let mut coordinates = Vec::new();
    let mut current = Coordinate::new(0, 0);
    let mut total_boundary_points = 0;
    for (direction, length) in directions {
        let (next, boundary_points) = match (direction, length) {
            ('R', x) => (Coordinate::new(current.x + x, current.y), x),
            ('L', x) => (Coordinate::new(current.x - x, current.y), x),
            ('U', y) => (Coordinate::new(current.x, current.y - y), y),
            ('D', y) => (Coordinate::new(current.x, current.y + y), y),
            _ => panic!("Invalid input"),
        };

        total_boundary_points += boundary_points;

        coordinates.push(next);
        current = next;
    }

    coordinates.push(coordinates[0]);

    let mut double_area = 0;
    for i in 1..coordinates.len() {
        double_area +=
            coordinates[i - 1].x * coordinates[i].y - coordinates[i].x * coordinates[i - 1].y;
    }

    let interior_points = double_area / 2 - total_boundary_points / 2 + 1;

    interior_points + total_boundary_points
}

fn main() {
    println!("Part 1: {}", solve(parse_coordinates_part1()));
    println!("Part 2: {}", solve(parse_coordinates_part2()));
}

use aoc2023::read_input_lines;

fn get_sensor_data() -> Vec<Vec<i64>> {
    read_input_lines()
        .iter()
        .map(|s| {
            s.split(' ')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn get_differences(inp: &[i64]) -> Vec<i64> {
    inp.iter()
        .enumerate()
        .skip(1)
        .map(|(i, &v)| v - inp[i - 1])
        .collect::<Vec<_>>()
}

fn reduce_differences(
    mut diffs: Vec<i64>,
    tail: fn(&Vec<i64>) -> i64,
    reduction: fn(a: i64, b: i64) -> i64,
) -> i64 {
    let mut tails = vec![tail(&diffs)];
    while !diffs.iter().all(|d| d == &0) {
        diffs = get_differences(&diffs);
        tails.push(tail(&diffs));
    }

    tails.into_iter().rev().reduce(reduction).unwrap()
}

fn part1() {
    let sensor_data = get_sensor_data();
    let res = sensor_data
        .into_iter()
        .map(|sd| reduce_differences(sd, |v| *v.last().unwrap(), |a, b| a + b))
        .sum::<i64>();

    println!("{:?}", res);
}

fn part2() {
    let sensor_data = get_sensor_data();
    let res = sensor_data
        .into_iter()
        .map(|sd| reduce_differences(sd, |v| *v.first().unwrap(), |a, b| b - a))
        .sum::<i64>();
    println!("{:?}", res);
}

fn main() {
    part1();
    part2();
}

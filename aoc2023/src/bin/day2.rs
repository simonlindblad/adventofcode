use aoc2023::read_input_file;

#[derive(Debug, Clone)]
struct CubeSet {
    blue: usize,
    green: usize,
    red: usize,
}

impl CubeSet {
    fn parse(set: &str) -> CubeSet {
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;

        for (count, color) in set.split(", ").map(|cube| cube.split_once(' ').unwrap()) {
            match color {
                "blue" => blue = count.parse::<usize>().unwrap(),
                "green" => green = count.parse::<usize>().unwrap(),
                "red" => red = count.parse::<usize>().unwrap(),
                _ => panic!("Unknown color"),
            }
        }

        CubeSet { blue, green, red }
    }

    fn can_play(&self, set: &CubeSet) -> bool {
        self.blue <= set.blue && self.green <= set.green && self.red <= set.red
    }

    fn get_max(&self, set: &CubeSet) -> CubeSet {
        CubeSet {
            blue: self.blue.max(set.blue),
            green: self.green.max(set.green),
            red: self.red.max(set.red),
        }
    }

    fn power(&self) -> usize {
        self.blue * self.green * self.red
    }
}

#[derive(Debug)]
struct Game {
    hands: Vec<CubeSet>,
    id: usize,
}

impl Game {
    fn parse<S: AsRef<str>>(line: S) -> Game {
        let line = line.as_ref();
        let (game, hands) = line.split_once(':').unwrap();
        let id = game
            .strip_prefix("Game ")
            .unwrap()
            .chars()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let hands = hands
            .split(';')
            .map(|hand| hand.trim())
            .map(CubeSet::parse)
            .collect::<Vec<_>>();

        Game { hands, id }
    }

    fn can_play(&self, available: &CubeSet) -> bool {
        self.hands.iter().all(|hand| hand.can_play(available))
    }

    fn get_power(self) -> usize {
        self.hands
            .into_iter()
            .reduce(|a, b| a.get_max(&b))
            .unwrap()
            .power()
    }
}

fn part2() {
    let score = read_input_file()
        .iter()
        .map(Game::parse)
        .map(|game| game.get_power())
        .sum::<usize>();

    println!("Score: {}", score);
}

fn part1() {
    let available = CubeSet {
        blue: 14,
        green: 13,
        red: 12,
    };

    let score = read_input_file()
        .iter()
        .map(Game::parse)
        .filter(|game| game.can_play(&available))
        .map(|game| game.id)
        .sum::<usize>();

    println!("Score: {}", score);
}

fn main() {
    part1();

    part2();
}

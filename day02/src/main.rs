use std::cmp::max;
use std::fs;

type Parsed = Vec<Game>;

#[derive(Debug)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

impl Default for Game {
    fn default() -> Self {
        Game { red: 0, green: 0, blue: 0 }
    }
}

impl Game {
    fn merge(&mut self, other: &Game) {
        self.red = max(self.red, other.red);
        self.green = max(self.green, other.green);
        self.blue = max(self.blue, other.blue);
    }

    fn is_ok(&self, limit: &Game) -> bool {
        self.red <= limit.red && self.green <= limit.green && self.blue <= limit.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

struct PullPart(u32, String);

impl PullPart {
    fn parse(value: &str) -> Option<Self> {
        let mut parts = value.splitn(2, " ");
        let count = parts.next()?.parse::<u32>().ok()?;
        let color = parts.next()?.to_string();
        Some(PullPart(count, color))
    }
}

fn parse_pull(pull: &str) -> Option<Game> {
    let blocks: Vec<&str> = pull.split(", ").collect();
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for block in blocks {
        if let Some(part) = PullPart::parse(block) {
            match part {
                PullPart(count, color) => {
                    if color == "red" {
                        red += count;
                    } else if color == "green" {
                        green += count;
                    } else if color == "blue" {
                        blue += count;
                    } else {
                        return None;
                    }
                }
            }
        }
    }
    Some(Game {red, green, blue})
}

fn parse_line(line: &str) -> Option<Game> {
    let parts: Vec<&str> = line.split(": ").collect();
    let pulls: Vec<&str> = parts.get(1)?.split("; ").collect();
    let mut game = Game::default();
    for pull in pulls {
        game.merge(&parse_pull(pull)?);
    }
    Some(game)
}

fn parse(content: &String) -> Parsed {
    let mut result: Vec<Game> = Vec::new();
    for line in content.split("\n") {
        if let Some(p) = parse_line(line) {
            result.push(p);
        }
    }
    return result;
}

fn part1(root: &Parsed) {
    let limit = Game { red: 12, green: 13, blue: 14};
    let mut sum = 0;

    // iterates over root with indices
    for (i, game) in root.iter().enumerate() {
        if game.is_ok(&limit) {
            sum += i + 1;
        }
    }

    println!("Part 1: {}", sum);
}

fn part2(root: &Parsed) {
    let mut sum = 0;
    for game in root {
        sum += game.power();
    }
    println!("Part 2: {}", sum);
}

fn main() {
    let files = vec!["sample.txt", /*"sample2.txt" ,*/ "input.txt"];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content);
        part1(&root);
        part2(&root);
    }
}
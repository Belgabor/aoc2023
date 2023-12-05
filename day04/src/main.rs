use std::cmp::min;
use std::collections::HashMap;
use std::fs;

type Parsed = Vec<Card>;

#[derive(Debug)]
struct Card {
    winning: Vec<u32>,
    drawn: Vec<u32>,
}

fn parse_numbers(part: &str) -> Option<Vec<u32>> {
    let numbers: Result<Vec<_>, _> = part.split_whitespace().map(|s| s.parse::<u32>()).collect();
    numbers.ok()
}

impl Card {
    fn parse(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(" | ").collect();
        if parts.len() != 2 {
            return None;
        }
        let winning: Vec<u32> = parse_numbers(parts[0])?;
        let drawn: Vec<u32> = parse_numbers(parts[1])?;

        return Some(Card{winning, drawn});
    }

    fn match_count(&self) -> u32 {
        let mut matching: Vec<u32> = Vec::new();
        for number in self.drawn.iter() {
            if self.winning.contains(number) {
                matching.push(*number);
            }
        }
        matching.len() as u32
    }

    fn score(&self) -> u32 {
        let matching = self.match_count();

        if matching == 0 {
            0
        } else if matching == 1 {
            1
        } else {
            let base: u32 = 2;
            base.pow(matching-1)
        }

    }
}

fn parse(content: &String) -> Parsed {
    let mut cards: Vec<Card> = Vec::new();

    for line in content.split("\n") {
        let parts: Vec<&str> = line.split(": ").collect();
        cards.push(Card::parse(parts[1].trim()).unwrap());
    }

    return cards;
}

fn part1(root: &Parsed) {
    //println!("{:?}", root);
    let mut sum = 0;
    for game in root {
        //println!("{}", game.score());
        sum += game.score();
    }
    println!("Part 1: {}", sum);
}

fn part2(root: &Parsed) {
    let mut counts: HashMap<usize, u32> = HashMap::new();
    let last_index = root.len() - 1;

    for (index, game) in root.iter().enumerate() {
        let count = game.match_count();

        if count > 0 && index < last_index {
            let copies = *counts.entry(index).or_insert(1);
            for sub in index+1..=min(index+count as usize, last_index) {
                *counts.entry(sub).or_insert(1) += copies;
            }
        }
    }

    let mut sum = 0;

    for index in 0..=last_index {
        sum += *counts.entry(index).or_insert(1);
    }

    println!("Part 2: {}", sum);
}

fn main() {
    let files = vec!["sample.txt", "input.txt" ];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content);
        part1(&root);
        part2(&root);
    }
}
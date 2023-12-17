use std::fs;

type Parsed = Records;

#[derive(Debug)]
struct Records(Vec<Record>);

impl Records {
    fn calculate_arrangements(&self) -> usize {
        self.0.iter().map(|r| r.calculate_arrangements()).sum()
    }
}

#[derive(Debug, Clone)]
struct Springs(Vec<Option<bool>>);

impl Springs {
    fn groups(&self) -> Option<Groups> {
        let mut current = 0;
        let mut groups = Vec::new();
        for spring in self.0.iter() {
            let spring = (*spring)?;
            if spring {
                current += 1;
            } else {
                if current > 0 {
                    groups.push(current);
                    current = 0;
                }
            }
        }
        if current > 0 {
            groups.push(current);
            current = 0;
        }
        Some(Groups(groups))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Groups(Vec<usize>);

#[derive(Debug)]
struct Record {
    springs: Springs,
    unknown: Vec<usize>,
    groups: Groups,
}

impl Record {
    fn parse(content: &str) -> Record {
        let (springs, groups) = content.split_once(" ").unwrap();
        let mut unknown = Vec::new();
        let springs = Springs(springs.chars().enumerate().map(|(index, c)| {
            let el = match c {
                '#' => Some(true),
                '.' => Some(false),
                '?' => None,
                _ => unreachable!(),
            };
            if el.is_none() {
                unknown.push(index)
            }
            el
        }).collect());

        let groups = Groups(groups.split(",").map(|s| s.parse().unwrap()).collect());

        Record {springs, unknown, groups}
    }

    fn parse2(content: &str) -> Record {
        let (springs, groups) = content.split_once(" ").unwrap();
        let mut unknown = Vec::new();

        let springs = vec![springs; 5].join("?");
        let groups = vec![groups; 5].join(",");

        let springs = Springs(springs.chars().enumerate().map(|(index, c)| {
            let el = match c {
                '#' => Some(true),
                '.' => Some(false),
                '?' => None,
                _ => unreachable!(),
            };
            if el.is_none() {
                unknown.push(index)
            }
            el
        }).collect());

        let groups = Groups(groups.split(",").map(|s| s.parse().unwrap()).collect());

        Record {springs, unknown, groups}
    }

    fn calculate_arrangements(&self) -> usize {
        let mut possibilities = Vec::new();
        possibilities.push(self.springs.clone());
        for unknown in self.unknown.iter() {
            let mut new_possibilities = Vec::new();
            for springs in possibilities.iter() {
                let mut el = springs.clone();
                el.0[*unknown] = Some(true);
                new_possibilities.push(el);
                el = springs.clone();
                el.0[*unknown] = Some(false);
                new_possibilities.push(el);
            }
            possibilities = new_possibilities;
        }

        let valid_possibilities = possibilities.iter().filter(|possibility| {
            possibility.groups().unwrap() == self.groups
        }).collect::<Vec<_>>();

        valid_possibilities.len()
    }
}

fn parse(content: &String) -> Parsed {
    let mut records = Vec::new();
    for line in content.split("\n") {
        records.push(Record::parse(line));
    }
    return Records(records);
}

fn parse2(content: &String) -> Parsed {
    let mut records = Vec::new();
    for line in content.split("\n") {
        records.push(Record::parse2(line));
    }
    return Records(records);
}

fn part1(root: &Parsed) {
    // println!("{:?}", root);
    println!("Part 1: {}", root.calculate_arrangements());
}

fn part2(root: &Parsed) {

    println!("Part 2: {}", root.calculate_arrangements());
}

fn main() {
    let files = vec!["sample.txt", /*"sample2.txt" ,*/ /*"input.txt"*/ ];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content);
        part1(&root);
        let root = parse2(&content);
        part2(&root);
    }
}
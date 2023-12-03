use std::collections::HashMap;
use std::fs;
use std::ops::Deref;

type Parsed = Vec<String>;

fn parse(content: &String) -> Parsed {
    return content.split("\n").map(|s| s.to_string()).collect();
}

fn part1(root: &Parsed) {
    println!("{:?}", root);
    let items: Vec<u32> = root.iter().map(|e| {
        let mut first = None;
        let mut last = None;
        // iterate over characters in string
        e.chars().for_each(|c| {
            // check if character is a number
            c.to_digit(10).and_then(|d| {
                if first.is_none() {
                    first = Some(d);
                }
                last = Some(d);
                None::<()>
            });
        });

        first.unwrap_or(0) * 10 + last.unwrap_or(0)
    }).collect();
    let sum: u32 = items.iter().sum();
    println!("Part 1: {}", sum);
}

fn part2(root: &Parsed) {
    let digits: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let items: Vec<u32> = root.iter().map(|e| {
        let mut first = None;
        let mut last = None;

        let mut index = 0;

        // iterate over the length of the string
        e.chars().for_each(|c| {
            let digit = c.to_digit(10).or_else(|| {
                let part = e.get(index..)?;
                //println!("{}", part);
                let found = digits.iter().find(|(key, _)| {
                    part.starts_with(key.deref())
                });

                return found.map(|(_, value)| (*value).into())
            });

            digit.and_then(|d| {
                if first.is_none() {
                    first = Some(d);
                }
                last = Some(d);
                None::<()>
            });
            index += 1;
        });
        /*
        e.chars().for_each(|c| {
            // check if character is a number
            c.to_digit(10).and_then(|d| {
                if first.is_none() {
                    first = Some(d);
                }
                last = Some(d);
                None::<()>
            });
        });

         */

        first.unwrap() * 10 + last.unwrap()
    }).collect();
    let sum: u32 = items.iter().sum();

    println!("Part 2: {}", sum);
}

fn main() {
    let files = vec!["sample.txt", "sample2.txt", "input.txt" ];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content);
        part1(&root);
        part2(&root);
    }
}
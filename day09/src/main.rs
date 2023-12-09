use std::fs;

type Parsed = Vec<Line>;

#[derive(Debug)]
struct Line {
    values: Vec<i64>,
    derivatives: Vec<Vec<i64>>,
}

impl Line {
    fn parse(content: &str) -> Line {
        let values: Vec<_> = content.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();

        let mut derivatives = Vec::new();
        let mut previous_derivatives = values.clone();
        loop {
            let mut current_derivatives = Vec::new();

            let mut all_zero = true;
            for i in 1..previous_derivatives.len() {
                let diff = previous_derivatives[i] - previous_derivatives[i-1];
                if diff != 0 {
                    all_zero = false;
                }
                current_derivatives.push(diff);
            }
            if all_zero {
                break;
            }
            previous_derivatives = current_derivatives.clone();
            derivatives.push(current_derivatives);
        }

        Line { values, derivatives }
    }

    fn next_number(&self) -> i64 {
        self.values.last().unwrap() + self.derivatives.iter().map(|d| d.last().unwrap()).sum::<i64>()
    }
    fn prev_number(&self) -> i64 {
        self.values.first().unwrap() - self.derivatives.iter().rev().map(|d| d.first().unwrap()).fold(0, |acc, el| {
            el - acc
        })
    }
}

fn parse(content: &String) -> Parsed {
    let mut result = Vec::new();

    for line in content.split("\n") {
        result.push(Line::parse(line));
    }

    result
}

fn part1(root: &Parsed) {
    let mut sum = 0;

    for line in root.iter() {
        sum += line.next_number();
    }

    println!("Part 1: {}", sum);
}

fn part2(root: &Parsed) {
    let mut sum = 0;

    for line in root.iter() {
        sum += line.prev_number();
    }

    println!("Part 2: {}", sum);
}

fn main() {
    let files = vec!["sample.txt", /*"sample2.txt" ,*/ "input.txt" ];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content);
        part1(&root);
        part2(&root);
    }
}
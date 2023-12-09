use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Range;

//type Parsed = String;

#[derive(Debug)]
struct Parsed {
    directions: Vec<char>,
    nodes: HashMap<String, Node>
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn parse(content: &String) -> Parsed {
    let (raw_directions, raw_nodes) = content.split_once("\n\n").unwrap();
    let mut nodes = HashMap::new();
    for line in raw_nodes.split("\n") {
        let (from, to) = line.split_once(" = ").unwrap();
        let (left, right) = to[1..to.len()-1].split_once(", ").unwrap();
        nodes.insert(from.to_string(), Node { left: left.to_string(), right: right.to_string() });
    }
    return Parsed{directions: raw_directions.trim().chars().collect(), nodes};
}

fn part1(root: &Parsed) {
    //println!("{:?}", root);

    let mut current_node = "AAA".to_string();
    let mut position = root.directions.iter();
    let mut steps = 0;
    loop {
        let direction = *position.next().or_else(|| {
            position = root.directions.iter();
            position.next()
        }).unwrap();
        let node = root.nodes.get(&current_node).unwrap();
        current_node = match direction {
            'R' => node.right.clone(),
            'L' => node.left.clone(),
            _ => unreachable!()
        };
        steps += 1;
        if current_node == "ZZZ" {
            break;
        }
    }

    println!("Part 1: {}", steps);
}

#[derive(Debug)]
struct Loop {
    start: String,
    range: Range<i32>,
}

fn part2(root: &Parsed) {
    let mut positions = Vec::new();

    for node in root.nodes.keys() {
        if node.ends_with("A") {
            positions.push(node.clone())
        }
    }

    println!("Starts: {:?}", positions);

    let mut loops = Vec::new();
    for position in positions.iter() {
        println!("Position: {}", position);
        let mut current_position = position.clone();
        let mut current = root.directions.iter();
        let mut history = HashMap::new();
        let mut steps: i32 = 0;
        loop {
            steps += 1;
            let direction = *current.next().or_else(|| {
                current = root.directions.iter();
                current.next()
            }).unwrap();
            let node = root.nodes.get(&current_position).unwrap();
            let next = match direction {
                'R' => node.right.clone(),
                'L' => node.left.clone(),
                _ => unreachable!()
            };
            if history.contains_key(&next) {
                let start = history.get(&next).unwrap();
                loops.push(Loop { start: position.clone(), range: *start..steps});
                println!("Loop detected: {}", current_position);
                break;
            }
            if (current_position.ends_with("Z")) {
                println!("No Loop: {}", current_position);
            }
            current_position = next;
            history.insert(current_position.clone(), steps);
        }
    }
    println!("Starts: {:?} {:?}", loops, loops.iter().map(|l| l.range.len()).collect::<Vec<_>>());

    let mut step = 1;
    let count = loops.len();
    'steps: loop {
        step += 1;
        let mut on_end = 0;
        for l in loops.iter() {
            let rest: i32 = step - l.range.start;
            if rest < 0 {
                continue 'steps;
            }
            if rest % (l.range.len() as i32) == 0 {
                on_end += 1;
            }
        }
        if on_end == count {
            break;
        }
    }

    println!("Part 2: {}", step);
}

fn main() {
    {
        println!("Part 1");
        let files = vec!["sample.txt", /*"sample2.txt" ,*/ "input.txt"];
        for file in files {
            println!("Reading {}", file);
            let content = fs::read_to_string(file).expect("Cannot read file");
            let root = parse(&content);
            part1(&root);
        }
    }
    {
        println!("---");
        println!("Part 2");
        let files = vec!["sample2.txt"/*, "input.txt"*/];
        for file in files {
            println!("Reading {}", file);
            let content = fs::read_to_string(file).expect("Cannot read file");
            let root = parse(&content);
            part2(&root);
        }
    }
}
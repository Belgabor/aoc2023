use std::fs;

type Parsed = String;

fn parse(content: &String) -> Parsed {
    for line in content.split("\n") {
        todo!()
    }
    return content.clone();
}

fn part1(root: &Parsed) {
    println!("{:?}", root);
    println!("Part 1: {}", "TODO");
}

fn part2(_root: &Parsed) {
    println!("Part 2: {}", "TODO");
}

fn main() {
    let files = vec!["sample.txt", "sample2.txt" , "input.txt" ];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content);
        part1(&root);
        part2(&root);
    }
}
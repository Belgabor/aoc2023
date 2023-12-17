#![feature(ascii_char)]
#![feature(hash_raw_entry)]

use std::collections::HashMap;
use std::fs;
use indexmap::IndexMap;
use crate::Op::{Add, Remove};

type Parsed = Instructions;
type Int = usize;

#[derive(Debug)]
struct Instructions(Vec<Instruction>);

impl Instructions {
    fn hash_sum(&self) -> Int {
        self.0.iter().map(|x| x.hash()).sum()
    }

    fn execute(&self) -> Factory {
        let mut factory = Factory::default();
        for instruction in self.0.iter() {
            // println!("{:#?}", factory);
            let box_number = hash(&instruction.label);
            let (_, box_) = factory.boxes.raw_entry_mut().from_key(&box_number).or_insert(box_number, LensBox::default());
            match instruction.op {
                Remove => {
                    box_.lenses.shift_remove(&instruction.label);
                }
                Add(focal) => {
                    box_.lenses.insert(instruction.label.clone(), focal);
                }
            }
        }

        factory
    }
}

fn hash(val: &str) -> Int {
    let mut hash: Int = 0;

    for c in val.chars() {
        hash += c.as_ascii().unwrap().to_u8() as Int;
        hash *= 17;
        hash %= 256;
    }

    hash
}

#[derive(Debug)]
enum Op {
    Remove,
    Add(Int),
}

#[derive(Debug)]
struct Instruction {
    raw: String,
    label: String,
    op: Op,
}

impl Instruction {
    fn parse(raw: &str) -> Self {
        let (op, label) = if raw.ends_with("-") {
            (Remove, raw[..raw.len()-1].to_string())
        } else {
            let parts = raw.split_once("=").unwrap();
            (Add(parts.1.parse().unwrap()), parts.0.to_string())
        };

        Instruction {raw: raw.to_string(), op, label}
    }

    fn hash(&self) -> Int {
        hash(&self.raw)
    }
}

#[derive(Debug, Default)]
struct LensBox {
    lenses: IndexMap<String, Int>
}

impl LensBox {
    fn power(&self, box_: Int) -> Int {
        self.lenses.iter()
            .enumerate()
            .map(|(i, (_, focal))| {
                let pow = box_ * focal * (i + 1);

                // println!("{} {} {} {}", box_, i + 1, focal, pow);

                pow
            })
            .sum()
    }
}

#[derive(Debug, Default)]
struct Factory {
    boxes: HashMap<Int, LensBox>
}

impl Factory {
    fn power(&self) -> Int {
        self.boxes.iter()
            .map(|(pos, box_)| box_.power(pos + 1))
            .sum()
    }
}


fn parse(content: &String) -> Parsed {
    let mut instructions = Vec::new();
    let line = content.split("\n").next().unwrap();
    for i in line.split(",") {
        instructions.push(Instruction::parse(i));
    }
    return Instructions(instructions);
}

fn part1(root: &Parsed) {
    println!("{:?}", root);
    println!("Part 1: {}", root.hash_sum());
}

fn part2(root: &Parsed) {
    // println!("{:#?}", root.execute());

    println!("Part 2: {}", root.execute().power());
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
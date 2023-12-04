use std::cell::RefCell;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::rc::Rc;

type Parsed = Board;

#[derive(Debug, Clone)]
struct Part {
    id: u32,
    number: u32,
    counted: bool,
}

#[derive(Debug)]
struct Board {
    part_id: u32,
    width: usize,
    height: usize,
    parts: HashMap<(usize, usize), Rc<RefCell<Part>>>,
    symbols: HashMap<(usize, usize), char>,
}

impl Board {
    fn finalize_part(&mut self, part: u32, start: Option<usize>, row: usize, index: usize) -> Option<usize> {
        if let Some(start) = start {
            let part = Rc::new(RefCell::new(Part{id: self.part_id, number: part, counted: false}));
            self.part_id += 1;
            for i in start..index {
                self.parts.insert((row, i), Rc::clone(&part));
            }
        }
        None
    }
}

fn parse(content: &String) -> Parsed {
    let mut row = 0;
    let mut board = Board { parts: HashMap::new(), symbols: HashMap::new(), width: 0, height: 0, part_id: 0 };
    for line in content.split("\n") {
        board.width = line.len();
        let mut part: u32 = 0;
        let mut start: Option<usize> = None;

        for (i, c) in line.chars().enumerate() {
            if c == '.' {
                start = board.finalize_part(part, start, row, i);
                continue;
            }
            if let Some(d) = c.to_digit(10) {
                if start.is_none() {
                    start = Some(i);
                    part = d;
                } else {
                    part = part * 10 + d;
                }
            } else {
                board.symbols.insert((row, i), c);
                start = board.finalize_part(part, start, row, i);
            }
        }
        board.finalize_part(part, start, row, board.width+1);
        row += 1;
    }
    board.height = row;
    return board;
}

fn part1(root: &Parsed) {
    //println!("{:?}", root);

    let mut sum = 0;
    let mut parts = root.parts.clone();

    for ((row, col), _) in root.symbols.iter() {
        for r in max(row-1, 0)..=min(row+1, root.height-1) {
            for c in max(col-1, 0)..=min(col+1, root.width-1) {
                if r == *row && c == *col {
                    continue;
                }
                if let Some(_part) = parts.get(&(r, c)) {
                    let mut part = _part.borrow_mut();
                    if !part.counted {
                        //println!("{}", part.number);
                        sum += part.number;
                        part.counted = true;
                    }
                }
            }
        }
    }

    println!("Part 1: {}", sum);
}

fn part2(root: &Parsed) {
    let mut sum = 0;
    let mut parts = root.parts.clone();

    for ((row, col), symbol) in root.symbols.iter() {
        if *symbol != '*' {
            continue;
        }

        let mut ids: HashSet<u32> = HashSet::new();
        let mut used: HashSet<u32> = HashSet::new();

        for r in max(row-1, 0)..=min(row+1, root.height-1) {
            for c in max(col-1, 0)..=min(col+1, root.width-1) {
                if r == *row && c == *col {
                    continue;
                }
                if let Some(_part) = parts.get(&(r, c)) {
                    let part = _part.borrow();
                    if !ids.contains(&part.id) {
                        ids.insert(part.id);
                        used.insert(part.number);
                    }
                }
            }
        }
        if used.len() == 2 {
            sum += used.iter().product::<u32>();
        }
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
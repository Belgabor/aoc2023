use std::collections::{HashMap, HashSet};
use std::fs;
use either::{Left, Right};
use Direction::{East, South, West};
use crate::Direction::North;
use crate::Orientation::{Horizontal, Vertical};

type Parsed = Platform;
type Int = usize;

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Default, Clone)]
struct Platform {
    rocks: HashSet<Coordinate>,
    blocks: HashSet<Coordinate>,
    width: Int,
    height: Int,
}

impl Platform {
    fn print(&self) {
        println!("--------");
        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                line += match Coordinate(x,y) {
                    c if self.rocks.contains(&c) => "O",
                    c if self.blocks.contains(&c) => "#",
                    _ => "."
                }
            }
            println!("{}", line);
        }
    }

    fn tilted(&self, direction: Direction) -> Platform {
        let mut tilted = Platform::default();
        tilted.width = self.width;
        tilted.height = self.height;

        let (outer_range, inner_range, orientation, reverted, border) = match direction {
            North => (0..self.height, 0..self.width, Vertical, false, 0),
            East => (0..self.width, 0..self.height, Horizontal, true, self.width - 1),
            South => (0..self.height, 0..self.width, Vertical, true, self.height - 1),
            West => (0..self.width, 0..self.height, Horizontal, false, 0),
        };


        for outer in if reverted { Left(outer_range.rev()) } else { Right(outer_range) } {
            for inner in inner_range.clone() {
                let coord = if orientation == Vertical {Coordinate(inner, outer)} else {Coordinate(outer, inner)};
                if self.blocks.contains(&coord) {
                    tilted.blocks.insert(coord);
                    continue;
                }
                if !self.rocks.contains(&coord) {
                    continue;
                }
                if outer == border {
                    tilted.rocks.insert(coord);
                    continue;
                }
                let mut current = coord;
                let outer_max = if orientation == Vertical { self.height } else { self.width };
                for outer2 in if reverted { Right(outer+1..outer_max) } else { Left((0..outer).rev()) } {
                    let next = if orientation == Vertical {Coordinate(inner, outer2)} else {Coordinate(outer2, inner)};
                    if tilted.blocks.contains(&next) || tilted.rocks.contains(&next) {
                        tilted.rocks.insert(current);
                        break;
                    }
                    if outer2 == border {
                        tilted.rocks.insert(next);
                    } else {
                        current = next;
                    }
                }
            }
        }

        tilted
    }

    fn cycle(&self) -> Platform {
        self.tilted(North).tilted(West).tilted(South).tilted(East)
    }

    fn load(&self) -> Int {
        let mut load = 0;

        for rock in self.rocks.iter() {
            load += self.height - rock.1;
        }

        load
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
struct Coordinate(Int, Int);

fn parse(content: &String) -> Parsed {
    let mut platform = Platform::default();
    for (y, line) in content.split("\n").enumerate() {
        platform.height += 1;
        platform.width = line.len() as Int;
        for (x, c) in line.chars().enumerate() {
            match c {
                'O' => platform.rocks.insert(Coordinate(x as Int, y as Int)),
                '#' => platform.blocks.insert(Coordinate(x as Int, y as Int)),
                '.' => false,
                _ => unreachable!(),
            };
        }
    }
    return platform;
}

fn part1(root: &Parsed) {
    //println!("{:?}", root);
    root.print();
    let tilted = root.tilted(North);
    tilted.print();
    println!("Part 1: {}", tilted.load());
}

fn part2(root: &Parsed) {
    root.print();
    let mut cycled = (*root).clone();
    for i in 0..1_000_000_000 {
        if i % 1_000_000 == 0 {
            println!("{}", i)
        }
        cycled = cycled.cycle();
    }
    cycled.print();
    println!("Part 2: {}", cycled.load());
}

fn main() {
    let files = vec!["sample.txt", /*"sample2.txt" ,*/ /*"input.txt"*/ ];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content);
        //part1(&root);
        part2(&root);
    }
}
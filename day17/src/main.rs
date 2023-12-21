#![feature(let_chains)]

use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs;
use pathfinding::prelude::dijkstra;
use Direction::{Down, Right, Up};
use crate::Direction::Left;

type Parsed = City;
type Int = isize;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
struct Coordinate(Int, Int);

impl Coordinate {
    fn ok(&self, width: Int, height: Int) -> bool {
        self.0 >= 0 && self.1 >= 0 && self.0 < width && self.1 < height
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }

    fn dx(&self) -> Int {
        match self {
            Up => 0,
            Right => 1,
            Down => 0,
            Left => -1,
        }
    }

    fn dy(&self) -> Int {
        match self {
            Up => -1,
            Right => 0,
            Down => 1,
            Left => 0,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct CostVariant {
    from: Direction,
    length: Int,
}

impl CostVariant {
    fn can_go(&self, direction: Direction) -> bool {
        if direction == self.from {
            return false;
        }

        if direction == self.from.opposite() {
            return self.length < 3;
        }

        true
    }

    fn next(&self, direction: Direction) -> Self {
        if direction == self.from {
            unreachable!();
        }

        if direction == self.from.opposite() {
            return CostVariant { from: self.from, length: self.length + 1 };
        }

        CostVariant { from: direction.opposite(), length: 1 }
    }

    fn can_go_part2(&self, direction: Direction) -> bool {
        if direction == self.from {
            return false;
        }

        if direction == self.from.opposite() {
            return self.length < 10;
        }

        self.length >= 4
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    coordinate: Coordinate,
    variant: CostVariant,
}

impl State {
    fn next(&self, direction: Direction, width: Int, height: Int) -> Option<Self> {
        if !self.variant.can_go(direction) {
            return None;
        }

        let coordinate = Coordinate(self.coordinate.0 + direction.dx(), self.coordinate.1 + direction.dy());
        if !coordinate.ok(width, height) {
            return None;
        }

        let variant = self.variant.next(direction);

        Some(State { coordinate, variant })
    }

    fn next_part2(&self, direction: Direction, width: Int, height: Int) -> Option<Self> {
        if !self.variant.can_go_part2(direction) {
            return None;
        }

        let coordinate = Coordinate(self.coordinate.0 + direction.dx(), self.coordinate.1 + direction.dy());
        if !coordinate.ok(width, height) {
            return None;
        }

        let variant = self.variant.next(direction);

        Some(State { coordinate, variant })
    }
}

#[derive(Debug, Default, Clone)]
struct City {
    heat_loss: Vec<Vec<Int>>,
    width: Int,
    height: Int,
}

impl City {
    fn get_loss(&self, coordinate: &Coordinate) -> Int {
        self.heat_loss[coordinate.1 as usize][coordinate.0 as usize]
    }


    fn get_optimal_v2(&self, x: Int, y: Int) -> Int {
        let result = dijkstra(
            &State { coordinate: Coordinate(0, 0), variant: CostVariant { from: Left, length: 0 } },
            |state| {
                [
                    state.next(Up, self.width, self.height),
                    state.next(Right, self.width, self.height),
                    state.next(Down, self.width, self.height),
                    state.next(Left, self.width, self.height),
                ]
                    .into_iter()
                    .filter_map(|new_state| {
                        let new_state = new_state?;
                        let cost = self.get_loss(&new_state.coordinate);

                        Some((new_state, cost))
                    })
            },
            |state| {
                state.coordinate.0 == x && state.coordinate.1 == y
            },
        );

        result.unwrap().1
    }

    fn get_optimal_part2(&self, x: Int, y: Int) -> Int {
        let result = dijkstra(
            &State { coordinate: Coordinate(0, 0), variant: CostVariant { from: Left, length: 0 } },
            |state| {
                [
                    state.next_part2(Up, self.width, self.height),
                    state.next_part2(Right, self.width, self.height),
                    state.next_part2(Down, self.width, self.height),
                    state.next_part2(Left, self.width, self.height),
                ]
                    .into_iter()
                    .filter_map(|new_state| {
                        let new_state = new_state?;
                        let cost = self.get_loss(&new_state.coordinate);

                        Some((new_state, cost))
                    })
            },
            |state| {
                state.coordinate.0 == x && state.coordinate.1 == y
            },
        );

        result.unwrap().1
    }

}

fn parse(content: &String) -> Parsed {
    let mut city = City::default();
    let mut rows = Vec::new();
    for line in content.split("\n") {
        let mut losses = Vec::new();
        for c in line.chars() {
            losses.push(c.to_digit(10).unwrap() as Int)
        }
        rows.push(losses);
    }
    city.heat_loss = rows;
    city.height = city.heat_loss.len() as Int;
    city.width = city.heat_loss[0].len() as Int;
    return city;
}

fn part1(root: &Parsed) {
    let mut city = root.clone();
    //city.run();
    //println!("{:#?}", city);
    //city.show();

    println!("Part 1: {}", city.get_optimal_v2(city.width - 1, city.height - 1));
}

fn part2(root: &Parsed) {
    println!("Part 2: {}", root.get_optimal_part2(root.width - 1, root.height - 1));
}

fn main() {
    let files = vec!["sample.txt" /*"sample2.txt" ,*/, "input.txt"];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content);
        part1(&root);
        part2(&root);
    }
}
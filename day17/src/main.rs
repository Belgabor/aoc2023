#![feature(let_chains)]

use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs;
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
            return CostVariant{from: self.from, length: self.length + 1};
        }

        CostVariant {from: direction.opposite(), length: 1}
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
}

#[derive(Debug, Default, Clone)]
struct City {
    heat_loss: Vec<Vec<Int>>,
    width: Int,
    height: Int,
    visited: HashSet<State>,
    costs: HashMap<State, Int>,
    //cost_variants: HashMap<Coordinate, Vec<CostVariant>>,
}

impl City {
    fn get_loss(&self, coordinate: &Coordinate) -> Int {
        self.heat_loss[coordinate.1 as usize][coordinate.0 as usize]
    }

    fn handle_direction(&mut self, current_state: &State, current_cost: &Int, direction: Direction) {
        let next = current_state.next(direction, self.width, self.height);
        if let Some(next) = next && !self.visited.contains(&next) {
            let next_cost: Int = current_cost + self.get_loss(&next.coordinate);
            if !self.costs.contains_key(&next) || next_cost < *self.costs.get(&next).unwrap() {
                self.costs.insert(next.clone(), next_cost);
            }
        }
        /*
        let coordinate = Coordinate(current_state.coordinate.0 + direction.dx(), current_state.coordinate.1 + direction.dy());
        let min_comp = match direction {
            Up => 0,
            Right => coordinate.0,
            Down => coordinate.1,
            Left => 0,
        };

        let max_comp = match direction {
            Up => coordinate.1,
            Right => self.width - 1,
            Down => self.height - 1,
            Left => coordinate.0
        };

        if min_comp <= max_comp && !self.visited.contains(&coordinate) && current_variant.can_go(direction) {
            let next_cost: Int = current_cost + self.get_loss(&coordinate);
            if !self.costs.contains_key(&coordinate) || next_cost < *self.costs.get(&coordinate).unwrap() {
                self.costs.insert(coordinate.clone(), next_cost);
                self.cost_variants.insert(coordinate, vec![current_variant.next(direction)]);
            } else if next_cost == *self.costs.get(&coordinate).unwrap() {
                let mut variants = self.cost_variants.get(&coordinate).unwrap().clone();
                variants.push(current_variant.next(direction));
                self.cost_variants.insert(coordinate, variants);
            }
        }
         */
    }

    fn get_next_state(&self) -> Option<State> {
        let mut min_cost: Option<Int> = None;
        let mut found = None;
        for (coordinate, cost) in self.costs.iter() {
            if self.visited.contains(coordinate) {
                continue
            }
            if min_cost.is_none() || *cost < min_cost.unwrap() {
                found = Some(coordinate.clone());
                min_cost = Some(*cost);
            }
        }

        found
    }

    fn run(&mut self) {
        let mut current_state = State{coordinate: Coordinate(0, 0), variant: CostVariant { from: Left, length: 0 }};
        self.costs.insert(current_state.clone(), 0);

        loop {
            let current_cost = self.costs.get(&current_state).unwrap().clone();

            self.handle_direction(&current_state, &current_cost, Up);
            self.handle_direction(&current_state, &current_cost, Right);
            self.handle_direction(&current_state, &current_cost, Down);
            self.handle_direction(&current_state, &current_cost, Left);

            self.visited.insert(current_state.clone());

            if current_state.coordinate.0 == self.width - 1 && current_state.coordinate.1 == self.height - 1 {
                break;
            }

            let next = self.get_next_state();
            if next.is_none() {
                break;
            }
            current_state = next.unwrap();
        }
    }

    fn get_optimal(&self, x: Int, y: Int) -> Int {
        // *self.costs.get(&Coordinate(self.width - 1, self.height -1)).unwrap()
        let mut minimum = Int::MAX;
        for state in self.costs.keys().filter(|state| state.coordinate.0 == x && state.coordinate.1 == y) {
            minimum = min(minimum, *self.costs.get(state).unwrap())
        }

        minimum
    }

    /*
    fn show(&self) {
        for y in 0..self.height {
            let mut a = String::new();
            let mut b = String::new();
            for x in 0..self.width {
                a.push_str(&self.heat_loss[y as usize][x as usize].to_string());
                if let Some(cost) = self.costs.get(&Coordinate(x, y)) {
                    a.push_str(format!("|{:04} ", cost).as_str());
                } else {
                    a.push_str("|----")
                }
            }
            println!("{} {}", a, b);
        }
    }
     */
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
    city.run();
    //println!("{:#?}", city);
    //city.show();

    println!("Part 1: {}", city.get_optimal(city.width - 1, city.height - 1));
}

fn part2(root: &Parsed) {
    println!("Part 2: {}", "TODO");
}

fn main() {
    let files = vec!["sample.txt" /*"sample2.txt" ,*/ ,"input.txt"];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content);
        part1(&root);
        part2(&root);
    }
}
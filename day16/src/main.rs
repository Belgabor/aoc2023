use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;
use enumset::{EnumSet, EnumSetType};
use crate::Cell::{MirrorLB, MirrorLT, SplitterH, SplitterV};
use crate::Direction::{Down, Left, Right, Up};

type Parsed = Grid;
type Int = isize;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
struct Coordinate(Int, Int);

#[derive(Debug)]
enum Cell {
    MirrorLB,
    MirrorLT,
    SplitterV,
    SplitterH,
}

impl Cell {
    fn parse(c: &char) -> Option<Self> {
        match c {
            '|' => Some(SplitterV),
            '-' => Some(SplitterH),
            '\\' => Some(MirrorLB),
            '/' => Some(MirrorLT),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
struct Grid {
    cells: HashMap<Coordinate, Cell>,
    width: Int,
    height: Int,
}

impl Grid {
    fn add(&mut self, x: &Int, y: &Int, cell: &char) -> Option<()> {
        self.width = max(x + 1, self.width);
        self.height = max(y + 1, self.height);

        let cell = Cell::parse(cell)?;
        self.cells.insert(Coordinate(*x, *y), cell);

        Some(())
    }

    fn get_energy(&self, beam: Beam) -> usize {
        let mut beams = vec![beam];
        let mut visited = Visited::default();

        loop {
            let beam = beams.pop();
            if beam.is_none() {
                break
            }
            let mut beam = beam.unwrap();
            let mut valid = true;
            while valid {
                if beam.valid(self) && visited.visit(&beam) {
                    break;
                }
                let mut new_beam;
                (new_beam, valid) = beam.next(self);
                if let Some(new_beam) = new_beam {
                    beams.push(new_beam);
                }
            }
        }

        visited.0.len()
    }
}

#[derive(Debug, Default)]
struct Visited(HashMap<Coordinate, EnumSet<Direction>>);

impl Visited {
    fn visit(&mut self, beam: &Beam) -> bool {
        let contains = self.0.get(&beam.position);

        if let Some(contains) = contains {
            if contains.contains(beam.direction) {
                return true;
            } else {
                let mut contains = contains.clone();
                contains.insert(beam.direction.clone());
                self.0.insert(beam.position.clone(), contains);
            }
        } else {
            let mut contains = EnumSet::new();
            contains.insert(beam.direction.clone());
            self.0.insert(beam.position.clone(), contains);
        }


        false
    }
}

#[derive(Debug, EnumSetType)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn apply(&self, cell: &Cell) -> (Direction, Option<Direction>) {
        match self {
            Up => match cell {
                MirrorLB => (Left, None),
                MirrorLT => (Right, None),
                SplitterV => (Up, None),
                SplitterH => (Left, Some(Right)),
            }
            Right => match cell {
                MirrorLB => (Down, None),
                MirrorLT => (Up, None),
                SplitterV => (Up, Some(Down)),
                SplitterH => (Right, None),
            }
            Down => match cell {
                MirrorLB => (Right, None),
                MirrorLT => (Left, None),
                SplitterV => (Down, None),
                SplitterH => (Right, Some(Left))
            }
            Left => match cell {
                MirrorLB => (Up, None),
                MirrorLT => (Down, None),
                SplitterV => (Down, Some(Up)),
                SplitterH => (Left, None),
            }
        }
    }
}

#[derive(Debug)]
struct Beam {
    position: Coordinate,
    direction: Direction,
}

impl Beam {
    fn next(&mut self, grid: &Grid) -> (Option<Beam>, bool) {
        self.position = match self.direction {
            Up => Coordinate(self.position.0, self.position.1 - 1),
            Right => Coordinate(self.position.0+1, self.position.1),
            Down => Coordinate(self.position.0, self.position.1 + 1),
            Left => Coordinate(self.position.0-1, self.position.1),
        };

        let el = grid.cells.get(&self.position);

        let mut splitted = None;
        if let Some(cell) = el {
            let (direction, splitted_direction) = self.direction.apply(cell);
            self.direction = direction;
            if let Some(direction) = splitted_direction {
                splitted = Some(Beam {position: self.position.clone(), direction})
            }
        }

        (splitted, self.valid(grid))
    }

    fn valid(&self, grid: &Grid) -> bool {
        (0 <= self.position.0 && self.position.0 < grid.width) && (0 <= self.position.1 && self.position.1 < grid.height)
    }
}

fn parse(content: &String) -> Parsed {
    let mut grid = Grid::default();
    for (y, line) in content.split("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.add(&(x as Int), &(y as Int), &c);
        }
    }
    return grid;
}

fn part1(root: &Parsed) {
    //println!("{:?}", root);

    let beam = Beam {position: Coordinate(-1, 0), direction: Right};

    println!("Part 1: {}", root.get_energy(beam));
}

fn part2(root: &Parsed) {
    let mut max_energy = 0;

    for y in 0..root.height {
        let beam = Beam {position: Coordinate(-1, y), direction: Right};
        max_energy = max(max_energy, root.get_energy(beam));
        let beam = Beam {position: Coordinate(root.width, y), direction: Left};
        max_energy = max(max_energy, root.get_energy(beam));
    }
    for x in 0..root.width {
        let beam = Beam {position: Coordinate(x, -1), direction: Down};
        max_energy = max(max_energy, root.get_energy(beam));
        let beam = Beam {position: Coordinate(x, root.height), direction: Up};
        max_energy = max(max_energy, root.get_energy(beam));
    }

    println!("Part 2: {}", max_energy);
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
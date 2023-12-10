#![feature(let_chains)]

use std::collections::{HashMap, HashSet};
use std::fs;
use enum_iterator::{all, Sequence};
use NodeType::{BottomLeft, BottomRight, Horizontal, Start, TopLeft, TopRight, Vertical};
use crate::Direction::{Left, Right, Top, Bottom};
use crate::Side::{Inside, Outside};

type Parsed = Maze;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
struct Coordinate(isize, isize);

impl Coordinate {
    fn next(&self, direction: Option<Direction>, width: isize, height: isize) -> Option<Coordinate> {
        let coordinate = match direction? {
            Left => Coordinate(self.0 - 1, self.1),
            Right => Coordinate(self.0 + 1, self.1),
            Top => Coordinate(self.0, self.1 - 1),
            Bottom => Coordinate(self.0, self.1 + 1),
        };
        if coordinate.0 < 0 || coordinate.0 >= width || coordinate.1 < 0 || coordinate.1 >= height {
            return None;
        }
        Some(coordinate)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
enum NodeType {
    #[default]
    Start,
    Horizontal,
    Vertical,
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

#[derive(Debug, Clone, Sequence)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Left => Right,
            Right => Left,
            Top => Bottom,
            Bottom => Top,
        }
    }
}

impl NodeType {
    fn parse(c: char) -> Option<NodeType> {
        match c {
            'S' => Some(Start),
            '|' => Some(Vertical),
            '-' => Some(Horizontal),
            '7' => Some(BottomLeft),
            'F' => Some(BottomRight),
            'J' => Some(TopLeft),
            'L' => Some(TopRight),
            _ => None,
        }
    }

    fn symbol(&self) -> char {
        match self {
            Start => 'S',
            Vertical => '┃',
            Horizontal => '━',
            BottomLeft => '┑',
            BottomRight => '┍',
            TopLeft => '┙',
            TopRight => '┕',
        }
    }

    fn determine(left_to: &Direction, arrived_from_going_towards: &Direction) -> Self {
        println!("{:?} {:?}", left_to, arrived_from_going_towards);
        match (left_to, arrived_from_going_towards) {
            (Left, Left) => Horizontal,
            (Left, Right) => unreachable!(),
            (Left, Top) => BottomLeft,
            (Left, Bottom) => TopLeft,
            (Right, Left) => unreachable!(),
            (Right, Right) => Horizontal,
            (Right, Top) => BottomRight,
            (Right, Bottom) => TopRight,
            (Top, Left) => TopRight,
            (Top, Right) => TopLeft,
            (Top, Top) => Vertical,
            (Top, Bottom) => unreachable!(),
            (Bottom, Left) => BottomRight,
            (Bottom, Right) => BottomLeft,
            (Bottom, Top) => unreachable!(),
            (Bottom, Bottom) => Vertical,
        }
    }

    fn next(&self, coming_from: &Direction) -> Option<Direction> {
        match self {
            Start => None,
            Horizontal => match coming_from {
                Top => None,
                Bottom => None,
                Left => Some(Right),
                Right => Some(Left),
            },
            Vertical => match coming_from {
                Left => None,
                Right => None,
                Top => Some(Bottom),
                Bottom => Some(Top),
            },
            BottomLeft => match coming_from {
                Left => Some(Bottom),
                Right => None,
                Top => None,
                Bottom => Some(Left),
            },
            BottomRight => match coming_from {
                Top => None,
                Bottom => Some(Right),
                Left => None,
                Right => Some(Bottom),
            },
            TopLeft => match coming_from {
                Top => Some(Left),
                Bottom => None,
                Left => Some(Top),
                Right => None
            },
            TopRight => match coming_from {
                Top => Some(Right),
                Bottom => None,
                Left => None,
                Right => Some(Top),
            },
        }
    }

    fn next_sided(&self, leaving_to: &Direction, sided: &Sides) -> Sides {
        let coming_from = leaving_to.opposite();
        //print!("{:?} {:?} {:?} ", self, coming_from, sided);
        let result = match self {
            Start => unreachable!(),
            Horizontal => match coming_from {
                Top => unreachable!(),
                Bottom => unreachable!(),
                Left => Sides {
                    tl: sided.tr,
                    tr: sided.tr,
                    bl: sided.br,
                    br: sided.br,
                },
                Right => Sides {
                    tl: sided.tl,
                    tr: sided.tl,
                    bl: sided.bl,
                    br: sided.bl,
                },
            },
            Vertical => match coming_from {
                Left => unreachable!(),
                Right => unreachable!(),
                Top => Sides {
                    tl: sided.bl,
                    tr: sided.br,
                    bl: sided.bl,
                    br: sided.br,
                },
                Bottom => Sides {
                    tl: sided.tl,
                    tr: sided.tr,
                    bl: sided.tl,
                    br: sided.tr,
                },
            },
            BottomLeft => match coming_from {
                Left => Sides {
                    tl: sided.tr,
                    tr: sided.tr,
                    bl: sided.br,
                    br: sided.tr,
                },
                Right => unreachable!(),
                Top => unreachable!(),
                Bottom => Sides {
                    tl: sided.tr,
                    tr: sided.tr,
                    bl: sided.tl,
                    br: sided.tr,
                },
            },
            BottomRight => match coming_from {
                Top => unreachable!(),
                Bottom => Sides {
                    tl: sided.tl,
                    tr: sided.tl,
                    bl: sided.tl,
                    br: sided.tr,
                },
                Left => unreachable!(),
                Right => Sides {
                    tl: sided.tl,
                    tr: sided.tl,
                    bl: sided.tl,
                    br: sided.bl,
                },
            },
            TopLeft => match coming_from {
                Top => Sides {
                    tl: sided.bl,
                    tr: sided.br,
                    bl: sided.br,
                    br: sided.br,
                },
                Bottom => unreachable!(),
                Left => Sides {
                    tl: sided.tr,
                    tr: sided.br,
                    bl: sided.br,
                    br: sided.br,
                },
                Right => unreachable!(),
            },
            TopRight => match coming_from {
                Top => Sides {
                    tl: sided.bl,
                    tr: sided.br,
                    bl: sided.bl,
                    br: sided.bl,
                },
                Bottom => unreachable!(),
                Left => unreachable!(),
                Right => Sides {
                    tl: sided.bl,
                    tr: sided.tl,
                    bl: sided.bl,
                    br: sided.bl,
                },
            },
        };

        //println!("-> {:?}", result);

        result
    }

    fn switches(&self, inside: bool, sides: &Sides) -> bool {
        match self {
            Start => unreachable!(),
            Horizontal => true,
            Vertical => false,
            BottomLeft => inside && sides.tl == Inside,
            BottomRight => inside && sides.tl == Inside,
            TopLeft => !inside && sides.bl == Inside,
            TopRight => !inside && sides.bl == Inside,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Side {
    Outside,
    Inside,
}

impl Side {
    fn opposite(&self) -> Side {
        match self {
            Outside => Inside,
            Inside => Outside,
        }
    }

    fn char(&self) -> char {
        match self {
            Outside => 'O',
            Inside => 'I',
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Sides {
    tl: Side,
    tr: Side,
    bl: Side,
    br: Side,
}

impl Sides {
    fn first(node_type: &NodeType) -> Sides {
        println!("Sides::first {:?}", node_type);
        Sides {
            tl: Outside,
            tr: if *node_type == BottomRight { Outside } else { Inside },
            bl: Outside,
            br: if *node_type == TopRight { Outside } else { Inside },
        }
    }

    fn char(&self) -> char {
        match (self.tl, self.tr, self.bl, self.br) {
            (Outside, Outside, Outside, Outside) => ' ',
            (Outside, Outside, Outside, Inside) => '▗',
            (Outside, Outside, Inside, Outside) => '▖',
            (Outside, Inside, Outside, Outside) => '▝',
            (Inside, Outside, Outside, Outside) => '▘',
            (Outside, Outside, Inside, Inside) => '▄',
            (Outside, Inside, Outside, Inside) => '▐',
            (Inside, Outside, Inside, Outside) => '▌',
            (Inside, Inside, Outside, Outside) => '▀',
            (Inside, Inside, Inside, Outside) => '▛',
            (Inside, Inside, Outside, Inside) => '▜',
            (Inside, Outside, Inside, Inside) => '▙',
            (Outside, Inside, Inside, Inside) => '▟',
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct LoopNode {
    coordinate: Coordinate,
    sides: Option<Sides>,
}


#[derive(Debug, Default)]
struct Maze {
    nodes: HashMap<Coordinate, Option<NodeType>>,
    start: Coordinate,
    start_type: NodeType,
    the_loop: HashMap<Coordinate, LoopNode>,
    width: isize,
    height: isize,
    loop_length: isize,
}

impl Maze {
    fn find_next(&self, node: &Coordinate, to: &Direction) -> Option<(Coordinate, Option<Direction>)> {
        let next = node.next(Some(to.clone()), self.width, self.height)?;
        let next_type = (*self.nodes.get(&next)?).clone()?;
        let direction = if next_type == Start { None } else { Some(next_type.next(&to.opposite())?) };
        Some((next, direction))
    }

    fn determine_start_type(&mut self) {
        for direction in all::<Direction>() {
            let mut current = self.start.clone();
            let mut length = 0;
            let mut next = direction.clone();
            let mut the_loop = HashMap::new();
            //println!("--------------------------------");
            //println!("{:?} {:?}", current, next);
            loop {
                the_loop.insert(current.clone(), LoopNode { coordinate: current.clone(), sides: None });
                let next_step = self.find_next(&current, &next);
                //println!("{:?}", next_step);
                if next_step.is_none() {
                    break;
                }
                length += 1;
                let next_data = next_step.unwrap();
                current = next_data.0;
                if current == self.start {
                    self.start_type = NodeType::determine(&direction, &next);
                    self.nodes.insert(self.start.clone(), Some(self.start_type.clone()));
                    self.loop_length = length;
                    self.the_loop = the_loop;
                    return;
                }
                next = next_data.1.unwrap();
            }
        }
        unreachable!()
    }

    fn magnetize_loop(&mut self) {
        let y = self.height / 2;
        let mut first_ = None;
        for x in 0..self.width {
            first_ = self.the_loop.get_mut(&Coordinate(x, y));
            if first_.is_some() {
                break;
            }
        }
        let first = first_.unwrap();
        first.sides = Some(Sides::first(&self.nodes.get(&first.coordinate).unwrap().clone().unwrap()));
        let mut current = first.clone();
        let mut node_type = (*self.nodes.get(&current.coordinate).unwrap()).clone().unwrap();
        let mut next_direction = node_type.next(&Top).or_else(|| node_type.next(&Right)).unwrap();
        // println!("First: {:?} {:?} {:?} {:?} {:?}", first.coordinate, node_type, next_direction, first.sides, self.start_type);
        loop {
            let next = self.find_next(&current.coordinate, &next_direction).unwrap();
            let next_node = self.the_loop.get_mut(&next.0).unwrap();
            if next_node.sides.is_some() {
                break;
            }
            node_type = (*self.nodes.get(&next.0).unwrap()).clone().unwrap();
            next_node.sides = Some(node_type.next_sided(&next_direction, &current.sides.unwrap()));
            // println!("{:?} {:?} {:?}", next.0, node_type.symbol(), next_node.sides);

            current = next_node.clone();
            next_direction = next.1.unwrap();
        }
    }

    fn calculate_area(&self) -> usize {
        let mut area = 0;
        let mut inside = false;
        let mut parts = HashSet::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let coordinate = Coordinate(x, y);
                let node = self.the_loop.get(&coordinate);
                if let Some(node) = node {
                    let node_type = (*self.nodes.get(&coordinate).unwrap()).clone().unwrap();
                    let switches = node_type.switches(inside, &node.sides.clone().unwrap());
                    if x == 0 {
                        println!("{} {} {:?} {:?} {} {}", x, y, node_type, node.sides, inside, switches);
                    }
                    if switches {
                        inside = !inside;
                    }
                } else {
                    if inside {
                        area += 1;
                        parts.insert(coordinate);
                    }
                }
            }
        }

        println!("----------------------------------------------------------------");
        for y in 0..self.height {
            let mut line = "".to_string();
            let mut line2 = "".to_string();
            for x in 0..self.width {
                let c = Coordinate(x, y);
                if self.the_loop.contains_key(&c) {
                    let el = (*self.nodes.get(&c).unwrap()).clone().unwrap();
                    line.push(el.symbol());
                    line2.push(self.the_loop.get(&c).unwrap().sides.clone().unwrap().char());
                } else {
                    line2.push('.');
                    if parts.contains(&c) {
                        line.push('*');
                    } else {
                        line.push('.');
                    }
                }
            }
            println!("{} {}", line, line2);
        }

        area
    }
}

fn parse(content: &String) -> Parsed {
    let mut maze = Maze::default();
    let mut y = 0;
    let mut start = None;
    for line in content.split("\n") {
        let mut x = 0;
        for c in line.chars() {
            let node_type = NodeType::parse(c);
            let coordinate = Coordinate(x, y);
            if let Some(node) = &node_type && *node == Start {
                start = Some(coordinate.clone());
            }
            maze.nodes.insert(coordinate, node_type);
            x += 1;
        }
        maze.width = x;
        y += 1;
    }
    maze.start = start.unwrap();
    println!("Start: {:?}", maze.start);
    maze.height = y;

    maze.determine_start_type();
    maze.magnetize_loop();

    return maze;
}

fn part1(root: &Parsed) {
    //println!("{:?}", root);
    println!("Part 1: {}", root.loop_length / 2);
}

fn part2(root: &Parsed) {
    //println!("{:#?}", root.the_loop);
    println!("Part 2: {}", root.calculate_area());
}

fn main() {
    let files = vec!["sample.txt", "sample2.txt", "input.txt"];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content);
        part1(&root);
        part2(&root);
    }
}
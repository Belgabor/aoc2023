use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs;
use glam::I64Vec2;

use miette::Result;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::{self, char, line_ending, one_of, space1};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};

use tools::{Direction, NodeType};
use tools::Direction::{Down, Left, Right, Up};

use crate::custom_error::AocError;

pub mod custom_error;

type Parsed = Instructions;
type AocResult = u64;
type AocResult2 = AocResult;
type Int = i64;

#[derive(Debug,PartialEq)]
pub struct Color {
    pub red:   u8,
    pub green: u8,
    pub blue:  u8,
}

fn from_hex(input: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_direction(input: &str) -> IResult<&str, Direction> {
    let (input, direction) = one_of("0123")(input)?;

    let direction= match direction {
        '0' => Right,
        '2' => Left,
        '3' => Up,
        '1' => Down,
        _ => unreachable!(),
    };

    Ok((input, direction))
}


fn hex_distance(input: &str) -> IResult<&str, u64> {
    map_res(
        take_while_m_n(5, 5, is_hex_digit),
        from_hex
    )(input)
}

fn hex_color(input: &str) -> IResult<&str, AltInstruction> {
    let (input, _) = tag("#")(input)?;
    let (input, (distance, direction)) = tuple((hex_distance, hex_direction))(input)?;

    Ok((input, AltInstruction{distance: distance as Int, direction}))
}


#[derive(Debug)]
struct AltInstruction {
    direction: Direction,
    distance: Int,
}

impl AltInstruction {
    fn deltas(&self) -> I64Vec2 {
        match self.direction {
            Up => I64Vec2::new(0, -self.distance),
            Right => I64Vec2::new(self.distance, 0),
            Down => I64Vec2::new(0, self.distance),
            Left => I64Vec2::new(-self.distance, 0),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    wrong: AltInstruction,
    color: AltInstruction,
}

#[derive(Debug)]
struct Instructions(Vec<Instruction>);

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, direction) = one_of("RLDU")(input)?;
    let (input, _) = space1(input)?;
    let (input, distance) = complete::u8(input)?;
    let (input, _) = space1(input)?;
    let (input, color)= delimited(char('('), hex_color, char(')'))(input)?;

    let wrong = AltInstruction{
        direction: match direction {
            'R' => Right,
            'L' => Left,
            'U' => Up,
            'D' => Down,
            _ => unreachable!(),
        },
        distance: distance as Int,
    };

    Ok((input, Instruction{
        wrong,
        color,
    }))
}

fn parse_instructions(input: &str) -> IResult<&str, Instructions> {
    let (input, instructions) = separated_list1(line_ending, instruction)(input)?;
    Ok((input, Instructions(instructions)))
}

fn parse(content: &String) -> Parsed {
    return parse_instructions(content).unwrap().1;
}


#[derive(Debug)]
struct DugPath {
    from: I64Vec2,
    to: I64Vec2,
    horizontal: bool,
}

#[derive(Debug)]
struct DugPaths(Vec<DugPath>);


#[derive(Debug, Default)]
struct Board {
    blocks: HashMap<I64Vec2, NodeType>,
    left: Int,
    right: Int,
    top: Int,
    bottom: Int,
}

impl Board {
    fn dig(&mut self, instructions: &Instructions, wrong: bool) {
        self.left = Int::MAX;
        self.top = Int::MAX;

        let first = instructions.0.first().unwrap();
        let first = if wrong {&first.wrong} else {&first.color};
        let last = instructions.0.last().unwrap();
        let last = if wrong {&last.wrong} else {&last.color};

        let mut position = I64Vec2::new(0, 0);
        let mut last_direction: Option<Direction> = None;

        for instruction_ in instructions.0.iter() {
            let instruction = if wrong {&instruction_.wrong} else {&instruction_.color};
            if let Some(last_direction) = last_direction {
                self.blocks.insert(position.clone(), NodeType::new(&last_direction.opposite(), &instruction.direction));
            }
            let delta: I64Vec2 = instruction.direction.delta().into();
            for _ in 0..instruction.distance {
                position += delta;
                self.blocks.insert(position.clone(), NodeType::route(&instruction.direction));
                self.left = min(self.left, position.x as Int);
                self.right = max(self.right, position.x as Int);
                self.top = min(self.top, position.y as Int);
                self.bottom = max(self.bottom, position.y as Int);
            }
            last_direction = Some(instruction.direction.clone());
        }
        self.blocks.insert(I64Vec2::new(0, 0), NodeType::new(&last.direction.opposite(), &first.direction));
    }

    fn paths(&mut self, instructions: &Instructions, wrong: bool) -> Vec<DugPath> {
        self.left = Int::MAX;
        self.top = Int::MAX;

        let mut position = I64Vec2::new(0, 0);
        let mut paths = Vec::new();

        for instruction_ in instructions.0.iter() {
            let instruction = &instruction_.color;
            let horizontal = instruction.direction == Left || instruction.direction == Right;
        }
        paths
    }

    fn filled(&mut self) -> AocResult {
        let mut filled = 0;
        for y in self.top..=self.bottom {
            let mut inside = false;
            let mut last_down = None;
            for x in self.left..=self.right {
                let position = I64Vec2::new(x as i64, y as i64);
                if let Some(node) = self.blocks.get(&position) {
                    //inside = !inside;
                    filled += 1;
                    match node {
                        NodeType::Horizontal => (),
                        NodeType::Vertical => {
                            inside = !inside;
                        }
                        NodeType::BottomRight => {
                            last_down = Some(true);
                        }
                        NodeType::BottomLeft => {
                            if !last_down.unwrap() {
                                inside = !inside;
                            }
                        }
                        NodeType::TopLeft => {
                            if last_down.unwrap() {
                                inside = !inside;
                            }
                        }
                        NodeType::TopRight => {
                            last_down = Some(false);
                        }
                    }
                } else if inside {
                    filled += 1;
                }
            }
        }

        filled
    }

    /*
    fn draw(&self) {
        for y in self.top..=self.bottom {
            let mut line = String::new();
            for x in self.left..=self.right {
                let position = I64Vec2::new(x, y);
                if self.blocks.contains_key(&position) {
                    line.push(self.blocks.get(&position).unwrap().symbol());
                } else if self.filled.contains(&position) {
                    line.push('O');
                } else {
                    line.push('.');
                }
            }
            println!("{}", line);
        }
    }
     */
}

fn part1(root: &Parsed) -> Result<AocResult, AocError> {
    println!("{:#?}", root);
    let mut board = Board::default();
    board.dig(root, true);
    // println!("{:#?}", board);
    //board.fill();
    //board.draw();

    Ok(board.filled())
}

fn part2(root: &Parsed) -> Result<AocResult2, AocError> {
    let mut board = Board::default();
    println!("Digging...");
    board.dig(root, false);

    println!("Filling...");
    Ok(board.filled())
}

fn main() -> Result<(), AocError> {
    let files = vec![ "input.txt" ];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content);
        println!("Part 1: {}", part1(&root)?);
        println!("Part 2: {}", part2(&root)?);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn tests() -> miette::Result<()> {
        let file = "sample.txt";
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = crate::parse(&content);
        assert_eq!(62, crate::part1(&root)?);
        //assert_eq!(952408144115, crate::part2(&root)?);

        Ok(())
    }
}
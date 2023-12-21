use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs;
use glam::IVec2;

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
type AocResult = usize;
type AocResult2 = AocResult;
type Int = isize;

#[derive(Debug,PartialEq)]
pub struct Color {
    pub red:   u8,
    pub green: u8,
    pub blue:  u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, is_hex_digit),
        from_hex
    )(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: Int,
    color: Color,
}

#[derive(Debug)]
struct Instructions(Vec<Instruction>);

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, direction) = one_of("RLDU")(input)?;
    let (input, _) = space1(input)?;
    let (input, distance) = complete::u8(input)?;
    let (input, _) = space1(input)?;
    let (input, color)= delimited(char('('), hex_color, char(')'))(input)?;

    Ok((input, Instruction{
        direction: match direction {
            'R' => Right,
            'L' => Left,
            'U' => Up,
            'D' => Down,
            _ => unreachable!(),
        },
        distance: distance as Int,
        color
    }))
}

fn parse_instructions(input: &str) -> IResult<&str, Instructions> {
    let (input, instructions) = separated_list1(line_ending, instruction)(input)?;
    Ok((input, Instructions(instructions)))
}

fn parse(content: &String) -> Parsed {
    return parse_instructions(content).unwrap().1;
}




#[derive(Debug, Default)]
struct Board {
    blocks: HashMap<IVec2, NodeType>,
    filled: HashSet<IVec2>,
    left: Int,
    right: Int,
    top: Int,
    bottom: Int,
}

impl Board {
    fn dig(&mut self, instructions: &Instructions) {
        self.left = Int::MAX;
        self.top = Int::MAX;

        let first = instructions.0.first().unwrap();
        let last = instructions.0.last().unwrap();

        let mut position = IVec2::new(0, 0);
        let mut last_direction: Option<Direction> = None;

        for instruction in instructions.0.iter() {
            if let Some(last_direction) = last_direction {
                self.blocks.insert(position.clone(), NodeType::new(&last_direction.opposite(), &instruction.direction));
            }
            let delta = instruction.direction.delta();
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
        self.blocks.insert(IVec2::new(0, 0), NodeType::new(&last.direction.opposite(), &first.direction));
    }

    fn fill(&mut self) {
        for y in self.top..=self.bottom {
            let mut inside = false;
            let mut last_down = None;
            for x in self.left..=self.right {
                let position = IVec2::new(x as i32, y as i32);
                if let Some(node) = self.blocks.get(&position) {
                    //inside = !inside;
                    self.filled.insert(position);
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
                    self.filled.insert(position);
                }
            }
        }
    }

    fn draw(&self) {
        for y in self.top..=self.bottom {
            let mut line = String::new();
            for x in self.left..=self.right {
                let position = IVec2::new(x as i32, y as i32);
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
}

fn part1(root: &Parsed) -> Result<AocResult, AocError> {
    // println!("{:#?}", root);
    let mut board = Board::default();
    board.dig(root);
    // println!("{:#?}", board);
    board.fill();
    board.draw();

    Ok(board.filled.len())
}

fn part2(root: &Parsed) -> Result<AocResult2, AocError> {
    todo!("Implement Part 2");
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
        assert_eq!(62, crate::part2(&root)?);

        Ok(())
    }
}
use std::fs;

use crate::custom_error::AocError;
use miette::Result;
pub mod custom_error;

type Parsed = String;
type AocResult = Int;
type AocResult2 = AocResult;
type Int = usize;

fn parse(content: &String) -> Parsed {
    for line in content.split("\n") {
        todo!()
    }
    return content.clone();
}

fn part1(root: &Parsed) -> Result<AocResult, AocError> {
    println!("{:?}", root);
    todo!("Implement Part 1");
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
        assert_eq!("", crate::part1(&root)?);
        assert_eq!("", crate::part2(&root)?);

        Ok(())
    }
}
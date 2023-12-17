use std::collections::HashSet;
use std::fs;
use Orientation::{Horizontal, Vertical};
use crate::IsMirrored::{No, Smudged, Yes};

type Parsed = Patterns;
type Int = isize;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Mirrored {
    orientation: Orientation,
    index: Int,
}

impl Mirrored {
    fn score(&self) -> Int {
        match *self {
            Mirrored { orientation: Vertical, index: i } => i,
            Mirrored { orientation: Horizontal, index: i } => i * 100,
        }
    }
}

#[derive(Debug)]
struct Patterns(Vec<Pattern>);

impl Patterns {
    pub fn get_score(&self) -> Int {
        self.0.iter().map(|p| p.mirrored().score()).sum()
    }
    pub fn get_smudged_score(&self) -> Int {
        self.0.iter().map(|p| p.mirrored_smudged().score()).sum()
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
struct Coordinate(Int, Int);

#[derive(Debug, PartialEq, Eq, Clone)]
enum IsMirrored {
    No,
    Yes,
    Smudged,
}

#[derive(Debug)]
struct Pattern {
    grid: HashSet<Coordinate>,
    width: Int,
    height: Int,
}

impl Pattern {
    fn parse(content: &str) -> Pattern {
        let lines: Vec<_> = content.split("\n").collect();
        let width = lines[0].len() as Int;
        let height = lines.len() as Int;
        let mut grid = HashSet::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    grid.insert(Coordinate(x as isize, y as isize));
                }
            }
        }

        Pattern { grid, width, height }
    }

    fn is_row_mirrored_at(&self, row: Int, x: Int) -> IsMirrored {
        let mut state = Yes;
        for offset in 0..self.width {
            let left = x - offset;
            let right = x + offset + 1;
            if left < 0 || right >= self.width {
                break;
            }

            let is_left = self.grid.contains(&Coordinate(left, row));
            let is_right = self.grid.contains(&Coordinate(right, row));
            if is_left != is_right {
                state = match state {
                    No => unreachable!(),
                    Yes => Smudged,
                    Smudged => No,
                };
                if state == No {
                    break;
                }
            }
        }

        state
    }

    fn get_vertical_mirror_score(&self) -> Option<Int> {
        'outer: for x in 0..self.width - 1 {
            if self.is_row_mirrored_at(0, x) == Yes {
                for y in 1..self.height {
                    if self.is_row_mirrored_at(y, x) != Yes {
                        continue 'outer;
                    }
                }
                // println!("X {}", x);
                return Some(x + 1);
            }
        }

        None
    }

    fn get_vertical_mirror_scores(&self) -> Vec<Int> {
        let mut result = Vec::new();
        'outer: for x in 0..self.width - 1 {
            let mut base_result = self.is_row_mirrored_at(0, x);
            if base_result != No {
                for y in 1..self.height {
                    let iter_result = self.is_row_mirrored_at(y, x);
                    base_result = match iter_result {
                        No => No,
                        Yes => base_result,
                        Smudged => if base_result != Yes { No } else { Smudged },
                    };
                    if base_result == No {
                        continue 'outer;
                    }
                }
                // println!("X {}", x);
                result.push(x + 1);
            }
        }

        result
    }

    fn is_col_mirrored_at(&self, col: Int, y: Int) -> IsMirrored {
        let mut state = Yes;
        for offset in 0..self.height {
            let top = y - offset;
            let bottom = y + offset + 1;
            if top < 0 || bottom >= self.height {
                break;
            }

            let is_top = self.grid.contains(&Coordinate(col, top));
            let is_bottom = self.grid.contains(&Coordinate(col, bottom));
            if is_top != is_bottom {
                state = match state {
                    No => unreachable!(),
                    Yes => Smudged,
                    Smudged => No,
                };
                if state == No {
                    break;
                }
            }
        }

        state
    }

    fn get_horizontal_mirror_score(&self) -> Option<Int> {
        'outer: for y in 0..self.height - 1 {
            if self.is_col_mirrored_at(0, y) == Yes {
                for x in 1..self.width {
                    if self.is_col_mirrored_at(x, y) != Yes {
                        continue 'outer;
                    }
                }
                // println!("Y {}", y);
                return Some(y + 1);
            }
        }

        None
    }

    fn get_horizontal_mirror_scores(&self) -> Vec<Int> {
        let mut result = Vec::new();
        'outer: for y in 0..self.height - 1 {
            let mut base_result = self.is_col_mirrored_at(0, y);
            if base_result != No {
                for x in 1..self.width {
                    let iter_result = self.is_col_mirrored_at(x, y);
                    base_result = match iter_result {
                        No => No,
                        Yes => base_result,
                        Smudged => if base_result != Yes { No } else { Smudged },
                    };
                    if base_result == No {
                        continue 'outer;
                    }
                }
                // println!("X {}", x);
                result.push(y + 1);
            }
        }

        result
    }

    fn mirrored(&self) -> Mirrored {
        self.get_vertical_mirror_score()
            .map(|i| Mirrored { orientation: Vertical, index: i })
            .or_else(|| self.get_horizontal_mirror_score().map(|x| Mirrored { orientation: Horizontal, index: x }))
            .unwrap()
    }

    fn mirrored_smudged(&self) -> Mirrored {
        let unsmudged = self.mirrored();

        self.get_vertical_mirror_scores()
            .iter()
            .map(|i| Mirrored { orientation: Vertical, index: *i })
            .filter(|el| *el != unsmudged)
            .collect::<Vec<_>>()
            .first()
            .map(|x| x.clone())
            .or_else(|| self.get_horizontal_mirror_scores()
                .iter()
                .map(|i| Mirrored { orientation: Horizontal, index: *i })
                .filter(|el| *el != unsmudged)
                .collect::<Vec<_>>()
                .first()
                .map(|x| x.clone())
            )
            .unwrap()

    }

}

fn parse(content: &String) -> Parsed {
    let mut patterns = Vec::new();
    for pattern in content.split("\n\n") {
        patterns.push(Pattern::parse(pattern));
    }
    return Patterns(patterns);
}

fn part1(root: &Parsed) {
    //println!("{:?}", root);
    println!("Part 1: {}", root.get_score());
}

fn part2(root: &Parsed) {
    println!("Part 2: {}", root.get_smudged_score());
}

fn main() {
    let files = vec!["sample.txt" /*"sample2.txt" ,*/ , "input.txt"];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content);
        part1(&root);
        part2(&root);
    }
}
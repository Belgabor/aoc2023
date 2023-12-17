use std::collections::HashSet;
use std::fs;

type Parsed = Galaxy;
type Int = i128;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Star(Int, Int);

impl Star {
    fn distance_to(&self, other: &Star) -> Int {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

#[derive(Debug, Default, Clone)]
struct Galaxy {
    stars: HashSet<Star>,
    width: Int,
    height: Int,
}

impl Galaxy {
    fn expand(&mut self, by: Int) {
        let mut columns = Vec::new();
        for x in 0..self.width {
            let mut empty = true;
            for y in 0..self.height {
                if self.stars.contains(&Star(x, y)) {
                    empty = false;
                    break;
                }
            };
            if empty {
                columns.push(x);
            }
        }

        let mut rows = Vec::new();
        for y in 0..self.height {
            let mut empty = true;
            for x in 0..self.width {
                if self.stars.contains(&Star(x, y)) {
                    empty = false;
                    break;
                }
            };
            if empty {
                rows.push(y);
            }
        }

        for x in columns.iter().rev() {
            let mut new_stars = HashSet::new();
            for star in self.stars.iter() {
                if star.0 > *x {
                    new_stars.insert(Star(star.0 + by, star.1));
                } else {
                    new_stars.insert(star.clone());
                }
            }
            self.stars = new_stars;
            self.width += 1;
        }

        for y in rows.iter().rev() {
            let mut new_stars = HashSet::new();
            for star in self.stars.iter() {
                if star.1 > *y {
                    new_stars.insert(Star(star.0, star.1 + by));
                } else {
                    new_stars.insert(star.clone());
                }
            }
            self.stars = new_stars;
            self.height += 1;
        }
    }

    fn sum_distances(&self) -> Int {
        let mut stars: Vec<Star> = self.stars.iter().map(|s| s.clone()).collect();
        let mut sum = 0;

        for i in 0..stars.len() {
            for j in i+1..stars.len() {
                sum += stars[i].distance_to(&stars[j]);
            }
        }

        sum
    }
}

fn parse(content: &String) -> Parsed {
    let mut y = 0;
    let mut galaxy = Galaxy::default();
    for line in content.split("\n") {
        let mut x = 0;
        for char in line.chars() {
            if char == '#' {
                galaxy.stars.insert(Star(x, y));
            }
            x = x + 1;
        }
        galaxy.width = x;
        y = y + 1;
    }
    galaxy.height = y;
    return galaxy;
}

fn part1(root: &Parsed) {
    //println!("{:?}", root);
    let mut galaxy = root.clone();
    galaxy.expand(1);
    //println!("{:?}", galaxy);
    println!("Part 1: {}", galaxy.sum_distances());
}

fn part2(root: &Parsed) {
    let mut galaxy = root.clone();
    galaxy.expand(999999);
    println!("Part 2: {}", galaxy.sum_distances());
}

fn main() {
    let files = vec!["sample.txt", /*"sample2.txt" ,*/ "input.txt"];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content);
        part1(&root);
        part2(&root);
    }
}
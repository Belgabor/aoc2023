use std::collections::HashMap;
use std::fs;

use crate::custom_error::AocError;
use miette::Result;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete;
use nom::character::complete::{alpha1, char, line_ending, one_of};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::delimited;

pub mod custom_error;

type Parsed = System;
type AocResult = Int;
type AocResult2 = AocResult;
type Int = u128;

#[derive(Debug, Default)]
struct Part {
    x: Int,
    m: Int,
    a: Int,
    s: Int,
}

#[derive(Debug, Clone)]
struct PartRange {
    x_min: Int,
    m_min: Int,
    a_min: Int,
    s_min: Int,
    x_max: Int,
    m_max: Int,
    a_max: Int,
    s_max: Int,
}

impl Part {
    fn set(&mut self, category: &Category, value: Int) {
        match category {
            Category::X => {
                self.x = value;
            }
            Category::M => {
                self.m = value;
            }
            Category::A => {
                self.a = value;
            }
            Category::S => {
                self.s = value;
            }
        }
    }

    fn get(&self, category: &Category) -> Int {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn score(&self) -> Int {
        self.x + self.m + self.a + self.s
    }
}

impl PartRange {
    fn get(&self, category: &Category) -> (Int, Int) {
        match category {
            Category::X => (self.x_min, self.x_max),
            Category::M => (self.m_min, self.m_max),
            Category::A => (self.a_min, self.a_max),
            Category::S => (self.s_min, self.s_max),
        }
    }

    fn split(&self, category: &Category, value: Int) -> (PartRange, PartRange) {
        let mut left = (*self).clone();
        let mut right = (*self).clone();
        match category {
            Category::X => {
                left.x_max = value - 1;
                right.x_min = value;
            }
            Category::M => {
                left.m_max = value - 1;
                right.m_min = value;
            }
            Category::A => {
                left.a_max = value - 1;
                right.a_min = value;
            }
            Category::S => {
                left.s_max = value - 1;
                right.s_min = value;
            }
        }
        (left, right)
    }

    fn valid(&self) -> bool {
        self.x_min <= self.x_max
            && self.m_min <= self.m_max
            && self.a_min <= self.a_max
            && self.s_min <= self.s_max
            && self.x_max <= 4000
            && self.m_max <= 4000
            && self.a_max <= 4000
            && self.s_max <= 4000
    }

    fn combinations(&self) -> Int {
        (self.x_max + 1 - self.x_min) * (self.m_max + 1 - self.m_min) * (self.a_max + 1 - self.a_min) * (self.s_max + 1 - self.s_min)
    }
}

#[derive(Debug, Clone)]
enum WorkflowTarget {
    Accept,
    Reject,
    Goto(String),
}

#[derive(Debug, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone)]
struct WorkflowCondition {
    less_than: bool,
    category: Category,
    value: Int,
    target: WorkflowTarget,
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    conditions: Vec<WorkflowCondition>,
    target: WorkflowTarget,
}

#[derive(Debug, Default)]
struct System {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl WorkflowCondition {
    fn run(&self, part: &Part) -> Option<WorkflowTarget> {
        let value = part.get(&self.category);
        if self.less_than && value < self.value {
            return Some(self.target.clone());
        }
        if !self.less_than && value > self.value {
            return Some(self.target.clone());
        }

        None
    }

    fn ranges(&self, range: &PartRange) -> (Option<PartRange>, Option<(WorkflowTarget, PartRange)>) {
        let mut modified = None;
        let mut redirect = None;
        let (minimum, maximum) = range.get(&self.category);
        if self.less_than {
            if minimum >= self.value {
                modified = Some((*range).clone())
            } else if maximum < self.value {
                redirect = Some((self.target.clone(), (*range).clone()))
            } else {
                let (left, right) = range.split(&self.category, self.value);
                if left.valid() {
                    redirect = Some((self.target.clone(), left))
                }
                if right.valid() {
                    modified = Some(right)
                }
            }
        } else {
            if maximum <= self.value {
                modified = Some((*range).clone())
            } else if minimum > self.value {
                redirect = Some((self.target.clone(), (*range).clone()))
            } else {
                let (left, right) = range.split(&self.category, self.value + 1);
                if right.valid() {
                    redirect = Some((self.target.clone(), right))
                }
                if left.valid() {
                    modified = Some(left)
                }
            }
        }

        (modified, redirect)
    }
}

impl Workflow {
    fn run(&self, part: &Part) -> WorkflowTarget {
        for condition in self.conditions.iter() {
            if let Some(target) = condition.run(part) {
                return target;
            }
        }

        self.target.clone()
    }
}

impl System {
    fn check_part(&self, part: &Part) -> bool {
        let mut workflow = self.workflows.get("in").unwrap();

        loop {
            let target = workflow.run(part);
            match target {
                WorkflowTarget::Accept => {
                    return true;
                }
                WorkflowTarget::Reject => {
                    return false;
                }
                WorkflowTarget::Goto(new_workflow) => {
                    workflow = self.workflows.get(&new_workflow).unwrap();
                }
            }
        }
    }

    fn score(&self) -> Int {
        self.parts
            .iter()
            .filter(|part| self.check_part(part))
            .map(|part| part.score())
            .sum()
    }

    fn run_though_workflow(&self, workflow: &str, range: PartRange) -> Vec<PartRange> {
        let workflow = self.workflows.get(workflow).unwrap();
        let mut ranges = Vec::new();
        let mut range = range.clone();
        for condition in workflow.conditions.iter() {
            let (new_range, redirect) = condition.ranges(&range);
            if let Some((target, target_range)) = redirect {
                match target {
                    WorkflowTarget::Accept => {
                        ranges.push(target_range);
                    }
                    WorkflowTarget::Reject => {
                        // Pass
                    }
                    WorkflowTarget::Goto(next_workflow) => {
                        let mut new = self.run_though_workflow(&next_workflow, target_range).clone();
                        ranges.append(&mut new);
                    }
                }
            }
            if new_range.is_none() {
                return ranges;
            }
            range = new_range.unwrap();
        }

        if range.valid() {
            match &workflow.target {
                WorkflowTarget::Accept => {
                    ranges.push(range);
                }
                WorkflowTarget::Reject => {
                    // Pass
                }
                WorkflowTarget::Goto(next_workflow) => {
                    let mut new = self.run_though_workflow(next_workflow, range).clone();
                    ranges.append(&mut new);
                }
            }
        }

        ranges
    }

    fn ranges(&self) -> Int {
        let range = PartRange {
            x_min: 1,
            m_min: 1,
            a_min: 1,
            s_min: 1,
            x_max: 4000,
            m_max: 4000,
            a_max: 4000,
            s_max: 4000,
        };

        let ranges = self.run_though_workflow("in", range);

        //println!("{:#?}", ranges);

        ranges.iter()
            .map(|range| range.combinations())
            .sum()
    }
}

fn sub_part(input: &str) -> IResult<&str, (Category, Int)> {
    let (input, category) = category(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, value) = complete::u32(input)?;
    Ok((input, (category, value.into())))
}

fn sub_parts(input: &str) -> IResult<&str, Vec<(Category, Int)>> {
    separated_list1(char(','), sub_part)(input)
}


fn part(input: &str) -> IResult<&str, Part> {
    let (input, parts) = delimited(char('{'), sub_parts, char('}'))(input)?;
    let mut part = Part::default();
    for (category, value) in parts {
        part.set(&category, value);
    }
    Ok((input, part))
}

fn parse_parts(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list1(line_ending, part)(input)
}


fn target(input: &str) -> IResult<&str, WorkflowTarget> {
    let (input, parsed) = alt((tag("A"), tag("R"), alpha1))(input)?;
    Ok((input, match parsed {
        "A" => WorkflowTarget::Accept,
        "R" => WorkflowTarget::Reject,
        workflow => WorkflowTarget::Goto(workflow.to_string()),
    }))
}

fn category(input: &str) -> IResult<&str, Category> {
    let (input, category) = one_of("xmas")(input)?;
    Ok((input, match category {
        'x' => Category::X,
        'm' => Category::M,
        'a' => Category::A,
        's' => Category::S,
        _ => unreachable!(),
    }))
}

fn condition(input: &str) -> IResult<&str, WorkflowCondition> {
    let (input, category) = category(input)?;
    let (input, comparator) = one_of("<>")(input)?;
    let (input, value) = complete::u128(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, target) = target(input)?;
    Ok((input, WorkflowCondition { category, target, value, less_than: comparator == '<' }))
}

fn conditions(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(char(','), is_not(",}"))(input)
}

fn workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = alpha1(input)?;
    let (input, steps) = delimited(char('{'), conditions, char('}'))(input)?;
    let mut steps = steps.clone();

    let mut conditions = Vec::new();

    let last = steps.pop().unwrap();
    let (_, target) = target(last)?;
    for step in steps {
        let (_, condition) = condition(step)?;
        conditions.push(condition);
    }

    Ok((input, Workflow { name: name.to_string(), conditions, target }))
}

fn parse_workflows(input: &str) -> IResult<&str, Vec<Workflow>> {
    separated_list1(line_ending, workflow)(input)
}

fn parse(content: &String) -> Parsed {
    let (workflows, parts) = content.split_once("\n\n").unwrap();

    let (_, workflows) = parse_workflows(workflows).unwrap();
    let workflows = workflows.iter()
        .map(|w| (w.name.clone(), w.clone()))
        .collect::<HashMap<_, _>>()
        ;

    let (_, parts) = parse_parts(parts).unwrap();

    return System { workflows, parts };
}

fn part1(root: &Parsed) -> Result<AocResult, AocError> {
    //println!("{:#?}", root);

    Ok(root.score())
}

fn part2(root: &Parsed) -> Result<AocResult2, AocError> {

    Ok(root.ranges())
}

fn main() -> Result<(), AocError> {
    let files = vec!["input.txt"];
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
        assert_eq!(19114, crate::part1(&root)?, "Part 1");
        assert_eq!(167409079868000, crate::part2(&root)?, "Part 2");

        Ok(())
    }
}
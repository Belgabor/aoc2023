use std::fs;

type Parsed = Races;

#[derive(Debug)]
struct Race {
    time: u128,
    distance: u128,
}

impl Race {
    fn run(&self) -> u128 {
        let mut wins = 0;
        for speed in 1..self.time {
            let distance = speed * (self.time - speed);
            if distance > self.distance {
                wins += 1;
            }
        }

        wins
    }
}

#[derive(Debug)]
struct Races {
    races: Vec<Race>,
    race: Race,
}

impl Races {
    fn score1(&self) -> u128 {
        self.races.iter().map(|race| race.run()).product()
    }
}

fn parse(content: &String) -> Parsed {
    let lines: Vec<_> = content.split("\n").collect();
    let times: Vec<_> = lines[0].split_whitespace().collect();
    let distances: Vec<_> = lines[1].split_whitespace().collect();

    let mut races = Vec::new();

    let mut t_time = "".to_string();
    let mut t_distance = "".to_string();

    for index in 1..times.len() {
        let time = times[index].parse::<u128>().unwrap();
        t_time.push_str(times[index]);
        let distance = distances[index].parse::<u128>().unwrap();
        t_distance.push_str(distances[index]);
        races.push(Race{time, distance});
    }

    println!("{t_time} {t_distance}");

    return Races { races, race: Race { time: t_time.parse().unwrap(), distance: t_distance.parse().unwrap()} };
}

fn part1(root: &Parsed) {


    println!("{:?}", root);
    println!("Part 1: {}", root.score1());
}

fn part2(root: &Parsed) {

    println!("Part 2: {}", root.race.run());
}

fn main() {
    let files = vec!["sample.txt", /*"sample2.txt" ,*/ "input.txt" ];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content);
        part1(&root);
        part2(&root);
    }
}
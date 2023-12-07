use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

type Parsed = Game;

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord, Clone)]
enum HandType {
    FiveOfAKind = 7,
    // 1
    FourOfAKind = 6,
    // 2
    FullHouse = 5,
    // 2
    ThreeOfAKind = 4,
    // 3
    TwoPair = 3,
    // 3
    OnePair = 2,
    // 4
    HighCard = 1, // 5
}

impl HandType {
    fn from(cards: &Vec<u32>) -> Option<Self> {
        let mut counts: HashMap<u32, usize> = HashMap::new();

        for card in cards.iter() {
            let count = counts.entry(*card).or_insert(0);
            *count += 1;
        }

        match counts.len() {
            5 => Some(HandType::HighCard),
            4 => Some(HandType::OnePair),
            3 => {
                let mut result = None;
                for (_, count) in counts.iter() {
                    if *count == 3 {
                        result = Some(HandType::ThreeOfAKind);
                        break;
                    }
                    if *count == 2 {
                        result = Some(HandType::TwoPair);
                        break;
                    }
                }
                result
            }
            2 => match counts.iter().next() {
                Some((_, count)) if *count == 2 || *count == 3 => Some(HandType::FullHouse),
                Some((_, count)) if *count == 1 || *count == 4 => Some(HandType::FourOfAKind),
                _ => None
            },
            1 => Some(HandType::FiveOfAKind),
            _ => None
        }
    }

    fn from_joker(cards: &Vec<u32>) -> Option<Self> {
        let mut counts: HashMap<u32, usize> = HashMap::new();

        for card in cards.iter() {
            let count = counts.entry(*card).or_insert(0);
            *count += 1;
        }

        let _jokers = counts.get(&1);

        if _jokers.is_none() {
            return HandType::from(cards);
        }

        let joker = *_jokers.unwrap();

        let length = counts.len();
        counts.remove(&1);

        match length {
            5 => Some(HandType::OnePair),
            4 => Some(HandType::ThreeOfAKind),
            3 => {
                match joker {
                    1 => match counts.iter().next() {
                        Some((_, count)) if *count == 1 || *count == 3 => Some(HandType::FourOfAKind),
                        _ => Some(HandType::FullHouse),
                    },
                    _ => Some(HandType::FourOfAKind)
                }
            }
            2 => Some(HandType::FiveOfAKind),
            1 => Some(HandType::FiveOfAKind),
            _ => None
        }
    }
}

#[derive(Debug, Eq, Clone)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
    _type: HandType,
}

impl Hand {
    fn parse(line: &str, part2: bool) -> Option<Self> {
        let parts = line.split_once(" ")?;
        let mut cards = Vec::new();
        let bid = parts.1.parse::<u32>().ok()?;
        for character in parts.0.chars() {
            let value = match character {
                '2' => Some(2),
                '3' => Some(3),
                '4' => Some(4),
                '5' => Some(5),
                '6' => Some(6),
                '7' => Some(7),
                '8' => Some(8),
                '9' => Some(9),
                'T' => Some(10),
                'J' => if part2 { Some(1) } else { Some(11) },
                'Q' => Some(12),
                'K' => Some(13),
                'A' => Some(14),
                _ => None,
            }?;
            cards.push(value);
        }
        Some(Hand { bid, _type: (if part2 { HandType::from_joker(&cards) } else { HandType::from(&cards) })?, cards })
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self._type == other._type
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let _type = self._type.cmp(&other._type);
        match _type {
            Ordering::Equal => self.cards.cmp(&other.cards),
            differ => differ,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct Game(Vec<Hand>, Vec<Hand>);

fn parse(content: &String) -> Parsed {
    let mut hands = Vec::new();
    let mut hands_2 = Vec::new();
    for line in content.split("\n") {
        hands.push(Hand::parse(line, false).unwrap());
        hands_2.push(Hand::parse(line, true).unwrap());
    }
    return Game(hands, hands_2);
}

fn part1(root: &Parsed) {
    let mut game = root.clone();
    game.0.sort();
    //println!("{:?}", game);

    let mut winnings = 0;
    for (index, hand) in game.0.iter().enumerate() {
        winnings += hand.bid * (index as u32 + 1);
    }

    println!("Part 1: {}", winnings);
}

fn part2(root: &Parsed) {
    let mut game = root.clone();
    game.1.sort();
    println!("{:#?}", game.1);

    let mut winnings = 0;
    for (index, hand) in game.1.iter().enumerate() {
        winnings += hand.bid * (index as u32 + 1);
    }

    println!("Part 2: {}", winnings);
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
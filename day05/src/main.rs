use std::cmp::max;
use std::fs;

type Parsed = Almanac;

#[derive(Debug, Clone)]
struct Mapping {
    from: u64,
    to: u64,
    destination: u64,
    destination_to: u64,
    size: u64,
}

#[derive(Debug, Default)]
struct AlmanacMap(Vec<Mapping>);

impl AlmanacMap {
    fn parse_line(&mut self, line: &str) -> Option<()> {
        let raw_values: Result<Vec<u64>, _> = line.split(" ").map(|el| el.parse::<u64>()).collect();
        let values = raw_values.ok()?;
        if values.len()!= 3 {
            return None;
        }

        self.0.push(Mapping{
            from: values[1],
            to: values[1] + values[2] - 1,
            destination: values[0],
            destination_to: values[0] + values[2] - 1,
            size: values[2],
        });

        Some(())
    }

    fn finalize(&mut self) {
        self.0.sort_by(|a, b| a.from.cmp(&b.from));
    }

    fn get(&self, value: u64) -> u64 {
        for mapping in self.0.iter() {
            if value < mapping.from {
                return value;
            }
            if value <= mapping.to {
                return mapping.destination + (value - mapping.from);
            }
        }
        value
    }

    fn maximum(&self, value: u64) -> u64 {
        let last = self.0.last().unwrap();
        let maximum = max(last.to, last.destination + last.size);
        max(maximum, value)
    }

    fn all_mappings(&self, maximum: u64) -> AlmanacMap {
        let mut mappings = Vec::new();

        let mut last_start = 0;
        for mapping in self.0.iter() {
            if last_start < mapping.from {
                mappings.push(Mapping{
                    from: last_start,
                    to: mapping.from - 1,
                    destination: last_start,
                    destination_to: mapping.from - 1,
                    size: mapping.from - last_start,
                });
            }
            mappings.push((*mapping).clone());
            last_start = mapping.to + 1;
        }

        if last_start < maximum {
            mappings.push(Mapping{
                from: last_start,
                to: maximum,
                destination: last_start,
                destination_to: maximum,
                size: maximum + 1 - last_start,
            });
        }

        AlmanacMap(mappings)
    }

    fn merge(&self, next_step: &AlmanacMap) -> AlmanacMap {
        let mut new_mappings = Vec::new();
        let mut mappings = self.0.clone();
        mappings.sort_by(|a, b| a.destination.cmp(&b.destination));
        for mapping in mappings.iter() {
            // println!("Old {:#?}", mapping);
            let mut current_start = mapping.destination;
            loop {
                // println!("start: {:?}", current_start);
                let relevant_mapping = next_step.0.iter().find(|m| m.from <= current_start && m.to >= current_start).unwrap();
                // println!("found {:#?}", relevant_mapping);

                let from = mapping.from + (current_start - mapping.destination);
                let destination = relevant_mapping.destination + (current_start - relevant_mapping.from);
                if mapping.destination_to <= relevant_mapping.to {
                    let to = mapping.to;
                    let size = to + 1 - from;
                    let new_mapping = Mapping{
                        from,
                        to,
                        destination,
                        destination_to: relevant_mapping.destination + size -1,
                        size,
                    };
                    // println!("New {:#?}", new_mapping);
                    new_mappings.push(new_mapping);
                    break;
                }
                let smaller = mapping.destination_to - relevant_mapping.to;
                let to = mapping.to - smaller;
                // println!("smaller: {:?}", smaller);
                // println!("to: {:?}", to);
                let size = to + 1 - from;
                let new_mapping = Mapping{
                    from,
                    to,
                    destination,
                    destination_to: relevant_mapping.destination + size - 1,
                    size,
                };
                // println!("New {:#?}", new_mapping);
                new_mappings.push(new_mapping);
                current_start = relevant_mapping.to + 1;
            }

        }

        new_mappings.sort_by(|a, b| a.from.cmp(&b.from));
        AlmanacMap(new_mappings)
    }

    fn find_minimal_seed(&self, seed_ranges: &Vec<(u64, u64)>) -> Option<u64> {
        for mapping in self.0.iter() {
            // println!("{:?}", mapping);
            for seed_range in seed_ranges.iter() {
                // println!("{:?}", seed_range);
                let last_index = seed_range.1 - 1;
                if seed_range.0 >= mapping.from && seed_range.0 <= mapping.to {
                    return Some(seed_range.0);
                }
                if last_index >= mapping.from && last_index <= mapping.to {
                    return Some(mapping.from);
                }
                if seed_range.0 < mapping.from && last_index > mapping.to {
                    return Some(mapping.from);
                }
            }
        }

        None
    }
}

#[derive(Debug)]
struct MappedSeed {
    seed: u64,
    soil: u64,
    fertilizer: u64,
    water: u64,
    light: u64,
    temperature: u64,
    humidity: u64,
    location: u64,
}

#[derive(Debug)]
struct MappedSeeds(Vec<MappedSeed>);

impl MappedSeeds {
    fn lowest_location(&self) -> u64 {
        let mut lowest_location: Option<u64> = None;
        for seed in self.0.iter() {
            if lowest_location.is_none() || seed.location < lowest_location.unwrap() {
                lowest_location = Some(seed.location);
            }
        }

        lowest_location.unwrap()
    }
}

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: AlmanacMap,
    soil_to_fertilizer: AlmanacMap,
    fertilizer_to_water: AlmanacMap,
    water_to_light: AlmanacMap,
    light_to_temperature: AlmanacMap,
    temperature_to_humidity: AlmanacMap,
    humidity_to_location: AlmanacMap,
}

impl Almanac {
    fn parse_seeds(&mut self, seeds: &str) -> Option<()> {
        let parts: Vec<_> = seeds.splitn(2, ": ").collect();
        if parts.len()!= 2 {
            return None;
        }

        let parsed: Result<Vec<_>, _>  = parts[1].split(" ").map(|s| s.parse::<u64>()).collect();
        self.seeds = parsed.ok()?;

        Some(())
    }

    fn map_seed(&self, seed: u64) -> MappedSeed {
        let soil = self.seed_to_soil.get(seed);
        let fertilizer = self.soil_to_fertilizer.get(soil);
        let water = self.fertilizer_to_water.get(fertilizer);
        let light = self.water_to_light.get(water);
        let temperature = self.light_to_temperature.get(light);
        let humidity = self.temperature_to_humidity.get(temperature);
        let location = self.humidity_to_location.get(humidity);

        MappedSeed{seed, soil, fertilizer, water, light, temperature, humidity, location}
    }

    fn seed_location(&self, seed: u64) -> u64 {
        self.map_seed(seed).location
    }

    fn map_seeds(&self) -> MappedSeeds {
        let mut mapped = Vec::new();
        for seed in self.seeds.iter() {
            mapped.push(self.map_seed(*seed));
        }

        MappedSeeds(mapped)
    }

    fn seed_ranges(&self) -> Vec<(u64, u64)> {
        let mut ranges = Vec::new();
        for index in 0..(self.seeds.len()/2) {
            let start = self.seeds[index *2];
            let length = self.seeds[index *2 + 1];
            ranges.push((start, start + length));
        }

        ranges
    }

    fn lowest_location_from_ranges(&self) -> u64 {
        let mut lowest_location: Option<u64> = None;
        // vector of pairs from vector
        for (start, end) in self.seed_ranges() {
            println!("Range: {start} {end}");
            for inner in start..end {
                let location = self.seed_location(inner);
                if lowest_location.is_none() || location < lowest_location.unwrap() {
                    lowest_location = Some(location);
                }
            }
        }

        lowest_location.unwrap()
    }

    fn reverse_find_maximum(&self) -> u64 {
        let mut maximum = *self.seed_ranges().iter().map(|(_, end)| end).max().unwrap();
        maximum = self.seed_to_soil.maximum(maximum);
        maximum = self.soil_to_fertilizer.maximum(maximum);
        maximum = self.fertilizer_to_water.maximum(maximum);
        maximum = self.water_to_light.maximum(maximum);
        maximum = self.light_to_temperature.maximum(maximum);
        maximum = self.temperature_to_humidity.maximum(maximum);
        maximum = self.humidity_to_location.maximum(maximum);

        maximum
    }

    fn merged_maps(&self) -> AlmanacMap {
        let maximum = self.reverse_find_maximum();
        let step1 = self.humidity_to_location.all_mappings(maximum);
        let step2 = self.temperature_to_humidity.all_mappings(maximum).merge(&step1);
        let step3 = self.light_to_temperature.all_mappings(maximum).merge(&step2);
        let step4 = self.water_to_light.all_mappings(maximum).merge(&step3);
        let step5 = self.fertilizer_to_water.all_mappings(maximum).merge(&step4);
        let step6 = self.soil_to_fertilizer.all_mappings(maximum).merge(&step5);
        self.seed_to_soil.all_mappings(maximum).merge(&step6)
    }
}

fn parse(content: &String) -> Option<Parsed> {
    let mut almanac = Almanac::default();
    let mut iter = content.split("\n");

    almanac.parse_seeds(iter.next()?)?;
    iter.next()?;
    iter.next()?;

    let mut line = iter.next()?;

    while line != "" {
        almanac.seed_to_soil.parse_line(line)?;
        line = iter.next()?;
    }
    almanac.seed_to_soil.finalize();

    iter.next()?;
    line = iter.next()?;
    while line != "" {
        almanac.soil_to_fertilizer.parse_line(line)?;
        line = iter.next()?;
    }
    almanac.soil_to_fertilizer.finalize();

    iter.next()?;
    line = iter.next()?;
    while line != "" {
        almanac.fertilizer_to_water.parse_line(line)?;
        line = iter.next()?;
    }
    almanac.fertilizer_to_water.finalize();

    iter.next()?;
    line = iter.next()?;
    while line != "" {
        almanac.water_to_light.parse_line(line)?;
        line = iter.next()?;
    }
    almanac.water_to_light.finalize();

    iter.next()?;
    line = iter.next()?;
    while line != "" {
        almanac.light_to_temperature.parse_line(line)?;
        line = iter.next()?;
    }
    almanac.light_to_temperature.finalize();

    iter.next()?;
    line = iter.next()?;
    while line != "" {
        almanac.temperature_to_humidity.parse_line(line)?;
        line = iter.next()?;
    }
    almanac.temperature_to_humidity.finalize();

    iter.next()?;
    line = iter.next()?;
    while line != "" {
        almanac.humidity_to_location.parse_line(line)?;

        let next = iter.next();
        if next.is_none() {
            break;
        }
        line = next.unwrap();
    }
    almanac.humidity_to_location.finalize();


    return Some(almanac);
}

fn part1(root: &Parsed) {
    println!("{:#?}", root);
    println!("Part 1: {}", root.map_seeds().lowest_location());
}

fn part2(root: &Parsed) {
    let maximum = root.reverse_find_maximum();
    println!("Max: {}", maximum);
    let mut merged = root.merged_maps();
    merged.0.sort_by(|a, b| a.destination.cmp(&b.destination));
    println!("{:#?}", merged);

    let lowest_seed = merged.find_minimal_seed(&root.seed_ranges()).unwrap();
    println!("Lowest {}", lowest_seed);

    //println!("Part 2: {}", root.lowest_location_from_ranges());
    println!("Part 2: {:?}", root.map_seed(lowest_seed));
}

fn main() {
    let files = vec!["sample.txt", /*"sample2.txt" ,*/ "input.txt"];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content).unwrap();
        part1(&root);
        part2(&root);
    }
}
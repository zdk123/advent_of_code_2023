use std::collections::HashMap;
use rayon::prelude::*;

fn main() {
    let input = include_str!("./input5.txt");
    let output = part2(input);
    println!("{}", output);
}


#[derive(Debug)]
struct SeedMapper {
    target_key: String,
    ranges: Vec<SeedRange>
}

impl SeedMapper {
    fn find_number(&self, number: &u64) -> u64 {
        self.ranges.iter().find(|range| range.contains(*number)).map_or(*number, |range| {
            range.calculate(*number)
        })
    }

    fn add_range(&mut self, source: u64, destination:u64, range:u64) {
        self.ranges.push(SeedRange {
            source, destination, range
        })
    }
}

#[derive(Debug)]
struct SeedRange {
    source: u64,
    destination: u64,
    range: u64
}

impl SeedRange {
    fn contains(&self, number: u64) -> bool {
        (number >= self.source) && (number <= (self.source + self.range-1))
    }
    fn calculate(&self, number: u64) -> u64 {
        let diff = number - self.source;
        self.destination + diff
    }
}


fn part2(input: &str) -> u64 {

    let mut extracted_almanac_iter = input
                                        .lines();

    // Process the first seeds line
    let seeds_str = extracted_almanac_iter.next().unwrap()
                                        .split(": ")
                                        .collect::<Vec<&str>>()[1];
    let seeds = seeds_str.split_whitespace()
                         .map(|number| number.parse::<u64>().unwrap())
                         .collect::<Vec<u64>>();

                         let mut mappings : HashMap<&str, SeedMapper> = HashMap::new();

    let mut ranges: Vec<(u64,u64)> = seeds.chunks(2).map(|pair| (pair[0], pair[0] + pair[1]) ).collect();

    let mut source_key = "";
    
    // Get the rest of the maps from each almanac table
    extracted_almanac_iter.map(|line| line.trim()).filter(|line| !line.is_empty())
        .for_each(|line| {

        if line.contains("map:") {
            let (source, target) = line.trim().split_once("-to-").unwrap();
            let target = target.replace(" map:", "");

            mappings.insert(source, SeedMapper {
                target_key: target,
                ranges: Vec::new()
            });
            source_key = source;
        } else {
            let numbers: Vec<u64> =  line.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
            let mapping = mappings.get_mut(source_key).unwrap();

            mapping.add_range(numbers[1], numbers[0], numbers[2]);
        }
    });

    let lowest_location: Vec<u64> = ranges.par_iter().flat_map(|range|  {
        range.0..range.1
    }).map(|seed| {
        let mut next_map = "seed";
        let mut next_number = seed;

        while next_map != "location" {
            let seed_mapper = mappings.get(next_map).unwrap();

            next_map = &seed_mapper.target_key;
            next_number = seed_mapper.find_number(&next_number);

        };

        next_number
    }).collect();

    return *lowest_location.iter().min().unwrap()

}


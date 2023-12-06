use itertools::Itertools;
use regex::Regex;

pub fn solve(input: &[String]) -> String {
    let mut almanac = Almanac::new();
    let seeds_re = Regex::new(r"seeds:([\s|\d]*)").unwrap();

    for line in input {
        if let Some(seeds) = seeds_re.captures(line) {
            almanac.seeds.extend(
                seeds.extract::<1>().1[0]
                    .split_whitespace()
                    .map(|number| number.parse::<u32>().unwrap()),
            );
        } else if line.contains("map") {
            almanac.maps.push(Vec::new());
        } else if let Some((destination_start, source_start, range_length)) = line
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect_tuple()
        {
            almanac.maps.last_mut().unwrap().push(Interval {
                destination_start,
                source_start,
                range_length,
            });
        }
    }

    format!("{}\n{}\n", part_1(&almanac), part_2())
}

fn part_1(almanac: &Almanac) -> u32 {
    almanac
        .seeds
        .iter()
        .map(|seed| seed_to_location(*seed, almanac))
        .min()
        .unwrap()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

fn seed_to_location(seed: u32, almanac: &Almanac) -> u32 {
    almanac.maps.iter().fold(seed, |number, map| {
        for interval in map {
            if let Some(difference) = number.checked_sub(interval.source_start) {
                if difference < interval.range_length {
                    return interval.destination_start + difference;
                }
            }
        }

        number
    })
}

struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Vec<Interval>>,
}

impl Almanac {
    fn new() -> Self {
        Almanac {
            seeds: Vec::new(),
            maps: Vec::new(),
        }
    }
}

struct Interval {
    destination_start: u32,
    source_start: u32,
    range_length: u32,
}

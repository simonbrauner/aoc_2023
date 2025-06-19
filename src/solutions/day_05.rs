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

    format!("{}\n{}\n", part_1(&almanac), part_2(&almanac))
}

fn part_1(almanac: &Almanac) -> u32 {
    almanac.lowest_location_number(almanac.seeds.clone().into_iter())
}

fn part_2(almanac: &Almanac) -> u32 {
    almanac.lowest_location_number(
        almanac
            .seeds
            .iter()
            .tuples()
            .flat_map(|(&start, &count)| (start..(start + count))),
    )
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

    fn lowest_location_number<I>(&self, iterator: I) -> u32
    where
        I: Iterator<Item = u32>,
    {
        iterator
            .map(|seed| seed_to_location(seed, self))
            .min()
            .unwrap()
    }
}

struct Interval {
    destination_start: u32,
    source_start: u32,
    range_length: u32,
}

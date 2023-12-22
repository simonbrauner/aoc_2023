use itertools::Itertools;

pub fn solve(input: &[String]) -> String {
    let mut records = Vec::new();

    for line in input {
        let (first, second) = line.split_whitespace().collect_tuple().unwrap();
        let springs = first.chars().collect();
        let sizes = second
            .split(',')
            .map(|number| number.parse().unwrap())
            .collect();

        records.push(Record { springs, sizes });
    }

    format!("{}\n{}\n", part_1(&records), part_2())
}

fn part_1(records: &[Record]) -> u32 {
    records.iter().map(count_valid).sum()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

fn count_valid(record: &Record) -> u32 {
    if let Some(index) = record.springs.iter().position(|&letter| letter == '?') {
        let mut assigned_record = record.clone();

        KNOWN_SYMBOLS
            .chars()
            .map(|symbol| {
                assigned_record.springs[index] = symbol;
                count_valid(&assigned_record)
            })
            .sum()
    } else {
        match record.is_valid() {
            true => 1,
            false => 0,
        }
    }
}

#[derive(Clone)]
struct Record {
    springs: Vec<char>,
    sizes: Vec<i32>,
}

impl Record {
    fn is_valid(&self) -> bool {
        let mut sizes_index = 0;
        let mut springs = self.springs.iter();

        while sizes_index < self.sizes.len() {
            if let Some(&spring) = springs.next() {
                if spring == '#' {
                    for _ in 1..self.sizes[sizes_index] {
                        if let Some('#') = springs.next() {
                        } else {
                            return false;
                        }
                    }

                    if let Some('#') = springs.next() {
                        return false;
                    }

                    sizes_index += 1;
                }
            } else {
                return false;
            };
        }

        springs.all(|&spring| spring != '#')
    }
}

const KNOWN_SYMBOLS: &str = ".#";

use itertools::Itertools;

pub fn solve(input: &[String]) -> String {
    let (times, records) = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|number| number.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect_tuple()
        .unwrap();

    let races: Vec<Race> = times
        .into_iter()
        .zip(records)
        .map(|(time, record)| Race { time, record })
        .collect();

    format!("{}\n{}\n", part_1(&races), part_2())
}

fn part_1(races: &[Race]) -> usize {
    races.iter().map(ways_to_beat).product()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

fn ways_to_beat(race: &Race) -> usize {
    (0..=race.time)
        .filter(|hold_duration| hold_duration * (race.time - hold_duration) > race.record)
        .count()
}

struct Race {
    time: u32,
    record: u32,
}

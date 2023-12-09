use itertools::Itertools;

pub fn solve(input: &[String]) -> String {
    let (times, records) = input
        .iter()
        .map(|line| line.split_whitespace().skip(1).collect::<Vec<&str>>())
        .collect_tuple()
        .unwrap();

    let races: Vec<Race> = times
        .iter()
        .zip(records.clone())
        .map(|(time, record)| Race {
            time: time.parse().unwrap(),
            record: record.parse().unwrap(),
        })
        .collect();

    let big_race = Race {
        time: times.join("").parse().unwrap(),
        record: records.join("").parse().unwrap(),
    };

    format!("{}\n{}\n", part_1(&races), part_2(&big_race))
}

fn part_1(races: &[Race]) -> u64 {
    races.iter().map(ways_to_beat).product()
}

fn part_2(race: &Race) -> u64 {
    ways_to_beat(race)
}

fn ways_to_beat(race: &Race) -> u64 {
    race.time + 1
        - 2 * (0..=race.time)
            .find(|hold_duration| hold_duration * (race.time - hold_duration) > race.record)
            .unwrap()
}

struct Race {
    time: u64,
    record: u64,
}

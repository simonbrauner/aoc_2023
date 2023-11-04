use std::io::BufRead;
use std::{fs::File, io::BufReader};

use anyhow::Result;

pub fn solve(reader: BufReader<File>) -> Result<String> {
    for line in reader.lines() {
        line?;
    }

    Ok(format!("{}\n{}\n", part_1(), part_2()))
}

fn part_1() -> String {
    "part 1 unimplemented".to_string()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

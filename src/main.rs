use std::{fs::read_to_string, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use clap::Parser;

mod solutions;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Args {
    /// Day of the puzzle, between 1 and 25
    day: u8,

    /// File to read input from
    input: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let lines: Vec<String> = read_to_string(&args.input)
        .with_context(|| format!("Failed to open '{}'", args.input.display()))?
        .lines()
        .map(|line| line.to_string())
        .collect();

    let solution = match args.day {
        1 => solutions::day_01::solve(&lines),
        2 => solutions::day_02::solve(&lines),
        3 => solutions::day_03::solve(&lines),
        4 => solutions::day_04::solve(&lines),
        5 => solutions::day_05::solve(&lines),
        6 => solutions::day_06::solve(&lines),
        7 => solutions::day_07::solve(&lines),
        8 => solutions::day_08::solve(&lines),
        9 => solutions::day_09::solve(&lines),
        10 => solutions::day_10::solve(&lines),
        11 => solutions::day_11::solve(&lines),
        12 => solutions::day_12::solve(&lines),
        13 => solutions::day_13::solve(&lines),
        14 => solutions::day_14::solve(&lines),
        15 => solutions::day_15::solve(&lines),
        16 => solutions::day_16::solve(&lines),
        17 => solutions::day_17::solve(&lines),
        18 => solutions::day_18::solve(&lines),
        19 => solutions::day_19::solve(&lines),
        20 => solutions::day_20::solve(&lines),
        21 => solutions::day_21::solve(&lines),
        22 => solutions::day_22::solve(&lines),
        23 => solutions::day_23::solve(&lines),
        24 => solutions::day_24::solve(&lines),
        25 => solutions::day_25::solve(&lines),
        _ => return Err(anyhow!("Invalid day: {}", args.day)),
    };

    print!("{}", solution);
    Ok(())
}

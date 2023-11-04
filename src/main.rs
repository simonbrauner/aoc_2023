use std::{fs::File, io::BufReader, path::PathBuf};

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
    let reader = BufReader::new(
        File::open(&args.input)
            .with_context(|| format!("Failed to open '{}'", args.input.display()))?,
    );

    print!(
        "{}",
        match args.day {
            1 => solutions::day_01::solve(reader)?,
            2 => solutions::day_02::solve(reader)?,
            3 => solutions::day_03::solve(reader)?,
            4 => solutions::day_04::solve(reader)?,
            5 => solutions::day_05::solve(reader)?,
            6 => solutions::day_06::solve(reader)?,
            7 => solutions::day_07::solve(reader)?,
            8 => solutions::day_08::solve(reader)?,
            9 => solutions::day_09::solve(reader)?,
            10 => solutions::day_10::solve(reader)?,
            11 => solutions::day_11::solve(reader)?,
            12 => solutions::day_12::solve(reader)?,
            13 => solutions::day_13::solve(reader)?,
            14 => solutions::day_14::solve(reader)?,
            15 => solutions::day_15::solve(reader)?,
            16 => solutions::day_16::solve(reader)?,
            17 => solutions::day_17::solve(reader)?,
            18 => solutions::day_18::solve(reader)?,
            19 => solutions::day_19::solve(reader)?,
            20 => solutions::day_20::solve(reader)?,
            21 => solutions::day_21::solve(reader)?,
            22 => solutions::day_22::solve(reader)?,
            23 => solutions::day_23::solve(reader)?,
            24 => solutions::day_24::solve(reader)?,
            25 => solutions::day_25::solve(reader)?,
            _ => return Err(anyhow!("Invalid day: {}", args.day)),
        }
    );

    Ok(())
}

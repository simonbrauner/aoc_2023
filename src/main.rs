use std::{fs::File, io::BufReader, path::PathBuf};

use anyhow::{Context, Result};
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
            _ => todo!(),
        }
    );

    Ok(())
}

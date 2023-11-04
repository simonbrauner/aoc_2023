use std::{fs::File, io::BufReader, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;

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
    let _reader = BufReader::new(
        File::open(&args.input)
            .with_context(|| format!("Failed to open '{}'", args.input.display()))?,
    );

    println!(
        "run solution {} on input file {}",
        args.day,
        args.input.display()
    );

    Ok(())
}

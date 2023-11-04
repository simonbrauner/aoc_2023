use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Args {
    /// Day of the puzzle, between 1 and 25
    day: u8,

    /// File to read input from
    input: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!(
        "run solution {} on input file {}",
        args.day,
        args.input.display()
    );
}

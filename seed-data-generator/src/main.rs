mod cli;
mod generator;
mod model;

use clap::Parser;

use crate::cli::Args;

fn main() -> Result<(), String> {
    let args = Args::parse();

    args.validate()?;

    let Args {
        seed,
        min,
        max,
        output,
    } = args;

    
    
    return Ok(());
}

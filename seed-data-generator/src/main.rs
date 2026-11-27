mod cli;
mod generator;
mod model;

use clap::Parser;
use rand::{RngExt, SeedableRng, rngs::StdRng};

use crate::{
    cli::Args,
    generator::{DailyReceiptGenerator, DistributionGenerator, Generator},
};

fn main() -> Result<(), String> {
    let args = Args::parse();

    args.validate()?;

    let Args {
        seed,
        min,
        max,
        start_date,
        output,
    } = args;

    let mut rng = StdRng::seed_from_u64(seed);

    let amount: usize = rng.random_range(min..=max);

    let daily_distribution = DistributionGenerator {
        rng: &mut rng,
        amount: amount,
    }
    .generate();

    let mut output = std::fs::File::create(output).map_err(|e| e.to_string())?;

    let receipts = DailyReceiptGenerator {
        rng: &mut rng,
        daily_distribution: &daily_distribution,
        start_date: &start_date,
        output: &mut output,
    }
    .generate();

    return Ok(());
}

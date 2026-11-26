use clap::Parser;
use rand::{RngExt, SeedableRng, rngs::StdRng};
use rand_distr::{Distribution, Gamma, Poisson};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    seed: u64,
    #[arg(long)]
    min: u64,
    #[arg(long)]
    max: u64,
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    validate_args(&args)?;

    let seed = args.seed;
    let min = args.min;
    let max = args.max;

    let mut rng = StdRng::seed_from_u64(seed);

    let amount = rng.random_range(min..=max);

    println!("Generating {} data...", amount);

    let mut remaining = amount;
    let mut daily_counts = Vec::new();

    let gamma = Gamma::new(20.0, 5.0).unwrap();

    while remaining > 0 {
        let lambda = gamma.sample(&mut rng);
        let distribution = Poisson::new(lambda).unwrap();

        let generated = distribution.sample(&mut rng) as usize;
        let count = generated.min(remaining as usize);

        daily_counts.push(count);
        remaining -= count as u64;

        println!("Day {}: {}", daily_counts.len(), count)
    }

    return Ok(());
}

fn validate_args(args: &Args) -> Result<(), String> {
    if args.min > args.max {
        return Err("min cannot be greater than max".to_string());
    }

    if args.max < args.min {
        return Err("max cannot be less than than min".to_string());
    }

    if args.output.is_empty() {
        return Err("output file name cannot be empty".to_string());
    }

    return Ok(());
}

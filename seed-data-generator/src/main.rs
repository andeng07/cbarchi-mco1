use clap::Parser;

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

    println!("Seed: {}", args.seed);
    println!("Min: {}", args.min);
    println!("Max: {}", args.max);

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

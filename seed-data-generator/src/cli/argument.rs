use chrono::NaiveDate;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    pub seed: u64,
    #[arg(long)]
    pub min: usize,
    #[arg(long)]
    pub max: usize,
    #[arg(long)]
    pub start_date: NaiveDate,
    #[arg(short, long)]
    pub output: String,
}

impl Args {
    pub fn validate(&self) -> Result<(), String> {
        if self.min > self.max {
            return Err("min cannot be greater than max".to_string());
        }

        if self.max < self.min {
            return Err("max cannot be less than than min".to_string());
        }

        if self.output.is_empty() {
            return Err("output file name cannot be empty".to_string());
        }

        return Ok(());
    }
}

use rand::rngs::StdRng;
use rand_distr::{Distribution, Gamma, Poisson};

pub fn generate_daily_counts(rng: &mut StdRng, amount: u64) -> Vec<usize> {
    let mut remaining = amount;
    let mut daily_counts = Vec::new();

    let gamma = Gamma::new(20.0, 5.0).unwrap();

    while remaining > 0 {
        let lambda = gamma.sample(rng);
        let distribution = Poisson::new(lambda).unwrap();

        let generated = distribution.sample(rng) as usize;
        let count = generated.min(remaining as usize);

        daily_counts.push(count);
        remaining -= count as u64;
    }

    daily_counts
}

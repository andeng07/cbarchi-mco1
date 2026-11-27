use crate::generator::Generator;
use rand::rngs::StdRng;
use rand_distr::{Distribution, Gamma, Poisson};

pub struct DistributionGenerator<'a> {
    pub rng: &'a mut StdRng,
    pub amount: usize,
}

impl Generator<Vec<usize>> for DistributionGenerator<'_> {
    fn generate(&mut self) -> Vec<usize> {
        let mut remaining = self.amount;
        let mut daily_counts = Vec::new();

        let gamma = Gamma::new(20.0, 5.0).unwrap();

        while remaining > 0 {
            let lambda = gamma.sample(self.rng);
            let distribution = Poisson::new(lambda).unwrap();

            let generated = distribution.sample(self.rng) as usize;
            let count = generated.min(remaining);

            daily_counts.push(count);
            remaining -= count;
        }

        daily_counts
    }
}

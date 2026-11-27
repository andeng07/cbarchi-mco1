use std::io::Write;

use chrono::{Datelike, Days, NaiveDate};
use rand::{RngExt, rngs::StdRng};

use crate::generator::Generator;

pub struct DailyReceiptGenerator<'a> {
    pub rng: &'a mut StdRng,
    pub daily_distribution: &'a [usize],
    pub start_date: &'a NaiveDate,
    pub output: &'a mut dyn Write,
}

impl Generator<()> for DailyReceiptGenerator<'_> {
    fn generate(&mut self) {
        let dates = std::iter::successors(Some(*self.start_date), |date| {
            date.checked_add_days(Days::new(1))
        });

        for (date, receipts_count) in dates.zip(self.daily_distribution.iter()) {
            for _ in 0..*receipts_count {
                let price = self.rng.random_range(10000..99999);

                writeln!(
                    self.output,
                    "{:04}-{:02}-{:02},{:.2}",
                    date.year(),
                    date.month(),
                    date.day(),
                    price
                )
                .expect("Failed to write receipt");
            }
        }
    }
}

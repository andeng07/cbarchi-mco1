use chrono::NaiveDate;

#[derive(Debug)]
pub struct Receipt {
    pub price: f32,
    pub date: NaiveDate,
}

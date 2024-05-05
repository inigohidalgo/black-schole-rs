use chrono::{DateTime, Local};

struct Call {
    strike: f64,
    maturity: DateTime<Local>,
}

impl Call {


    fn time_to_maturity(&self, current_date: DateTime<Local>) -> f64 {
        (self.maturity - current_date).num_days() as f64 / 365.0
    }

}
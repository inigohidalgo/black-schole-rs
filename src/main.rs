use chrono::{DateTime, Local, TimeZone};

/// A call option
struct Call {
    /// The price at which the option holder can exercise the option
    strike: f64,
    /// The date at which the option expires
    maturity: DateTime<Local>,
}

impl Call {
    /// Calculate the time to maturity of the option
    /// as a fraction of a year
    fn time_to_maturity(&self, current_date: DateTime<Local>) -> f64 {
        (self.maturity - current_date).num_days() as f64 / 365.0
    }

}

fn main() {
    let call = Call {
        strike: 100.0,
        maturity: Local.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
    };

    let current_date = Local.with_ymd_and_hms(2021, 11, 1, 0, 0, 0).unwrap();

    println!("Time to maturity: {}", call.time_to_maturity(current_date));
}
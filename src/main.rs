//! Implementation of Black-Scholes' Option Pricing Formula
//!
//! Refer to Hull, John C. *Options, Futures, and Other Derivatives* (10th ed., pp 333, 334).

// use core::time;

use chrono::{DateTime, Local, TimeZone};
use statrs::distribution::{ContinuousCDF, Normal};
use std::f64::consts::E;

/// Cumulative probability distribution function for a variable with a standard normal distribution
///
/// The probability that a variable would be less than or equal to `x`
fn standard_normal_cdf(x: f64) -> f64 {
    let normal_distribution = Normal::new(0.0, 1.0).unwrap();

    normal_distribution.cdf(x)
}

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

    fn d1(
        &self,
        spot_price: f64,
        time_to_maturity: f64,
        risk_free_rate: f64,
        volatility: f64,
    ) -> f64 {
        (1.0 / (time_to_maturity.sqrt() * volatility)) * (spot_price / self.strike).ln()
            + (time_to_maturity * (risk_free_rate + volatility.powf(2.0) / 2.0))
    }

    fn d2(&self, d1: f64, time_to_maturity: f64, volatility: f64) -> f64 {
        d1 - (volatility * time_to_maturity.sqrt())
    }

    fn price(
        &self,
        spot_price: f64,
        time_to_maturity: f64,
        risk_free_rate: f64,
        volatility: f64,
    ) -> f64 {
        let d1 = self.d1(spot_price, time_to_maturity, risk_free_rate, volatility);
        let d2 = self.d2(d1, time_to_maturity, volatility);
        let call_price = spot_price * standard_normal_cdf(d1)
            - (self.strike * E.powf(-risk_free_rate * time_to_maturity) * standard_normal_cdf(d2));
        call_price
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

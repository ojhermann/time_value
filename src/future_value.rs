//! Functions for calculating future values.

use num::Float;
use std::iter::Product;
use std::slice::Iter;

#[allow(dead_code)]
/// Converts a present value and expected rates into a future value.
///
/// # Example with f32
/// Assumptions
/// - Initial investment: EUR 10
/// - 100% rate of return in year one
/// - 200% rate of return in year two
/// - 300% rate of return in year two
///
/// Projections
/// - Year 0: EUR 10 as initial investment
/// - Year 1: EUR 20
/// - Year 2: EUR 60
/// - Year 3: EUR 240
/// ```
/// use time_value::future_value::from_pv_and_expected_rates;
/// let present_value: f32 = 10.0;
/// let rates: Vec<f32> = vec![1.0, 2.0, 3.0];
/// let expected_value: f32 = 240.0;
/// let value: f32 = from_pv_and_expected_rates(&present_value, rates.iter());
/// assert_eq!(value, expected_value);
/// ```
///
/// # Example with f64
/// Assumptions
/// - Initial investment: EUR 10
/// - 10% rate of return in year one
/// - 10% rate of return in year two
/// - 10% rate of return in year two
///
/// Projections
/// - Year 0: EUR 10 as initial investment
/// - Year 1: EUR 11
/// - Year 2: EUR 12.10
/// - Year 3: EUR 13.31
/// ```
/// use time_value::future_value::from_pv_and_expected_rates;
/// use num::{abs_sub, abs};
/// let present_value: f64 = 10.0;
/// let rates: Vec<f64> = vec![0.1, 0.1, 0.1];
/// let expected_value: f64 = 13.31;
/// let value: f64 = from_pv_and_expected_rates(&present_value, rates.iter());
/// assert!(abs(value - expected_value) < 0.001)
/// ```
pub fn from_pv_and_expected_rates<T>(present_value: &T, expected_rates: Iter<T>) -> T
where
    T: Float + Product<T>,
{
    *present_value * expected_rates.map(|rate| T::one() + *rate).product::<T>()
}

#[cfg(test)]
mod from_pv_and_expected_rates_tests {
    use crate::future_value;

    #[test]
    fn it_works_with_rates_f32() {
        let present_value: f32 = 10.0;
        let rates: Vec<f32> = vec![1.0, 2.0, 3.0];
        let expected_value: f32 = 240.0;
        let value: f32 = future_value::from_pv_and_expected_rates(&present_value, rates.iter());
        assert_eq!(value, expected_value);
        assert_eq!(rates.len(), 3);
    }

    #[test]
    fn it_works_without_rates_f32() {
        let present_value: f32 = 10.0;
        let rates: Vec<f32> = vec![];
        let value: f32 = future_value::from_pv_and_expected_rates(&present_value, rates.iter());
        assert_eq!(value, present_value);
        assert_eq!(rates.len(), 0);
    }

    #[test]
    fn it_works_with_rates_f64() {
        let present_value: f64 = 10.0;
        let rates: Vec<f64> = vec![1.0, 2.0, 3.0];
        let expected_value: f64 = 240.0;
        let value: f64 = future_value::from_pv_and_expected_rates(&present_value, rates.iter());
        assert_eq!(value, expected_value);
        assert_eq!(rates.len(), 3);
    }

    #[test]
    fn it_works_without_rates_f64() {
        let present_value: f64 = 10.0;
        let rates: Vec<f64> = vec![];
        let value: f64 = future_value::from_pv_and_expected_rates(&present_value, rates.iter());
        assert_eq!(value, present_value);
        assert_eq!(rates.len(), 0);
    }
}

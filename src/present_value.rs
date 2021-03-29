//! Functions for calculating present values.

use num::Float;
use std::iter::{Product, Sum};
use std::slice::Iter;

#[allow(dead_code)]
/// Converts a single value to a present value.
///
/// # Example with f32
/// Assumptions
/// - Cash flow: EUR 5.00
/// - Period: 1 i.e. one period from the present period 0
/// - Discount Rate: 20.00%
/// ```
/// use time_value::present_value::present_value;
/// use num::abs;
///
/// let cash_flow: f32 = 5.0;
/// let period: usize = 1;
/// let discount_rate: f32 = 0.20;
/// let expected_value: f32 = 4.167;
/// let value: f32 = present_value(&cash_flow, period, &discount_rate);
/// assert!(abs(value - expected_value) < 0.001);
/// ```
///
/// # Example with f64
/// Assumptions
/// - Cash flow: EUR 10.00
/// - Period: 2 i.e. two periods from the present period 0
/// - Discount Rate: 10.00%
/// ```
/// use time_value::present_value::present_value;
/// use num::abs;
///
/// let cash_flow: f64 = 10.0;
/// let period: usize = 2;
/// let discount_rate: f64 = 0.10;
/// let expected_value: f64 = 8.264;
/// let value: f64 = present_value(&cash_flow, period, &discount_rate);
/// assert!(abs(value - expected_value) < 0.001);
/// ```
pub fn present_value<T>(cash_flow: &T, period: usize, discount_rate: &T) -> T
where
    T: Float + Product<T>,
{
    let period: i32 = -(period as i32);
    let discount: T = T::one() + *discount_rate;
    let discount_factor: T = discount.powi(period);
    *cash_flow * discount_factor
}

#[cfg(test)]
mod present_value_tests {
    use crate::present_value::present_value;
    use num::abs;

    #[test]
    fn it_works_at_zero() {
        let cash_flows: Vec<f32> = vec![0.0, 1.0, -1.0, 1234.56789, -1234.56789];
        let period: usize = 0;
        let discount_rate: f32 = 0.20;
        for cash_flow in cash_flows {
            assert_eq!(cash_flow, present_value(&cash_flow, period, &discount_rate));
        }
    }

    #[test]
    fn it_works_at_one() {
        let cash_flows: Vec<f32> = vec![0.0, 1.0, -1.0, 1234.56789, -1234.56789];
        let period: usize = 1;
        let discount_rate: f32 = 0.20;
        let expected_present_values: Vec<f32> = vec![0.00, 0.833, -0.833, 1028.806, -1028.806];
        let precision: f32 = 0.001;
        for index in 0..cash_flows.len() {
            let actual_pv: f32 = present_value(&cash_flows[index], period, &discount_rate);
            let expected_pv: f32 = expected_present_values[index];
            assert!(abs(actual_pv - expected_pv) <= precision);
        }
    }

    #[test]
    fn it_works_at_two() {
        let cash_flows: Vec<f32> = vec![0.0, 1.0, -1.0, 1234.56789, -1234.56789];
        let period: usize = 2;
        let discount_rate: f32 = 0.20;
        let expected_present_values: Vec<f32> = vec![0.00, 0.6944, -0.6944, 857.338, -857.338];
        let precision: f32 = 0.001;
        for index in 0..cash_flows.len() {
            let actual_pv: f32 = present_value(&cash_flows[index], period, &discount_rate);
            let expected_pv: f32 = expected_present_values[index];
            assert!(abs(actual_pv - expected_pv) <= precision);
        }
    }
}

#[allow(dead_code)]
/// Converts a series of cash flows and a discount rate into a present value.
///
/// # Example with f32
/// Assumptions
/// - Cash flows: [10.00]
/// - Discount rate: 10.00%
/// ```
/// use time_value::present_value::from_cash_flows_and_discount_rate;
///
/// let cash_flows: Vec<f32> = vec![10.0];
/// let discount_rate: f32 = 0.10;
/// assert_eq!(
///     cash_flows[0],
///    from_cash_flows_and_discount_rate(cash_flows.iter(), &discount_rate)
/// )
/// ```
///
/// # Example with f64
/// Assumptions
/// - Cash flows: [10.0, 10.0, 10.0]
/// - Discount rate: 10.00%
/// ```
/// use time_value::present_value::from_cash_flows_and_discount_rate;
/// use num::abs;
///
/// let cash_flows: Vec<f64> = vec![10.0, 10.0, 10.0];
/// let discount_rate: f64 = 0.10;
/// let value: f64 = from_cash_flows_and_discount_rate(cash_flows.iter(), &discount_rate);
/// let expected_value: f64 = 27.35;
/// assert!(abs(value - expected_value) < 0.01);
/// ```
pub fn from_cash_flows_and_discount_rate<T>(cash_flows: Iter<T>, discount_rate: &T) -> T
where
    T: Float + Product<T> + Sum<T>,
{
    cash_flows
        .enumerate()
        .map(|(period, cash_flow)| {
            crate::present_value::present_value(cash_flow, period, discount_rate)
        })
        .sum()
}

#[cfg(test)]
mod from_cash_flows_and_discount_rate_tests {
    use crate::present_value::from_cash_flows_and_discount_rate;
    use num::abs;

    #[test]
    fn it_works_with_a_positive_npv() {
        let cash_flows: Vec<f32> = vec![0.0, 1.0, -1.0, 1234.56789, -1234.56789];
        let discount_rate: f32 = 0.20;
        let precision: f32 = 0.001;
        let expected_value: f32 = 119.2137;
        let actual_value: f32 =
            from_cash_flows_and_discount_rate(cash_flows.iter(), &discount_rate);
        assert!(abs(expected_value - actual_value) <= precision);
    }

    #[test]
    fn it_works_with_a_negative_npv() {
        let cash_flows: Vec<f32> = vec![-500.0, 100.0, 2.0, 3.0, 4.0];
        let discount_rate: f32 = 0.30;
        let precision: f32 = 0.001;
        let expected_value: f32 = -419.1275;
        let actual_value: f32 =
            from_cash_flows_and_discount_rate(cash_flows.iter(), &discount_rate);
        assert!(abs(expected_value - actual_value) <= precision);
    }
}

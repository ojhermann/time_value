use num::{Float, Signed, abs};
use std::iter::{Product, Sum};
use std::fmt::{Display, Debug};
use std::slice::Iter;

use crate::irr::bisection::constants::NPV_PRECISION;
use crate::present_value::from_cash_flows_and_discount_rate as pv;

pub fn determine<T>(cash_flows: Iter<T>, rate_guess: &T, iteration_limit: &i16) -> Option<(T, T)>
    where
        T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    if abs(pv(cash_flows.clone(), &rate_guess)) < T::from(NPV_PRECISION).unwrap() {
        return Some((*rate_guess, *rate_guess));
    }

    let mut f: T = T::from(10.0).unwrap();
    let mut rate_low: T = *rate_guess - f * T::epsilon();
    let mut rate_high: T = *rate_guess + f * T::epsilon();
    let mut npv_low: T = pv(cash_flows.clone(), &rate_low);
    let mut npv_high: T = pv(cash_flows.clone(), &rate_high);

    if abs(npv_low) < abs(npv_high) {
        for _ in 0..*iteration_limit {
            if npv_low * npv_high <= T::zero() {
                return Some((rate_low, rate_high));
            }

            rate_high = rate_low;
            f = f * T::from(2.0).unwrap();
            rate_low = rate_low - f * T::epsilon();

            npv_low = pv(cash_flows.clone(), &rate_low);
            npv_high = pv(cash_flows.clone(), &rate_high);
        }
    } else {
        for _ in 0..*iteration_limit {
            if npv_low * npv_high <= T::zero() {
                return Some((rate_low, rate_high));
            }

            rate_low = rate_high;
            f = f * T::from(2.0).unwrap();
            rate_high = rate_high + rate_high * f * T::epsilon();

            npv_low = pv(cash_flows.clone(), &rate_low);
            npv_high = pv(cash_flows.clone(), &rate_high);
        }
    }

    None
}

#[cfg(test)]
mod determine_test {
    use crate::irr::bisection::functions::initial_bounds;

    #[test]
    fn it_works_with_a_good_guess() {
        let mut cash_flows: Vec<f32> = vec![-100.00];
        for _ in 0..10 {
            cash_flows.push(20.00);
        }

        let rate_guess: f32 = 0.150984;
        let iteration_limit: i16 = 0;

        let results: Option<(f32, f32)> = initial_bounds::determine(
            cash_flows.iter(),
            &rate_guess,
            &iteration_limit,
        );

        assert!(results.is_some())
    }

    #[test]
    fn it_works_with_a_bad_guess() {
        let mut cash_flows: Vec<f32> = vec![-100.00];
        for _ in 0..10 {
            cash_flows.push(20.00);
        }

        let rate_guess: f32 = 0.10;
        let iteration_limit: i16 = 0;

        let results: Option<(f32, f32)> = initial_bounds::determine(
            cash_flows.iter(),
            &rate_guess,
            &iteration_limit,
        );

        assert!(results.is_none())
    }

    #[test]
    fn it_works_with_a_low_guess() {
        let mut cash_flows: Vec<f32> = vec![-100.00];
        for _ in 0..10 {
            cash_flows.push(20.00);
        }

        let rate_guess: f32 = 0.10;
        let iteration_limit: i16 = 100;

        let results: Option<(f32, f32)> = initial_bounds::determine(
            cash_flows.iter(),
            &rate_guess,
            &iteration_limit,
        );

        assert!(results.is_some())
    }

    #[test]
    fn it_works_with_a_high_guess() {
        let mut cash_flows: Vec<f32> = vec![-100.00];
        for _ in 0..10 {
            cash_flows.push(20.00);
        }

        let rate_guess: f32 = 0.2;
        let iteration_limit: i16 = 100;

        let results: Option<(f32, f32)> = initial_bounds::determine(
            cash_flows.iter(),
            &rate_guess,
            &iteration_limit,
        );

        assert!(results.is_some())
    }
}
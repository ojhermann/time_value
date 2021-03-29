//! Calculate the IRR of a series of cash flows with the bisection method.

use num::{abs, Float, Signed};
use std::iter::{Product, Sum};

use crate::irr::bisection::constants::NPV_PRECISION;
use crate::irr::bisection::functions::are_equal_enough;
use crate::irr::bisection::functions::midpoint;
use crate::irr::bisection::structs::irr::Irr;
use crate::present_value::from_cash_flows_and_discount_rate as pv;
use std::fmt::{Debug, Display};
use std::slice::Iter;

/// An implementation of the bisection root finding algorithm for calculating the IRR of a series of cash flows.
///
/// # Assumptions
/// It is assumed that the user has found two rates such that their respective NPVs have values of opposite signs i.e. `rate_low_guess * rate_high_guess < 0.0`
///
/// # Comments
/// A function for finding initial values may be added soon.
///
/// # Example with f32
/// ```
/// use time_value::irr::bisection::constants::NPV_PRECISION;
/// use time_value::irr::bisection::structs::irr::Irr;
/// use time_value::irr::bisection::functions::irr::{ bisection as irr};
///
/// let cash_flows: Vec<f32> = vec![-100.0, 50.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0,];
/// let rate_low: f32 = 0.05;
/// let rate_high: f32 = 0.18;
/// let iteration_limit: i16 = 100;
/// let calculated_irr: Irr<f32> = irr(cash_flows.iter(), &rate_low, &rate_high, &iteration_limit);
/// assert!(calculated_irr.is_valid());
/// assert!(calculated_irr.get_npv() <= NPV_PRECISION);
/// ```
///
/// # Example with f64
/// ```
/// use time_value::irr::bisection::constants::NPV_PRECISION;
/// use time_value::irr::bisection::structs::irr::Irr;
/// use time_value::irr::bisection::functions::irr::{ bisection as irr};
///
/// let cash_flows: Vec<f64> = vec![-122.3990963, 24.26782424, -18.61877741, -2.555946884, -8.814622596, 32.05035057, 12.11973328, 7.743486592, 9.158469173, -21.97032692, 11.18895709];
/// let rate_low: f64 = -0.25;
/// let rate_high: f64 = 0.25;
/// let iteration_limit: i16 = 100;
/// let calculated_irr: Irr<f64> = irr(cash_flows.iter(), &rate_low, &rate_high, &iteration_limit);
/// assert!(calculated_irr.is_valid());
/// assert!(calculated_irr.get_npv() <= f64::from(NPV_PRECISION));
/// ```
pub fn bisection<T>(
    cash_flows: Iter<T>,
    rate_low_guess: &T,
    rate_high_guess: &T,
    iteration_limit: &i16,
) -> Irr<T>
where
    T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    let mut rate_low: T = *rate_low_guess;
    let mut rate_high: T = *rate_high_guess;

    let mut npv_rate_low: T = pv(cash_flows.clone(), &rate_low);
    let mut npv_rate_high: T = pv(cash_flows.clone(), &rate_high);

    if T::zero() < npv_rate_low * npv_rate_high {
        return Irr::new(
            rate_low,
            npv_rate_low,
            rate_high,
            npv_rate_high,
            *iteration_limit,
            0,
            T::nan(),
            T::nan(),
            false,
        );
    }

    let mut irr: T = midpoint::calculate(&rate_low, &rate_high);
    let mut npv: T = pv(cash_flows.clone(), &irr);
    let mut iterations_run: i16 = 0;
    let precision: T = T::from(NPV_PRECISION).unwrap();

    while iterations_run < *iteration_limit && !are_equal_enough::is_true(&precision, &npv) {
        iterations_run += 1;

        if npv_rate_low * npv < T::zero() {
            rate_high = irr;
            npv_rate_high = npv;
        } else {
            rate_low = irr;
            npv_rate_low = npv;
        }

        irr = midpoint::calculate(&rate_low, &rate_high);
        npv = pv(cash_flows.clone(), &irr);
    }

    Irr::new(
        rate_low,
        npv_rate_low,
        rate_high,
        npv_rate_high,
        *iteration_limit,
        iterations_run,
        irr,
        npv,
        abs(npv) <= T::from(NPV_PRECISION).unwrap(),
    )
}

#[cfg(test)]
mod bisection_tests {
    use crate::irr::bisection::functions::initial_bounds;
    use crate::irr::bisection::functions::irr::{bisection as irr, Irr, NPV_PRECISION};
    use crate::irr::bisection::structs::initial_bounds::InitialBounds;

    use num::{Float, Signed};
    use rand::distributions::uniform::SampleUniform;
    use rand::prelude::ThreadRng;
    use rand::{thread_rng, Rng};
    use std::fmt::{Debug, Display};
    use std::iter::{Product, Sum};

    fn generate_random_cash_flows<T>(thread_range: &mut ThreadRng, vector_size: &i16) -> Vec<T>
    where
        T: Float + Product<T> + Sum<T> + Signed + Display + Debug + SampleUniform,
    {
        //ensure the first element is negative
        let mut cash_flows: Vec<T> =
            vec![thread_range.gen_range(T::from(-100.0).unwrap()..T::from(-1.0).unwrap())];
        for _ in 0..(vector_size - 1) {
            cash_flows
                .push(thread_range.gen_range(T::from(-50.0).unwrap()..T::from(50.0).unwrap()));
        }
        cash_flows
    }

    #[test]
    fn it_works_with_random_inputs() {
        let mut thread_range: ThreadRng = thread_rng();
        let vector_size: i16 = 20;
        let rate_guess: f32 = 0.05;
        let iteration_limit: i16 = 1_000;
        let mut cash_flows: Vec<f32> = generate_random_cash_flows(&mut thread_range, &vector_size);

        for _ in 0..100 {
            let initial_bounds: InitialBounds<f32> =
                initial_bounds::determine(cash_flows.iter(), &rate_guess, &iteration_limit);

            if initial_bounds.is_valid() {
                let calculated_irr: Irr<f32> = irr(
                    cash_flows.iter(),
                    &initial_bounds.get_rate_low(),
                    &initial_bounds.get_rate_high(),
                    &iteration_limit,
                );

                if calculated_irr.is_valid() {
                    assert!(calculated_irr.get_npv() <= NPV_PRECISION);
                } else {
                    if calculated_irr.get_iterations_run() == 0 {
                        assert!(calculated_irr.get_irr().is_nan())
                    } else {
                        assert!(calculated_irr.get_iterations_run() <= iteration_limit)
                    }
                }
            }

            cash_flows = generate_random_cash_flows(&mut thread_range, &vector_size);
        }
    }

    #[test]
    fn it_works_on_known_example_0_f32() {
        let cash_flows: Vec<f32> = vec![
            -100.0, 50.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0,
        ];
        let rate_low_guess: f32 = 0.05;
        let rate_high_guess: f32 = 0.18;
        let iteration_limit: i16 = 100;
        let irr_approximation: Irr<f32> = irr(
            cash_flows.iter(),
            &rate_low_guess,
            &rate_high_guess,
            &iteration_limit,
        );

        assert!(irr_approximation.is_valid());
        assert!(irr_approximation.get_npv() <= NPV_PRECISION);
    }

    #[test]
    fn it_works_on_known_example_0_f64() {
        let cash_flows: Vec<f64> = vec![
            -100.0, 50.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0,
        ];
        let rate_low_guess: f64 = 0.05;
        let rate_high_guess: f64 = 0.18;
        let iteration_limit: i16 = 100;
        let calculated_irr: Irr<f64> = irr(
            cash_flows.iter(),
            &rate_low_guess,
            &rate_high_guess,
            &iteration_limit,
        );

        assert!(calculated_irr.is_valid());
        assert!(calculated_irr.get_npv() <= f64::from(NPV_PRECISION));
    }

    #[test]
    fn it_works_on_known_example_1_f32() {
        let cash_flows: Vec<f32> = vec![
            -35.48821127,
            -8.172027706,
            -43.71313035,
            87.28622638,
            11.09652325,
            7.156975747,
            -55.68307465,
            -31.48959668,
            -6.008830411,
            44.02311388,
            39.82177996,
        ];
        let rate_low_guess: f32 = 0.01;
        let rate_high_guess: f32 = 0.05;
        let iteration_limit: i16 = 100;
        let calculated_irr: Irr<f32> = irr(
            cash_flows.iter(),
            &rate_low_guess,
            &rate_high_guess,
            &iteration_limit,
        );

        assert!(calculated_irr.is_valid());
        assert!(calculated_irr.get_npv() <= NPV_PRECISION);
    }

    #[test]
    fn it_works_on_known_example_1_f64() {
        let cash_flows: Vec<f64> = vec![
            -35.48821127,
            -8.172027706,
            -43.71313035,
            87.28622638,
            11.09652325,
            7.156975747,
            -55.68307465,
            -31.48959668,
            -6.008830411,
            44.02311388,
            39.82177996,
        ];
        let rate_low_guess: f64 = 0.01;
        let rate_high_guess: f64 = 0.05;
        let iteration_limit: i16 = 100;
        let calculated_irr: Irr<f64> = irr(
            cash_flows.iter(),
            &rate_low_guess,
            &rate_high_guess,
            &iteration_limit,
        );

        assert!(calculated_irr.is_valid());
        assert!(calculated_irr.get_npv() <= f64::from(NPV_PRECISION));
    }

    #[test]
    fn it_works_on_known_example_2_f32() {
        let cash_flows: Vec<f32> = vec![
            -122.3990963,
            24.26782424,
            -18.61877741,
            -2.555946884,
            -8.814622596,
            32.05035057,
            12.11973328,
            7.743486592,
            9.158469173,
            -21.97032692,
            11.18895709,
        ];
        let rate_low_guess: f32 = -0.25;
        let rate_high_guess: f32 = 0.25;
        let iteration_limit: i16 = 100;
        let calculated_irr: Irr<f32> = irr(
            cash_flows.iter(),
            &rate_low_guess,
            &rate_high_guess,
            &iteration_limit,
        );

        assert!(calculated_irr.is_valid());
        assert!(calculated_irr.get_npv() <= NPV_PRECISION);
    }

    #[test]
    fn it_works_on_known_example_2_f64() {
        let cash_flows: Vec<f64> = vec![
            -122.3990963,
            24.26782424,
            -18.61877741,
            -2.555946884,
            -8.814622596,
            32.05035057,
            12.11973328,
            7.743486592,
            9.158469173,
            -21.97032692,
            11.18895709,
        ];
        let rate_low_guess: f64 = -0.25;
        let rate_high_guess: f64 = 0.25;
        let iteration_limit: i16 = 100;
        let calculated_irr: Irr<f64> = irr(
            cash_flows.iter(),
            &rate_low_guess,
            &rate_high_guess,
            &iteration_limit,
        );

        assert!(calculated_irr.is_valid());
        assert!(calculated_irr.get_npv() <= f64::from(NPV_PRECISION));
    }
}

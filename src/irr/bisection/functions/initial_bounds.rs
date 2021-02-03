use num::{Float, Signed, abs};
use std::iter::{Product, Sum};
use std::fmt::{Display, Debug};
use std::slice::Iter;

use crate::irr::bisection::constants::NPV_PRECISION;
use crate::irr::bisection::structs::initial_bounds::InitialBounds;
use crate::present_value::from_cash_flows_and_discount_rate as pv;

pub fn determine<T>(cash_flows: Iter<T>, rate_guess: &T, iteration_limit: &i16) -> InitialBounds<T>
    where
        T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    let npv_rate_guess: T = pv(cash_flows.clone(), &rate_guess);
    if abs(pv(cash_flows.clone(), &rate_guess)) < T::from(NPV_PRECISION).unwrap() {
        return InitialBounds::new(
            *rate_guess,
            npv_rate_guess,
            *rate_guess,
            npv_rate_guess,
            *iteration_limit,
            0,
            true,
        );
    }

    let mut epsilon_multiple: T = T::from(10.00).unwrap();
    let mut rate_low: T = *rate_guess - epsilon_multiple * T::epsilon();
    let mut rate_high: T = *rate_guess + epsilon_multiple * T::epsilon();
    let mut npv_rate_low: T = pv(cash_flows.clone(), &rate_low);
    let mut npv_rate_high: T = pv(cash_flows.clone(), &rate_high);
    let mut iterations_run: i16 = 0;
    let go_low: bool = abs(npv_rate_low) < abs(npv_rate_high);

    while iterations_run < *iteration_limit {
        if npv_rate_low * npv_rate_high <= T::zero() {
            return InitialBounds::new(
                rate_low,
                npv_rate_low,
                rate_high,
                npv_rate_high,
                *iteration_limit,
                iterations_run,
                true,
            );
        }

        epsilon_multiple = generate_epsilon_multiple(epsilon_multiple);

        if go_low {
            rate_high = rate_low;
            rate_low = rate_low - epsilon_multiple * T::epsilon();
        } else {
            rate_low = rate_high;
            rate_high = rate_high + epsilon_multiple * T::epsilon();
        }

        npv_rate_low = pv(cash_flows.clone(), &rate_low);
        npv_rate_high = pv(cash_flows.clone(), &rate_high);

        iterations_run = iterations_run + 1;
    }

    InitialBounds::new(
        rate_low,
        npv_rate_low,
        rate_high,
        npv_rate_high,
        *iteration_limit,
        iterations_run,
        false,
    )
}

fn generate_epsilon_multiple<T>(epsilon_multiple: T) -> T
    where
        T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    if epsilon_multiple < T::max_value() / T::from(2.0).unwrap() {
        epsilon_multiple * T::from(2.0).unwrap()
    } else {
        T::max_value()
    }
}

#[cfg(test)]
mod determine_test {
    use num::{Float, Signed};
    use rand::distributions::uniform::SampleUniform;
    use rand::prelude::ThreadRng;
    use rand::{thread_rng, Rng};
    use std::fmt::{Display, Debug};
    use std::iter::{Product, Sum};

    use crate::irr::bisection::functions::initial_bounds;
    use crate::irr::bisection::structs::initial_bounds::InitialBounds;

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
        let rate_guess: f32 = 0.01;
        let iteration_limit: i16 = 1_000;
        let mut cash_flows: Vec<f32> = generate_random_cash_flows(&mut thread_range, &vector_size);

        for _ in 0..100 {
            let initial_bounds: InitialBounds<f32> = initial_bounds::determine(
                cash_flows.iter(),
                &rate_guess,
                &iteration_limit,
            );

            if initial_bounds.is_valid() {
                assert!(initial_bounds.get_npv_rate_low() * initial_bounds.get_npv_rate_high() <= 0.00);
            } else {
                assert_eq!(initial_bounds.get_iteration_limit(), initial_bounds.get_iterations_run());
            }

            cash_flows = generate_random_cash_flows(&mut thread_range, &vector_size)
        }
    }

    #[test]
    fn it_works_with_a_good_guess() {
        let mut cash_flows: Vec<f32> = vec![-100.00];
        for _ in 0..10 {
            cash_flows.push(20.00);
        }

        let rate_guess: f32 = 0.150984;
        let iteration_limit: i16 = 0;

        let initial_bounds: InitialBounds<f32> = initial_bounds::determine(
            cash_flows.iter(),
            &rate_guess,
            &iteration_limit,
        );

        assert!(initial_bounds.is_valid())
    }

    #[test]
    fn it_works_with_a_bad_guess() {
        let mut cash_flows: Vec<f32> = vec![-100.00];
        for _ in 0..10 {
            cash_flows.push(20.00);
        }

        let rate_guess: f32 = 0.10;
        let iteration_limit: i16 = 0;

        let initial_bounds: InitialBounds<f32> = initial_bounds::determine(
            cash_flows.iter(),
            &rate_guess,
            &iteration_limit,
        );

        assert!(!initial_bounds.is_valid())
    }

    #[test]
    fn it_works_with_a_low_guess() {
        let mut cash_flows: Vec<f32> = vec![-100.00];
        for _ in 0..10 {
            cash_flows.push(20.00);
        }

        let rate_guess: f32 = 0.10;
        let iteration_limit: i16 = 100;

        let initial_bounds: InitialBounds<f32> = initial_bounds::determine(
            cash_flows.iter(),
            &rate_guess,
            &iteration_limit,
        );

        assert!(initial_bounds.is_valid())
    }

    #[test]
    fn it_works_with_a_high_guess() {
        let mut cash_flows: Vec<f32> = vec![-100.00];
        for _ in 0..10 {
            cash_flows.push(20.00);
        }

        let rate_guess: f32 = 0.2;
        let iteration_limit: i16 = 100;

        let initial_bounds: InitialBounds<f32> = initial_bounds::determine(
            cash_flows.iter(),
            &rate_guess,
            &iteration_limit,
        );

        assert!(initial_bounds.is_valid())
    }
}
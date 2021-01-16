use num::{abs, Float, Signed};
use std::iter::{Product, Sum};
use std::slice::Iter;

use crate::present_value::from_cash_flows_and_discount_rate as pv;

fn generate_initial_bounds<T>(cash_flows: Iter<T>, guess: &T, allowed_error: &T) -> (T, T)
where
    T: Float + Product<T> + Sum<T> + Signed,
{
    let guess_npv: T = pv(cash_flows.clone(), guess);
    if abs(guess_npv) <= *allowed_error {
        return (*guess, *guess);
    }

    let mut guess_less: T = *guess - abs(*guess);
    let mut guess_less_npv: T = pv(cash_flows.clone(), &guess_less);
    if guess_npv * guess_less_npv <= T::zero() {
        return (guess_less, *guess);
    }

    let mut guess_more: T = *guess + abs(*guess);
    let mut guess_more_npv: T = pv(cash_flows.clone(), &guess_more);
    if guess_npv * guess_more_npv <= T::zero() {
        return (*guess, guess_more);
    }

    if abs(guess_less_npv) < abs(guess_more_npv) {
        // all guesses have the same sign, so we iterate over the guess brining us closer to zero
        loop {
            guess_less = guess_less - abs(*guess);
            guess_less_npv = pv(cash_flows.clone(), &guess_less);
            if guess_less_npv * guess_npv <= *allowed_error {
                return (guess_less, guess_less + abs(*guess));
            }
        }
    } else {
        loop {
            guess_more = guess_more + abs(*guess);
            guess_more_npv = pv(cash_flows.clone(), &guess_more);
            if guess_more_npv * guess_npv <= *allowed_error {
                return (guess_more - abs(*guess), guess_more);
            }
        }
    }
}

#[cfg(test)]
mod generate_initial_bounds_tests {
    use crate::irr::generate_initial_bounds;
    use crate::present_value::from_cash_flows_and_discount_rate as pv;
    use num::abs;

    #[test]
    fn it_works_if_passed_irr() {
        let cash_flows: Vec<f32> = vec![-100.0, 50.0, 10.0, 0.0, 5.0];
        let discount_rate: f32 = -0.24888;
        let precision: f32 = 0.001;

        let present_value: f32 = pv(cash_flows.iter(), &discount_rate);
        assert!(abs(present_value) <= precision);

        let (left, right): (f32, f32) =
            generate_initial_bounds(cash_flows.iter(), &discount_rate, &precision);
        assert_eq!(left, right);
        assert_eq!(left, discount_rate);
    }

    #[test]
    fn it_can_solve_from_a_low_guess() {
        // positive IRR
        let cash_flows: Vec<f32> = vec![-100.0, 50.0, 50.0, 50.0, 50.0];
        let guess: f32 = 0.25;
        let allowed_error: f32 = 0.001;

        let (left, right): (f32, f32) =
            generate_initial_bounds(cash_flows.iter(), &guess, &allowed_error);
        assert!(left < right);

        let left_npv: f32 = pv(cash_flows.iter(), &left);
        let right_npv: f32 = pv(cash_flows.iter(), &right);
        assert!(left_npv * right_npv <= 0.0);

        //negative IRR
        let cash_flows: Vec<f32> = vec![-100.0, 50.0, 10.0, 0.0, 5.0];
        let guess: f32 = -0.35;
        let allowed_error: f32 = 0.001;

        let (left, right): (f32, f32) =
            generate_initial_bounds(cash_flows.iter(), &guess, &allowed_error);
        assert!(left < right);

        let left_npv: f32 = pv(cash_flows.iter(), &left);
        let right_npv: f32 = pv(cash_flows.iter(), &right);
        assert!(left_npv * right_npv <= 0.0);
    }

    #[test]
    fn it_can_solve_from_a_high_guess() {
        // positive IRR
        let cash_flows: Vec<f32> = vec![-100.0, 50.0, 50.0, 50.0, 50.0];
        let guess: f32 = 0.50;
        let allowed_error: f32 = 0.001;

        let (left, right): (f32, f32) =
            generate_initial_bounds(cash_flows.iter(), &guess, &allowed_error);
        assert!(left < right);

        let left_npv: f32 = pv(cash_flows.iter(), &left);
        let right_npv: f32 = pv(cash_flows.iter(), &right);
        assert!(left_npv * right_npv <= 0.0);

        //negative IRR
        let cash_flows: Vec<f32> = vec![-100.0, 50.0, 10.0, 0.0, 5.0];
        let guess: f32 = 0.15;
        let allowed_error: f32 = 0.001;

        let (left, right): (f32, f32) =
            generate_initial_bounds(cash_flows.iter(), &guess, &allowed_error);
        assert!(left < right);

        let left_npv: f32 = pv(cash_flows.iter(), &left);
        let right_npv: f32 = pv(cash_flows.iter(), &right);
        assert!(left_npv * right_npv <= 0.0);
    }
}

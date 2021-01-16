use num::{abs, Float, Signed};
use std::fmt;
use std::iter::{Product, Sum};
use std::slice::Iter;

use crate::present_value::from_cash_flows_and_discount_rate as pv;

pub fn is_irr<T>(guess_npv: &T, allowed_error: &T) -> bool
where
    T: Float + Product<T> + Sum<T> + Signed + fmt::Display,
{
    abs(*guess_npv) <= *allowed_error
}

pub fn generate_initial_bounds<T>(cash_flows: Iter<T>, guess: &T, allowed_error: &T) -> (T, T)
where
    T: Float + Product<T> + Sum<T> + Signed + fmt::Display,
{
    let guess_npv: T = pv(cash_flows.clone(), guess);
    if is_irr(&guess_npv, allowed_error) {
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

// #[cfg(test)]
// mod is_irr_tests {
//     use crate::present_value::from_cash_flows_and_discount_rate as pv;
//     use crate::irr::is_irr;
//
//     #[test]
//     fn it_works_for_correct_guesses() {
//         let cash_flows: Vec<f64> = vec![-100.00, 50.00, 50.00, 50.00, 50.00, 50.00, 50.00];
//         let guess: f64 = 0.46557;
//         let allowed_error: f64 = 0.01;
//         let npv: f64 = pv(cash_flows.iter(), &guess);
//         assert_eq!(npv, 0.0);
//         assert!(is_irr(&npv, &allowed_error));
//     }
// }

#[cfg(test)]
mod generate_initial_bounds_tests {
    use crate::irr::generate_initial_bounds;

    #[test]
    fn it_works_if_passed_irr() {

    }

    #[test]
    fn it_works_from_a_low_guess_if_found_before_iteration() {

    }

    #[test]
    fn it_works_from_a_low_guess_if_found_via_iteration() {
        let cash_flows: Vec<f32> = vec![-100.00, 50.00, 50.00, 50.00, 50.00, 50.00, 50.00];
        let guess: f32 = 0.46557;
        let allowed_error: f32 = 0.01;
        let (left, right): (f32, f32) = generate_initial_bounds(cash_flows.iter(), &guess, &allowed_error);
        assert!(left <= right);
        assert_eq!((left, right), (1.0, 2.0));
    }

    #[test]
    fn it_works_from_a_high_guess_if_found_before_iteration() {

    }

    #[test]
    fn it_works_from_a_high_guess_found_via_iteration() {

    }
    //
    // #[test]
    // fn it_works_from_a_low_guess_which_means_it_pursues_guess_more() {
    //     let cash_flows: Vec<f32> = vec![-100.00, 50.00, 50.00, 50.00, 50.00, 50.00, 50.00];
    //     let mut guess: f32 = 0.10;
    //     let allowed_error: f32 = 0.01;
    //     let (left, right): (f32, f32) = generate_initial_bounds(cash_flows.iter(), &guess, &allowed_error);
    //     assert!(left <= right);
    //     assert_eq!(left, 0.4);
    //     assert_eq!(right, 0.5);
    //
    //     guess = 0.30;
    //     let (left, right): (f32, f32) = generate_initial_bounds(cash_flows.iter(), &guess, &allowed_error);
    //     assert!(left <= right);
    //     assert_eq!(left, 0.6);
    //     assert_eq!(right, 0.6);
    // }
}

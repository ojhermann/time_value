use crate::present_value;
use num::{abs, Float, Unsigned};
use std::iter::{Product, Sum};
use std::slice::Iter;
use crate::present_value::from_cash_flows_and_discount_rate;

pub fn bisection<T, U>(cash_flows: Iter<T>, discount_rate_guess: &T, tolerance: &T) -> T
where
    T: Float + Product<T> + Sum<T>,
    U: Iterator
{
    let mut discount_rate: T = *discount_rate_guess;
    let mut npv: T = present_value::from_cash_flows_and_discount_rate(cash_flows, &discount_rate);

    while tolerance < &npv {}

    discount_rate
}

fn generate_bounds<T>(
    cash_flows: Iter<T>,
    discount_rate_guess: &T,
    tolerance: &T,
    limit: i16,
) -> (T, T)
where
    T: Float + Product<T> + Sum<T>,
{
    let (mut left, mut right): (T, T) = generate_initial_bounds(discount_rate_guess);

    let mut count: i16 = 0;


    while count < limit {
        count += 1;

    }

    (left, right)
}

fn generate_initial_bounds<T>(discount_rate_guess: &T) -> (T, T)
where
    T: Float + Product<T> + Sum<T>,
{
    if discount_rate_guess == &T::zero() {
        return (-T::one(), T::one());
    }

    let mut left: T;
    let mut right: T;
    if discount_rate_guess < &T::zero() {
        left = *discount_rate_guess;
        right = *discount_rate_guess * -T::one();
    } else {
        left = *discount_rate_guess * -T::one();
        right = *discount_rate_guess;
    }

    (left, right)
}

#[cfg(test)]
mod generate_initial_bounds_tests {
    use crate::irr::generate_initial_bounds;

    #[test]
    fn it_works_for_zero_as_input() {
        for b in vec![0.0] {
            assert_eq!((-1.0, 1.0), generate_initial_bounds(&b));
        }
    }

    #[test]
    fn it_works_for_positive_inputs() {
        for b in vec![0.1, 0.2, 0.3, 0.4, 0.5] {
            assert_eq!((-b, b), generate_initial_bounds(&b));
        }
    }

    #[test]
    fn it_works_for_negative_inputs() {
        for b in vec![-0.1, -0.2, -0.3, -0.4, -0.5] {
            assert_eq!((b, -b), generate_initial_bounds(&b));
        }
    }
}

//! Obtain initial bounds for the bisection method.

use num::{abs, Float, Signed, Zero};
use std::iter::{Product, Sum};

use crate::present_value::from_cash_flows_and_discount_rate as pv;
use std::fmt::{Debug, Display, Error, Formatter};
use std::slice::Iter;
use crate::irr::bisection::function::are_equal_enough;


//todo create and return a struct
pub fn calculate_initial_bounds_for_bisection<T>(
    cash_flows: Iter<T>,
    guess: &T,
    iteration_limit: &i16,
) -> Option<(T, T)>
    where
        T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    let mut rate: T = *guess;
    let mut value: T = pv(cash_flows.clone(), &rate);
    if are_equal_enough(&value, &T::zero()) {
        return Some((rate, rate));
    }
    // early return if iteration is zero, but after making struct

    let mut left: T = shift_rate(&rate, true);
    let mut value_left: T = pv(cash_flows.clone(), &left);
    if value * value_left <= T::zero() {
        return Some((left, rate));
    }

    let mut right: T =
        shift_rate(&rate, false);
    let mut value_right: T = pv(cash_flows.clone(), &right);
    if value * value_right <= T::zero() {
        return Some((rate, right));
    }

    let mut count: i16 = 1;
    while count < *iteration_limit {
        count = count + 1;

        let shift_left: bool = (value.is_negative() && (value_right < value_left))
            || (value.is_positive() && (value_left < value_right));

        if shift_left {
            right = rate;
            rate = left;
            left = shift_rate(&left, shift_left);

            value_right = value;
            value = value_left;
            value_left = pv(cash_flows.clone(), &left);
        } else {
            left = rate;
            rate = right;
            right = shift_rate(&right, shift_left);

            value_left = value;
            value = value_right;
            value_right = pv(cash_flows.clone(), &right);
        }

        if are_equal_enough(&value, &T::zero()) {
            return Some((rate, rate));
        }

        if value * value_left <= T::zero() {
            return Some((left, rate));
        }

        if value * value_right <= T::zero() {
            return Some((rate, right));
        }
    }

    None
}

fn shift_rate<T>(rate: &T, shift_left: bool) -> T
    where
        T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    if rate.is_negative() {
        match shift_left {
            true => *rate * T::from(2.0).unwrap(),
            false => *rate * T::from(-2.0).unwrap(),
        }
    } else if rate.is_zero() {
        match shift_left {
            true => -T::one(),
            false => T::one(),
        }
    } else {
        match shift_left {
            true => *rate * T::from(-2.0).unwrap(),
            false => *rate * T::from(2.0).unwrap(),
        }
    }
}
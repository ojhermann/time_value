//! Determines if two floating point numbers are "equal enough" based on machine epsilon

use num::{abs, Float, Signed};
use std::fmt::{Debug, Display};
use std::iter::{Product, Sum};

/// # Comments
/// Hat tip to [Bruce Dawson](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
///
/// # Example with f32
/// ```
/// use time_value::irr::bisection::functions::are_equal_enough;
///
/// let a: f32 = 0.0010;
/// let b: f32 = 0.0010;
/// assert!(are_equal_enough::is_true(&a, &b));
///
/// let c: f32 = 0.0011;
/// assert!(!are_equal_enough::is_true(&a, &c));
/// ```
pub fn is_true<T>(a: &T, b: &T) -> bool
where
    T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    let difference: T = abs(*a - *b);
    let a_abs: T = abs(*a);
    let b_abs: T = abs(*b);
    let larger: T = if a_abs < b_abs { b_abs } else { a_abs };

    difference <= (larger * T::epsilon())
}

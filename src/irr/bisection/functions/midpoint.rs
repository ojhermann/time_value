//! Calculates the mid point of two floating point numbers.

use num::{Float, Signed};
use std::iter::{Product, Sum};

/// # Comments
/// Done in a way that avoids potential overflow that can occurs when using (a + b)/2
///
/// # Example with f32
/// ```
/// use time_value::irr::bisection::functions::are_equal_enough;
/// use time_value::irr::bisection::functions::midpoint;
///
/// let a: f32 = 1.0;
/// let b: f32 = 2.0;
/// let mid_point: f32 = midpoint::calculate(&a, &b);
/// assert!(are_equal_enough::is_true(&mid_point, &1.5));
/// ```
pub fn calculate<T>(a: &T, c: &T) -> T
where
    T: Float + Product<T> + Sum<T> + Signed,
{
    *a + (*c - *a) / T::from(2.0).unwrap()
}

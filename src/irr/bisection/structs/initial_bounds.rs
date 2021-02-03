//! A struct containing information for determining the initial bounds for use with the bisection method.

use num::{Float, Signed};
use std::fmt::{Debug, Display, Error, Formatter};
use std::iter::{Product, Sum};

/// # Example
/// ```
/// use time_value::irr::bisection::structs::initial_bounds::InitialBounds;
///
/// let initial_bounds = InitialBounds::new(
/// 0.01,
/// 1.00,
/// 0.02,
/// 2.00,
/// 100,
/// 24,
/// true,
/// );
/// ```
pub struct InitialBounds<T>
where
    T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    rate_low: T,
    npv_rate_low: T,
    rate_high: T,
    npv_rate_high: T,
    iteration_limit: i16,
    iterations_run: i16,
    is_valid: bool,
}

impl<T> InitialBounds<T>
where
    T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    pub fn new(
        rate_low: T,
        npv_rate_low: T,
        rate_high: T,
        npv_rate_high: T,
        iteration_limit: i16,
        iterations_run: i16,
        is_valid: bool,
    ) -> InitialBounds<T> {
        InitialBounds {
            rate_low,
            npv_rate_low,
            rate_high,
            npv_rate_high,
            iteration_limit,
            iterations_run,
            is_valid,
        }
    }

    pub fn get_rate_low(&self) -> T {
        self.rate_low
    }

    pub fn get_npv_rate_low(&self) -> T {
        self.npv_rate_low
    }

    pub fn get_rate_high(&self) -> T {
        self.rate_high
    }

    pub fn get_npv_rate_high(&self) -> T {
        self.npv_rate_high
    }

    pub fn get_iteration_limit(&self) -> i16 {
        self.iteration_limit
    }

    pub fn get_iterations_run(&self) -> i16 {
        self.iterations_run
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}

impl<T> Debug for InitialBounds<T>
where
    T: Float + Product<T> + Sum<T> + Signed + Debug + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("InitialBounds")
            .field("rate_low", &self.get_rate_low())
            .field("npv_rate_low", &self.get_npv_rate_low())
            .field("rate_high", &self.get_rate_high())
            .field("npv_rate_high", &self.get_npv_rate_high())
            .field("iteration_limit", &self.get_iteration_limit())
            .field("iterations_run", &self.get_iterations_run())
            .field("is_valid", &self.is_valid())
            .finish()
    }
}

impl<T> Display for InitialBounds<T>
where
    T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "rate_low: {}\nnpv_rate_low: {}\nrate_high: {}\nnpv_rate_high: {}\niteration_limit: {}\n iterations_run: {}\nis_valid: {}\n",
            self.get_rate_low(),
            self.get_npv_rate_low(),
            self.get_rate_high(),
            self.get_npv_rate_high(),
            self.get_iteration_limit(),
            self.get_iterations_run(),
            self.is_valid()
        )
    }
}

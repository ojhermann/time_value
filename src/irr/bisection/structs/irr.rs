//! A struct containing information for determining the IRR of a series of cash flows using the bisection method.

use num::{Float, Signed};
use std::iter::{Product, Sum};

use std::fmt::{Debug, Display, Error, Formatter};

/// Contains information useful to finding the IRR of a given cash flow series.
///
/// # Example: taking a punt at the IRR
/// ```
/// use time_value::present_value::from_cash_flows_and_discount_rate as pv;
/// use time_value::irr::bisection::structs::irr::Irr;
/// use time_value::irr::bisection::functions::midpoint;
/// use std::slice::Iter;
/// use num::abs;
/// use time_value::irr::bisection::constants::NPV_PRECISION;
///
///  let cash_flows: Vec<f32> = vec![-100.0, 50.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0];
///
/// let rate_one_guess: f32 = 0.01;
/// let rate_two_guess: f32 = 0.05;
/// let rate_guess: f32 = midpoint::calculate(&rate_one_guess, &rate_two_guess);
///
/// let npv_guess_one: f32 = pv(cash_flows.iter(), &rate_one_guess);
/// let npv_guess_two: f32 = pv(cash_flows.iter(), &rate_two_guess);
/// let npv_guess: f32 =  pv(cash_flows.iter(), &rate_guess);
///
/// let iteration_limit: i16 = 0;
///
/// let is_valid: bool = abs(npv_guess) <= NPV_PRECISION;
///
/// let irr_guess: Irr<f32> = Irr::new(
///             rate_one_guess,
///             npv_guess_one,
///             rate_two_guess,
///             npv_guess_two,
///             0,
///             0,
///             rate_guess,
///             npv_guess,
///             is_valid,
///         );
/// ```
///
/// # Example: Irr methods
///
/// ```
/// use time_value::irr::bisection::structs::irr::Irr;
///
/// let rate_one_guess: f32 = 0.01;
/// let rate_two_guess: f32 = 0.02;
/// let rate_guess: f32 = 0.015;
///
/// let npv_one_guess: f32 = 1.0;
/// let npv_two_guess: f32 = 2.0;
/// let npv_guess: f32 = 1.5;
///
/// let iteration_limit: i16 = 4;
/// let iterations_run: i16 = 3;
///
/// let is_valid: bool = false;
///
/// let irr_guess: Irr<f32> = Irr::new(
///             rate_one_guess,
///             npv_one_guess,
///             rate_two_guess,
///             npv_two_guess,
///             iteration_limit,
///             iterations_run,
///             rate_guess,
///             npv_guess,
///             false,
///         );
///
/// assert_eq!(irr_guess.rate_low(), rate_one_guess);
/// assert_eq!(irr_guess.get_rate_high(), rate_two_guess);
/// assert_eq!(irr_guess.get_npv_rate_low(), npv_one_guess);
/// assert_eq!(irr_guess.get_npv_rate_high(), npv_two_guess);
/// assert_eq!(irr_guess.get_irr(), rate_guess);
/// assert_eq!(irr_guess.get_iteration_limit(), iteration_limit);
/// assert_eq!(irr_guess.get_iterations_run(), iterations_run);
/// assert_eq!(irr_guess.get_npv(), npv_guess);
/// assert!(!irr_guess.is_valid());
/// ```
pub struct Irr<T>
where
    T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    rate_low: T,
    npv_rate_low: T,
    rate_high: T,
    npv_rate_high: T,
    iteration_limit: i16,
    iterations_run: i16,
    irr: T,
    npv: T,
    is_valid: bool,
}

impl<T> Irr<T>
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
        irr: T,
        npv: T,
        is_valid: bool,
    ) -> Irr<T> {
        Irr {
            rate_low,
            npv_rate_low,
            rate_high,
            npv_rate_high,
            iteration_limit,
            iterations_run,
            irr,
            npv,
            is_valid,
        }
    }

    pub fn rate_low(&self) -> T {
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

    pub fn get_irr(&self) -> T {
        self.irr
    }

    pub fn get_npv(&self) -> T {
        self.npv
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}

impl<T> Debug for Irr<T>
where
    T: Float + Product<T> + Sum<T> + Signed + Debug + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("Irr")
            .field("rate_low", &self.rate_low())
            .field("npv_rate_low", &self.get_npv_rate_low())
            .field("rate_high", &self.get_rate_high())
            .field("npv_rate_high", &self.get_npv_rate_high())
            .field("iteration_limit", &self.get_irr())
            .field("iterations_run", &self.get_npv())
            .field("irr", &self.get_irr())
            .field("npv", &self.get_npv())
            .field("is_valid", &self.is_valid())
            .finish()
    }
}

impl<T> Display for Irr<T>
where
    T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "rate_low: {}\nnpv_rate_low: {}\nrate_high: {}\nnpv_rate_high: {}\niteration_limit: {}\n iterations_run: {}\nirr: {}\n npv: {}\nis_valid: {}\n",
            self.rate_low(),
            self.get_npv_rate_low(),
            self.get_rate_high(),
            self.get_npv_rate_high(),
            self.get_iteration_limit(),
            self.get_iterations_run(),
            self.get_irr(),
            self.get_npv(),
            self.is_valid()
        )
    }
}

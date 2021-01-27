//! Calculate the IRR of a series of cash flows with the bisection method.

use num::{abs, Float, Signed, Zero};
use std::iter::{Product, Sum};

use crate::present_value::from_cash_flows_and_discount_rate as pv;
use std::fmt::{Debug, Display, Error, Formatter};
use std::slice::Iter;

/// Contains information useful to finding the IRR of a given cash flow series.
///
/// # Comments
/// Designed so that functions for determining acceptable initial bounds for the bisection function can be more easily developed.
pub struct IrrApproximation<T>
    where
        T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    rate_guess_one: T,
    npv_guess_one: T,
    rate_guess_two: T,
    npv_guess_two: T,
    iteration_limit: i16,
    iterations_run: i16,
    irr_approximation: T,
    npv_approximation: T,
    is_valid: bool,
}

impl<T> Debug for IrrApproximation<T>
    where
        T: Float + Product<T> + Sum<T> + Signed + Debug + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("IrrApproximation")
            .field("rate_guess_one", &self.get_rate_guess_one())
            .field("npv_guess_one", &self.get_npv_guess_one())
            .field("rate_guess_two", &self.get_rate_guess_two())
            .field("npv_guess_two", &self.get_npv_guess_two())
            .field("iteration_limit", &self.get_irr_approximation())
            .field("iterations_run", &self.get_npv_approximation())
            .field("irr_approximation", &self.get_irr_approximation())
            .field("npv_approximation", &self.get_npv_approximation())
            .field("is_valid", &self.is_valid())
            .finish()
    }
}

impl<T> Display for IrrApproximation<T>
    where
        T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "rate_guess_one: {}\nnpv_guess_one: {}\nrate_guess_two: {}\nnpv_guess_two: {}\niteration_limit: {}\n iterations_run: {}\nirr_approximation: {}\n npv_approximation: {}\nis_valid: {}\n",
            self.get_rate_guess_one(),
            self.get_npv_guess_one(),
            self.get_rate_guess_two(),
            self.get_npv_guess_two(),
            self.get_iteration_limit(),
            self.get_iterations_run(),
            self.get_irr_approximation(),
            self.get_npv_approximation(),
            self.is_valid()
        )
    }
}

impl<T> IrrApproximation<T>
    where
        T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    pub fn get_rate_guess_one(&self) -> T {
        self.rate_guess_one
    }

    pub fn get_npv_guess_one(&self) -> T {
        self.npv_guess_one
    }

    pub fn get_rate_guess_two(&self) -> T {
        self.rate_guess_two
    }

    pub fn get_npv_guess_two(&self) -> T {
        self.npv_guess_two
    }

    pub fn get_iteration_limit(&self) -> i16 {
        self.iteration_limit
    }

    pub fn get_iterations_run(&self) -> i16 {
        self.iterations_run
    }

    pub fn get_irr_approximation(&self) -> T {
        self.irr_approximation
    }

    pub fn get_npv_approximation(&self) -> T {
        self.npv_approximation
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}

/// Calculates the mid point of two floating point numbers.
///
/// # Comments
/// Done in a way that avoids potential overflow that can occurs when using (a + b)/2
///
/// # Example with f32
/// ```
/// use time_value::irr::bisection::function::{calculate_mid_point, are_equal_enough};
///
/// let a: f32 = 1.0;
/// let b: f32 = 2.0;
/// let mid_point: f32 = calculate_mid_point(&a, &b);
/// assert!(are_equal_enough(&mid_point, &1.5));
/// ```
pub fn calculate_mid_point<T>(a: &T, c: &T) -> T
    where
        T: Float + Product<T> + Sum<T> + Signed,
{
    *a + (*c - *a) / T::from(2.0).unwrap()
}

/// Determines if two floating point numbers are "equal enough" based on machine epsilon
///
/// # Comments
/// Hat tip to [Bruce Dawson](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
///
/// # Example with f32
/// ```
/// use time_value::irr::bisection::function::are_equal_enough;
///
/// let a: f32 = 0.0010;
/// let b: f32 = 0.0010;
/// assert!(are_equal_enough(&a, &b));
///
/// let c: f32 = 0.0011;
/// assert!(!are_equal_enough(&a, &c));
/// ```
pub fn are_equal_enough<T>(a: &T, c: &T) -> bool
    where
        T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    let difference: T = abs(*a - *c);
    let a_abs: T = abs(*a);
    let c_abs: T = abs(*c);
    let larger: T = if a_abs < c_abs { c_abs } else { a_abs };

    difference <= (larger * T::epsilon())
}

/// The level of precision we require for NPVs to zero.
///
/// NPVs will be calculated for some currency or other similarly represented medium of exchange, suggesting two decimals of precision are sufficient for our purposes.
pub const NPV_PRECISION: f32 = 0.001;

/// An implementation of the bisection root finding algorithm for calculating the IRR of a series of cash flows.
///
/// # Assumptions
/// It is assumed that the user has found two rates such that their respective NPVs have values of opposite signs i.e. `rates_a * rates_b < 0.0`
///
/// # Comments
/// A function for finding initial values may be added soon.
///
/// # Example with f32
/// ```
/// use time_value::irr::bisection::function::{IrrApproximation, NPV_PRECISION, bisection as irr};
///
/// let cash_flows: Vec<f32> = vec![-100.0, 50.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0,];
/// let rate_left: f32 = 0.05;
/// let rate_right: f32 = 0.18;
/// let iteration_limit: i16 = 100;
/// let calculated_irr: IrrApproximation<f32> = irr(cash_flows.iter(), &rate_left, &rate_right, &iteration_limit);
/// assert!(calculated_irr.is_valid());
/// assert!(calculated_irr.get_npv_approximation() <= NPV_PRECISION);
/// ```
///
/// # Example with f64
/// ```
/// use time_value::irr::bisection::function::{IrrApproximation, NPV_PRECISION, bisection as irr};
///
/// let cash_flows: Vec<f64> = vec![-122.3990963, 24.26782424, -18.61877741, -2.555946884, -8.814622596, 32.05035057, 12.11973328, 7.743486592, 9.158469173, -21.97032692, 11.18895709];
/// let rate_left: f64 = -0.25;
/// let rate_right: f64 = 0.25;
/// let iteration_limit: i16 = 100;
/// let calculated_irr: IrrApproximation<f64> = irr(cash_flows.iter(), &rate_left, &rate_right, &iteration_limit);
/// assert!(calculated_irr.is_valid());
/// assert!(calculated_irr.get_npv_approximation() <= f64::from(NPV_PRECISION));
/// ```
pub fn bisection<T>(
    cash_flows: Iter<T>,
    rate_a: &T,
    rate_c: &T,
    iteration_limit: &i16,
) -> IrrApproximation<T>
    where
        T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    let mut rate_a: T = *rate_a;
    let mut rate_c: T = *rate_c;

    let mut npv_a: T = pv(cash_flows.clone(), &rate_a);
    let mut npv_c: T = pv(cash_flows.clone(), &rate_c);
    if T::zero() < npv_a * npv_c {
        return IrrApproximation {
            rate_guess_one: rate_a,
            npv_guess_one: npv_a,
            rate_guess_two: rate_c,
            npv_guess_two: npv_c,
            iteration_limit: *iteration_limit,
            iterations_run: 0,
            irr_approximation: T::nan(),
            npv_approximation: T::nan(),
            is_valid: false,
        };
    }

    let mut rate_b: T = calculate_mid_point(&rate_a, &rate_c);
    let mut npv_b: T = pv(cash_flows.clone(), &rate_b);
    let mut iterations_run: i16 = 0;
    let precision: T = T::from(NPV_PRECISION).unwrap();

    while iterations_run < *iteration_limit && !are_equal_enough(&precision, &npv_b) {
        iterations_run = iterations_run + 1;

        if npv_a * npv_b < T::zero() {
            rate_c = rate_b;
            npv_c = npv_b;
        } else {
            rate_a = rate_b;
            npv_a = npv_b;
        }

        rate_b = calculate_mid_point(&rate_a, &rate_c);
        npv_b = pv(cash_flows.clone(), &rate_b);
    }

    IrrApproximation {
        rate_guess_one: rate_a,
        npv_guess_one: npv_a,
        rate_guess_two: rate_c,
        npv_guess_two: npv_c,
        iteration_limit: *iteration_limit,
        iterations_run,
        irr_approximation: rate_b,
        npv_approximation: npv_b,
        is_valid: abs(npv_b) <= T::from(NPV_PRECISION).unwrap(),
    }
}


#[cfg(test)]
mod bisection_tests {
    use crate::irr::bisection::function::{bisection as irr, IrrApproximation, NPV_PRECISION};
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
        // right now most of these will fail as the guesses are generally off
        // once a range finding function is added, I'll do random value tests that will attempt or not and test on that
        let mut thread_range: ThreadRng = thread_rng();
        let vector_size: i16 = 20;
        let rate_a: f32 = -100.0;
        let rate_c: f32 = 100.0;
        let iteration_limit: i16 = 1_000;
        let mut cash_flows: Vec<f32> = generate_random_cash_flows(&mut thread_range, &vector_size);

        for _ in 0..100 {
            let calculated_irr: IrrApproximation<f32> =
                irr(cash_flows.iter(), &rate_a, &rate_c, &iteration_limit);

            match calculated_irr.is_valid() {
                true => assert!(calculated_irr.get_npv_approximation() <= NPV_PRECISION),
                false => match calculated_irr.get_iterations_run() {
                    0 => assert!(calculated_irr.get_irr_approximation().is_nan()),
                    _ => assert_eq!(calculated_irr.get_iterations_run(), iteration_limit),
                },
            }
            cash_flows = generate_random_cash_flows(&mut thread_range, &vector_size);
        }
    }

    #[test]
    fn it_works_on_known_example_0_f32() {
        let cash_flows: Vec<f32> = vec![
            -100.0, 50.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0,
        ];
        let rate_left: f32 = 0.05;
        let rate_right: f32 = 0.18;
        let iteration_limit: i16 = 100;
        let irr_approximation: IrrApproximation<f32> =
            irr(cash_flows.iter(), &rate_left, &rate_right, &iteration_limit);

        assert!(irr_approximation.is_valid());
        assert!(irr_approximation.get_npv_approximation() <= NPV_PRECISION);
    }

    #[test]
    fn it_works_on_known_example_0_f64() {
        let cash_flows: Vec<f64> = vec![
            -100.0, 50.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0,
        ];
        let rate_left: f64 = 0.05;
        let rate_right: f64 = 0.18;
        let iteration_limit: i16 = 100;
        let calculated_irr: IrrApproximation<f64> =
            irr(cash_flows.iter(), &rate_left, &rate_right, &iteration_limit);

        assert!(calculated_irr.is_valid());
        assert!(calculated_irr.get_npv_approximation() <= f64::from(NPV_PRECISION));
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
        let rate_left: f32 = 0.01;
        let rate_right: f32 = 0.05;
        let iteration_limit: i16 = 100;
        let calculated_irr: IrrApproximation<f32> =
            irr(cash_flows.iter(), &rate_left, &rate_right, &iteration_limit);

        assert!(calculated_irr.is_valid());
        assert!(calculated_irr.get_npv_approximation() <= NPV_PRECISION);
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
        let rate_left: f64 = 0.01;
        let rate_right: f64 = 0.05;
        let iteration_limit: i16 = 100;
        let calculated_irr: IrrApproximation<f64> =
            irr(cash_flows.iter(), &rate_left, &rate_right, &iteration_limit);

        assert!(calculated_irr.is_valid());
        assert!(calculated_irr.get_npv_approximation() <= f64::from(NPV_PRECISION));
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
        let rate_left: f32 = -0.25;
        let rate_right: f32 = 0.25;
        let iteration_limit: i16 = 100;
        let calculated_irr: IrrApproximation<f32> =
            irr(cash_flows.iter(), &rate_left, &rate_right, &iteration_limit);

        assert!(calculated_irr.is_valid());
        assert!(calculated_irr.get_npv_approximation() <= NPV_PRECISION);
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
        let rate_left: f64 = -0.25;
        let rate_right: f64 = 0.25;
        let iteration_limit: i16 = 100;
        let calculated_irr: IrrApproximation<f64> =
            irr(cash_flows.iter(), &rate_left, &rate_right, &iteration_limit);

        assert!(calculated_irr.is_valid());
        assert!(calculated_irr.get_npv_approximation() <= f64::from(NPV_PRECISION));
    }
}

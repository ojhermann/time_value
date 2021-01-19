use num::{abs, Float, Signed};
use std::iter::{Product, Sum};

use crate::present_value::from_cash_flows_and_discount_rate as pv;
use std::fmt::{Debug, Display, Error, Formatter};
use std::slice::Iter;

pub struct IrrApproximation<T>
where
    T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    rate_a: T,
    npv_a: T,
    rate_c: T,
    npv_c: T,
    iteration_limit: i16,
    iterations_run: i16,
    irr: T,
    npv: T,
    is_valid: bool,
}

impl<T> Debug for IrrApproximation<T>
where
    T: Float + Product<T> + Sum<T> + Signed + Debug + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("IrrApproximation")
            .field("rate_a", &self.get_rate_a())
            .field("npv_a", &self.get_npv_a())
            .field("rate_c", &self.get_rate_c())
            .field("npv_c", &self.get_npv_c())
            .field("iteration_limit", &self.get_irr())
            .field("iterations_run", &self.get_npv())
            .field("irr", &self.get_irr())
            .field("npv", &self.get_npv())
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
            "rate_a: {}\nnpv_a: {}\nrate_c: {}\nnpv_c: {}\niteration_limit: {}\n iterations_run: {}\nirr: {}\n npv: {}\n",
            self.get_rate_a(),
            self.get_npv_a(),
            self.get_rate_c(),
            self.get_npv_c(),
            self.get_iteration_limit(),
            self.get_iterations_run(),
            self.get_irr(),
            self.get_npv()
        )
    }
}

impl<T> IrrApproximation<T>
where
    T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    pub fn get_rate_a(&self) -> T {
        self.rate_a
    }

    pub fn get_npv_a(&self) -> T {
        self.npv_a
    }

    pub fn get_rate_c(&self) -> T {
        self.rate_c
    }

    pub fn get_npv_c(&self) -> T {
        self.npv_c
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

fn calculate_mid_point<T>(a: &T, c: &T) -> T
where
    T: Float + Product<T> + Sum<T> + Signed,
{
    // this avoids possible overflow of (a+b)/2 e.g. a, b, both close to overflow, so their sum overflows
    *a + (*c - *a) / T::from(2.0).unwrap()
}

fn are_equal_enough<T>(a: &T, c: &T) -> bool
where
    T: Float + Product<T> + Sum<T> + Signed + Display + Debug,
{
    // https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
    let difference: T = abs(*a - *c);
    let a_abs: T = abs(*a);
    let c_abs: T = abs(*c);
    let larger: T = if a_abs < c_abs { c_abs } else { a_abs };

    difference <= (larger * T::epsilon())
}

// NPVs will be calculated to some currency or other similarly represented medium of exchange
// This suggests that two decimals of precision are sufficient for our purposes
const PRECISION: f32 = 0.001;

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
            rate_a,
            npv_a,
            rate_c,
            npv_c,
            iteration_limit: *iteration_limit,
            iterations_run: 0,
            irr: T::nan(),
            npv: T::nan(),
            is_valid: false,
        };
    }

    let mut rate_b: T = calculate_mid_point(&rate_a, &rate_c);
    let mut npv_b: T = pv(cash_flows.clone(), &rate_b);
    let mut iterations_run: i16 = 0;
    let precision: T = T::from(PRECISION).unwrap();

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
        rate_a,
        npv_a,
        rate_c,
        npv_c,
        iteration_limit: *iteration_limit,
        iterations_run,
        irr: rate_b,
        npv: npv_b,
        is_valid: are_equal_enough(&precision, &npv_b),
    }
}

#[cfg(test)]
mod bisection_tests {
    use crate::irr::{bisection as irr, IrrApproximation, PRECISION};
    use num::{Float, Signed};
    use rand::prelude::ThreadRng;
    use rand::{thread_rng, Rng};
    use std::fmt::{Debug, Display};
    use std::iter::{Product, Sum};
    use rand::distributions::uniform::SampleUniform;

    fn generate_random_cash_flows<T>(thread_range: &mut ThreadRng, vector_size: &i16) -> Vec<T>
    where
        T: Float + Product<T> + Sum<T> + Signed + Display + Debug + SampleUniform,
    {
        //ensure the first element is negative
        let mut cash_flows: Vec<T> = vec![thread_range.gen_range(T::from(-100.0).unwrap()..T::from(-1.0).unwrap())];
        for _ in 0..(vector_size - 1) {
            cash_flows.push(thread_range.gen_range(T::from(-50.0).unwrap()..T::from(50.0).unwrap()));
        }
        cash_flows
    }

    #[test]
    fn it_works_with_random_inputs() {
        let mut thread_range: ThreadRng = thread_rng();
        let vector_size: i16 = 20;
        let rate_a: f32 = -100.0;
        let rate_c: f32 = 100.0;
        let iteration_limit: i16 = 1_000;
        let mut cash_flows: Vec<f32> = generate_random_cash_flows(&mut thread_range, &vector_size);

        for _ in 0..100 {
            let calculated_irr: IrrApproximation<f32> =
                irr(cash_flows.iter(), &rate_a, &rate_c, &iteration_limit);

            match calculated_irr.is_valid {
                true => assert!(calculated_irr.get_npv() <= PRECISION),
                false => match calculated_irr.get_iterations_run() {
                    0 => assert!(calculated_irr.get_irr().is_nan()),
                    _ => assert_eq!(calculated_irr.get_iterations_run(), iteration_limit)
                }
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
        let calculated_irr: IrrApproximation<f32> =
            irr(cash_flows.iter(), &rate_left, &rate_right, &iteration_limit);

        assert!(!calculated_irr.get_irr().is_nan());
        assert!(calculated_irr.get_npv() <= PRECISION);
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

        assert!(!calculated_irr.get_irr().is_nan());
        assert!(calculated_irr.get_npv() <= f64::from(PRECISION));
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

        assert!(!calculated_irr.get_irr().is_nan());
        assert!(calculated_irr.get_npv() <= PRECISION);
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

        assert!(!calculated_irr.get_irr().is_nan());
        assert!(calculated_irr.get_npv() <= f64::from(PRECISION));
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

        assert!(!calculated_irr.get_irr().is_nan());
        assert!(calculated_irr.get_npv() <= PRECISION);
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

        assert!(!calculated_irr.get_irr().is_nan());
        assert!(calculated_irr.get_npv() <= f64::from(PRECISION));
    }
}

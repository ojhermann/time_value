use num::Float;
use std::iter::Product;
use std::slice::Iter;

#[allow(dead_code)]
pub fn future_value<T>(present_value: &T, rates: Iter<T>) -> T
where
    T: Float + Product<T>,
{
    *present_value * rates.map(|rate| T::one() + *rate).product::<T>()
}

#[cfg(test)]
mod f32_tests {
    use crate::future_value;

    #[test]
    fn it_works_with_rates() {
        let present_value: f32 = 10.0;
        let rates: Vec<f32> = vec![1.0, 2.0, 3.0];
        let expected_value: f32 = 240.0;
        let value: f32 = future_value::future_value(&present_value, rates.iter());
        assert_eq!(value, expected_value);
        assert_eq!(rates.len(), 3);
    }

    #[test]
    fn it_works_without_rates() {
        let present_value: f32 = 10.0;
        let rates: Vec<f32> = vec![];
        let value: f32 = future_value::future_value(&present_value, rates.iter());
        assert_eq!(value, present_value);
        assert_eq!(rates.len(), 0);
    }
}

#[cfg(test)]
mod f64_tests {
    use crate::future_value;

    #[test]
    fn it_works() {
        let present_value: f64 = 10.0;
        let rates: Vec<f64> = vec![1.0, 2.0, 3.0];
        let expected_value: f64 = 240.0;
        let value: f64 = future_value::future_value(&present_value, rates.iter());
        assert_eq!(value, expected_value);
        assert_eq!(rates.len(), 3);
    }

    #[test]
    fn it_works_without_rates() {
        let present_value: f64 = 10.0;
        let rates: Vec<f64> = vec![];
        let value: f64 = future_value::future_value(&present_value, rates.iter());
        assert_eq!(value, present_value);
        assert_eq!(rates.len(), 0);
    }
}

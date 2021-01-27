//! A collection of functions related to time value analysis.

pub mod future_value;
pub mod irr {
    //! Functions and structs for calculating the internal rate of return (IRR) of a series of cash flows.
    pub mod bisection {
        //! Functions and structs for calculating the IRR of a series of cash flows using the bisection method.
        pub mod initial_bounds;
        pub mod function;
    }
}
pub mod present_value;

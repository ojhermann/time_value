//! Functions and structs related to time value analysis

pub mod future_value;

pub mod irr {
    //! Functions and structs for calculating the internal rate of return (IRR) of a series of cash flows

    pub mod bisection {
        //! Items related to the bisection method

        pub mod constants;

        pub mod functions {
            //! Functions used for the bisection method (and related methods)

            pub mod are_equal_enough;
            pub mod initial_bounds;
            pub mod irr;
            pub mod midpoint;
        }

        pub mod structs {
            //! Structs used with the bisection method

            pub mod initial_bounds;
            pub mod irr;
        }
    }
}

pub mod present_value;

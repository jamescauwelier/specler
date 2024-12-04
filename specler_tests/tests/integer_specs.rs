#![allow(dead_code)]
pub(crate) mod common;

use specler::prelude::*;
use specler::specs::integer::{
    larger_or_equal_than, larger_than, smaller_or_equal_than, smaller_than,
};
use specler_macros::create_with_spec;

struct FirstNumberSpec;
impl ContainsSpec<isize> for FirstNumberSpec {
    fn spec() -> Spec<isize> {
        Require::<isize>::to()
            .be(larger_than(0))
            .be(smaller_than(100))
    }
}

#[derive(Debug)]
#[create_with_spec(FirstNumberSpec)]
struct FirstNumber(isize);

struct SecondNumberSpec;
impl ContainsSpec<i64> for SecondNumberSpec {
    fn spec() -> Spec<i64> {
        Require::<i64>::to()
            .be(larger_or_equal_than(0))
            .be(smaller_or_equal_than(100))
    }
}

#[derive(Debug)]
#[create_with_spec(SecondNumberSpec)]
struct SecondNumber(i64);

#[cfg(test)]
mod tests {
    mod first_number {
        mod test_invalid_inputs {
            use crate::*;

            verify_invalid_input!(empty_test, -1_isize, FirstNumber::create);
            verify_invalid_input!(too_small_test, 0_isize, FirstNumber::create);
            verify_invalid_input!(too_large_test, 100_isize, FirstNumber::create);
        }

        mod test_valid_inputs {
            use crate::*;

            verify_valid_input!(valid_test_1, 1_isize, FirstNumber::create);
            verify_valid_input!(valid_test_2, 99_isize, FirstNumber::create);
        }
    }

    mod second_number {
        mod test_invalid_inputs {
            use crate::*;

            verify_invalid_input!(empty_test, -1, SecondNumber::create);
            verify_invalid_input!(too_large_test, 101, SecondNumber::create);
        }

        mod test_valid_inputs {
            use crate::*;

            verify_valid_input!(valid_test_1, 0, SecondNumber::create);
            verify_valid_input!(valid_test_2, 100, SecondNumber::create);
        }
    }
}

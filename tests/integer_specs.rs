#[allow(dead_code)]
pub(crate) mod common;

use specler::core::require::Require;
use specler::core::spec::contains_spec::ContainsSpec;
use specler::core::spec::Spec;
use specler::core::spec_error::SpecError;
use specler::core::validated::Validated;
use specler::specs::integer::{larger_or_equal_than, larger_than, smaller_or_equal_than, smaller_than, Integer};

struct FirstNumberSpec;
impl ContainsSpec<isize> for FirstNumberSpec {
    fn spec() -> Spec<isize> {
        Require::<isize>::to()
            .be(larger_than(0))
            .be(smaller_than(100))
    }
}

#[derive(Debug)]
struct FirstNumber(isize);
impl FirstNumber {
    fn create(
        pre_validated_input: impl Into<Validated<isize, FirstNumberSpec>>,
    ) -> Result<FirstNumber, SpecError> {
        match pre_validated_input.into() {
            Validated::Valid { value, _spec } => Ok(FirstNumber(value)),
            Validated::Invalid { error, _spec } => Err(error),
        }
    }
}

struct SecondNumberSpec;
impl ContainsSpec<i64> for SecondNumberSpec {
    fn spec() -> Spec<i64> {
        Require::<i64>::to()
            .be(larger_or_equal_than(0))
            .be(smaller_or_equal_than(100))
    }
}

#[derive(Debug)]
struct SecondNumber(i64);
impl SecondNumber {
    fn create(
        pre_validated_input: impl Into<Validated<i64, SecondNumberSpec>>,
    ) -> Result<SecondNumber, SpecError> {
        match pre_validated_input.into() {
            Validated::Valid { value, _spec } => Ok(SecondNumber(value)),
            Validated::Invalid { error, _spec } => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod first_number {
        mod test_invalid_inputs {
            use crate::{verify_invalid_input, FirstNumber};

            verify_invalid_input!(empty_test, -1, FirstNumber::create);
            verify_invalid_input!(too_small_test, 0, FirstNumber::create);
            verify_invalid_input!(too_large_test, 100, FirstNumber::create);
        }

        mod test_valid_inputs {
            use crate::{verify_valid_input, FirstNumber};

            verify_valid_input!(valid_test_1, 1, FirstNumber::create);
            verify_valid_input!(valid_test_2, 99, FirstNumber::create);
        }
    }

    mod second_number {
        mod test_invalid_inputs {
            use crate::{verify_invalid_input, SecondNumber};

            verify_invalid_input!(empty_test, -1, SecondNumber::create);
            verify_invalid_input!(too_large_test, 101, SecondNumber::create);
        }

        mod test_valid_inputs {
            use crate::{verify_valid_input, SecondNumber};

            verify_valid_input!(valid_test_1, 0, SecondNumber::create);
            verify_valid_input!(valid_test_2, 100, SecondNumber::create);
        }
    }

}
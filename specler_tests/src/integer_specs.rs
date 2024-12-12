#![allow(dead_code)]

use specler::prelude::*;
use specler::specs::integer::{
    larger_or_equal_than, larger_than, smaller_or_equal_than, smaller_than,
};
use specler_macros::value_object;

#[cfg(test)]
use specler_arbitrary::prelude::*;

struct FirstNumberSpec;
impl ContainsSpec<usize> for FirstNumberSpec {
    fn spec() -> Spec<usize> {
        Require::<usize>::to()
            .be(larger_than(0))
            .be(smaller_than(100))
    }
}

#[cfg(test)]
use proptest::prelude::*;

#[cfg(test)]
impl ArbitraryValidSpecValue<usize> for FirstNumberSpec {
    fn any_valid_value() -> BoxedStrategy<usize> {
        (1_usize..100_usize).boxed()
    }
}

#[cfg(test)]
impl ArbitraryInvalidSpecValue<usize> for FirstNumberSpec {
    fn any_invalid_value() -> BoxedStrategy<usize> {
        (100_usize..=usize::MAX).boxed()
    }
}

#[derive(Debug)]
#[value_object(FirstNumberSpec)]
struct FirstNumber(usize);

#[cfg(test)]
impl_arbitrary!(FirstNumber, FirstNumberSpec);

struct SecondNumberSpec;
impl ContainsSpec<u64> for SecondNumberSpec {
    fn spec() -> Spec<u64> {
        Require::<u64>::to()
            .be(larger_or_equal_than(0))
            .be(smaller_or_equal_than(100))
    }
}

#[cfg(test)]
impl ArbitraryValidSpecValue<u64> for SecondNumberSpec {
    fn any_valid_value() -> BoxedStrategy<u64> {
        (0_u64..=100_u64).boxed()
    }
}

#[cfg(test)]
impl ArbitraryInvalidSpecValue<u64> for SecondNumberSpec {
    fn any_invalid_value() -> BoxedStrategy<u64> {
        (100_u64..=u64::MAX).boxed()
    }
}

#[derive(Debug)]
#[value_object(SecondNumberSpec)]
struct SecondNumber(u64);

#[cfg(test)]
impl_arbitrary!(SecondNumber, SecondNumberSpec);

#[cfg(test)]
mod tests {
    use super::*;

    mod first_number {
        use super::*;
        use proptest::proptest;

        proptest! {
            #[test]
            fn can_be_created_using_valid_input(input in FirstNumberSpec::any_valid_value()) {
                let first_number = FirstNumber::create(input);
                assert!(first_number.is_ok());
            }

            #[test]
            fn cannot_be_created_using_invalid_input(input in FirstNumberSpec::any_invalid_value()) {
                let first_number = FirstNumber::create(input);
                assert!(!first_number.is_ok());
            }

            #[test]
            fn can_extract_value(input: FirstNumber) {
                assert!(input.0 > 0);
                assert!(input.0 < 100);
            }
        }
    }

    mod second_number {
        use crate::integer_specs::{SecondNumber, SecondNumberSpec};
        use proptest::proptest;
        use specler::prelude::ValueObjectFactory;
        use specler_arbitrary::{ArbitraryInvalidSpecValue, ArbitraryValidSpecValue};

        proptest! {
            #[test]
            fn can_be_created_using_valid_input(input in SecondNumberSpec::any_valid_value()) {
                let second_number = SecondNumber::create(input);
                assert!(second_number.is_ok());
            }

            #[test]
            fn cannot_be_created_using_invalid_input(input in SecondNumberSpec::any_invalid_value()) {
                let second_number = SecondNumber::create(input);
                assert!(!second_number.is_ok());
            }

            #[test]
            fn can_extract_value(input: SecondNumber) {
                assert!(input.0 <= 100);
            }
        }
    }
}

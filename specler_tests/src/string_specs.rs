#![allow(dead_code)]

use specler::prelude::*;
use specler::specs::string::{no_longer_than, no_shorter_than, not_empty};
use specler_macros::value_object;

#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
use specler_arbitrary::prelude::*;

#[derive(Debug)]
#[value_object(SomeStringTypeSpecs)]
struct SomeStringType(String);

struct SomeStringTypeSpecs;
impl<'a> SpecProvider<String> for SomeStringTypeSpecs {
    fn spec() -> Spec<String> {
        Require::<String>::to()
            .be(not_empty())
            .be(no_shorter_than(2))
            .be(no_longer_than(3))
    }
}

#[cfg(test)]
impl ArbitraryValidSpecValue<String> for SomeStringTypeSpecs {
    fn any_valid_value() -> BoxedStrategy<String> {
        ".{2, 3}"
            .prop_filter(
                "A regex generates strings with character length, while our code constrains on byte length, so an additional filter is needed",
                |s| s.len() > 1 && s.len() < 4
            )
            .boxed()
    }
}

#[cfg(test)]
impl ArbitraryInvalidSpecValue<String> for SomeStringTypeSpecs {
    fn any_invalid_value() -> BoxedStrategy<String> {
        ".|.{4,}"
            .prop_filter(
                "A regex generates strings with character length, while our code constrains on byte length, so an additional filter is needed",
                |s| s.len() >= 4
            )
            .boxed()
    }
}

#[cfg(test)]
impl_arbitrary!(SomeStringType, SomeStringTypeSpecs);

#[cfg(test)]
mod tests {
    use crate::string_specs::{SomeStringType, SomeStringTypeSpecs};
    use proptest::prelude::*;
    use specler::prelude::*;
    use specler_arbitrary::{ArbitraryInvalidSpecValue, ArbitraryValidSpecValue};

    proptest! {
        #[test]
        fn can_be_created_using_valid_input(s in SomeStringTypeSpecs::any_valid_value()) {
            let result = SomeStringType::create(s);
            assert!(result.is_ok());
        }

        #[test]
        fn cannot_be_created_using_invalid_input(s in SomeStringTypeSpecs::any_invalid_value()) {
            let result = SomeStringType::create(s);
            assert!(!result.is_ok());
        }

        #[test]
        fn can_extract_value(input: SomeStringType) {
            assert!(input.0.len() > 0);
            assert!(input.0.len() < 4);
        }
    }
}

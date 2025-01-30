#![allow(dead_code)]

use specler::prelude::*;
use specler::specs::string::{no_longer_than, no_shorter_than, not_empty};
use specler_macros::specled;

#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
use specler_arbitrary::prelude::*;

#[derive(Debug)]
#[specled(SomeStringTypeSpecs)]
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
impl SpecStrategies<String> for SomeStringTypeSpecs {
    fn valid_strategy() -> BoxedStrategy<String> {
        ".{2, 3}"
            .prop_filter(
                "A regex generates strings with character length, while our code constrains on byte length, so an additional filter is needed",
                |s| s.len() > 1 && s.len() < 4
            )
            .boxed()
    }

    fn invalid_strategy() -> BoxedStrategy<String> {
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
    use specler_arbitrary::SpecStrategies;

    proptest! {
        #[test]
        fn can_be_created_using_valid_input(s in SomeStringTypeSpecs::valid_strategy()) {
            let result = SomeStringType::create(s);
            assert!(result.is_ok());
        }

        #[test]
        fn cannot_be_created_using_invalid_input(s in SomeStringTypeSpecs::invalid_strategy()) {
            let result = SomeStringType::create(s);
            assert!(!result.is_ok());
        }

        #[test]
        fn is_a_value_object(input: SomeStringType) {
            input.value();
        }
    }
}

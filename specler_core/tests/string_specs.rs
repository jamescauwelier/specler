#![allow(dead_code)]
pub(crate) mod common;

use specler::core::require::Require;
use specler::core::spec::contains_spec::ContainsSpec;
use specler::core::spec::Spec;
use specler::core::spec_error::SpecError;
use specler::core::validated::Validated;
use specler::specs::string::{no_longer_than, no_shorter_than, not_empty};

#[derive(Debug)]
struct SomeStringType(String);

struct SomeStringTypeSpecs;
impl<'a> ContainsSpec<&'a str> for SomeStringTypeSpecs {
    fn spec() -> Spec<&'a str> {
        Require::<&str>::to()
            .be(not_empty())
            .be(no_shorter_than(2))
            .be(no_longer_than(3))
    }
}

impl SomeStringType {
    fn create<'a>(
        pre_validated_input: impl Into<Validated<&'a str, SomeStringTypeSpecs>>,
    ) -> Result<SomeStringType, SpecError> {
        match pre_validated_input.into() {
            Validated::Valid { value, _spec } => Ok(SomeStringType(value.into())),
            Validated::Invalid { error, _spec } => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    mod test_invalid_inputs {
        use crate::{verify_invalid_input, SomeStringType};

        verify_invalid_input!(empty_test, "", SomeStringType::create);
        verify_invalid_input!(too_small_test, "a", SomeStringType::create);
        verify_invalid_input!(too_large_test, "abab", SomeStringType::create);
    }
    mod test_valid_inputs {
        use crate::{verify_valid_input, SomeStringType};

        verify_valid_input!(example_1, "ab", SomeStringType::create);
        verify_valid_input!(example_2, "abc", SomeStringType::create);
    }
}

#![allow(dead_code)]
pub(crate) mod common;

use specler::prelude::*;
use specler::specs::string::{an_email, no_longer_than, no_shorter_than, not_empty};
use specler_macros::create_with_spec;

#[derive(Debug)]
#[create_with_spec(SomeStringTypeSpecs)]
struct SomeStringType(String);

struct SomeStringTypeSpecs;
impl<'a> ContainsSpec<String> for SomeStringTypeSpecs {
    fn spec() -> Spec<String> {
        Require::<String>::to()
            .be(not_empty())
            .be(no_shorter_than(2))
            .be(no_longer_than(3))
    }
}

#[cfg(test)]
mod tests {
    mod test_invalid_inputs {
        use crate::*;

        verify_invalid_input!(empty_test, "", SomeStringType::create);
        verify_invalid_input!(too_small_test, "a", SomeStringType::create);
        verify_invalid_input!(too_large_test, "abab", SomeStringType::create);
    }
    mod test_valid_inputs {
        use crate::*;

        verify_valid_input!(example_1, "ab", SomeStringType::create);
        verify_valid_input!(example_2, "abc", SomeStringType::create);
    }
}

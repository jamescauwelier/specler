#![allow(dead_code)]

use specler::prelude::*;
use specler::specs::string::{no_longer_than, no_shorter_than, not_empty};
use specler_macros::value_object;

#[derive(Debug)]
#[value_object(SomeStringTypeSpecs)]
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
        use specler::prelude::ValueObjectFactory;
        use crate::*;
        use crate::string_specs::SomeStringType;

        verify_invalid_input!(empty_test, "", SomeStringType::create);
        verify_invalid_input!(too_small_test, "a", SomeStringType::create);
        verify_invalid_input!(too_large_test, "abab", SomeStringType::create);
    }
    mod test_valid_inputs {
        use specler::prelude::ValueObjectFactory;
        use crate::*;
        use crate::string_specs::SomeStringType;

        verify_valid_input!(example_1, "ab", SomeStringType::create);
        verify_valid_input!(example_2, "abc", SomeStringType::create);
    }
}

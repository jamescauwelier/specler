use std::fmt::Display;
use crate::core::validator::validator_result::ValidatorResult;

/// # Integer trait
///
/// Provides the ability to develop generic code that covers
/// all known integer types. Without it a concrete validator
/// function would have to be duplicated for every integer
/// type.
pub trait Integer: Copy + PartialOrd + PartialEq + Display {}
impl Integer for i8 {}
impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for i128 {}
impl Integer for isize {}
impl Integer for u8 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}
impl Integer for u128 {}
impl Integer for usize {}

/// Validator to check whether an input is larger or equal than a given x
pub fn larger_or_equal_than<T: Integer + 'static>(x: T) -> Box<dyn Fn(T) -> ValidatorResult> {
    Box::new(
        move |input: T| {
            if input >= x {
                ValidatorResult::valid()
            } else {
                ValidatorResult::invalid(&format!("cannot be smaller than {}", x))
            }
        }
    )
}

/// Validator to check whether an input is larger than a given x
pub fn larger_than<T: Integer + 'static>(x: T) -> Box<dyn Fn(T) -> ValidatorResult> {
    Box::new(
        move |input: T| {
            if input > x {
                ValidatorResult::valid()
            } else {
                ValidatorResult::invalid(&format!("cannot be smaller than {}", x))
            }
        }
    )
}

/// Validator to check whether an input is smaller or equal than x
pub fn smaller_or_equal_than<T: Integer + 'static>(x: T) -> Box<dyn Fn(T) -> ValidatorResult> {
    Box::new(
        move |input: T| {
            if input <= x {
                ValidatorResult::valid()
            } else {
                ValidatorResult::invalid(&format!("cannot be larger than {}", x))
            }
        }
    )
}

/// Validator to check whether an input is smaller than x
pub fn smaller_than<T: Integer + 'static>(x: T) -> Box<dyn Fn(T) -> ValidatorResult> {
    Box::new(
        move |input: T| {
            if input < x {
                ValidatorResult::valid()
            } else {
                ValidatorResult::invalid(&format!("cannot be larger than {}", x))
            }
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    mod smaller_than {
        #[test]
        fn test_valid() {
            let result = super::smaller_than(5)(4);
            assert!(result.is_valid());
        }

        #[test]
        fn test_invalid() {
            let result = super::smaller_than(5)(5);
            assert!(!result.is_valid());
        }
    }

    mod smaller_or_equal_than {
        #[test]
        fn test_valid() {
            let result = super::smaller_or_equal_than(5)(5);
            assert!(result.is_valid());
        }

        #[test]
        fn test_invalid() {
            let result = super::smaller_or_equal_than(5)(6);
            assert!(!result.is_valid());
        }
    }

    mod larger_than {
        #[test]
        fn test_valid() {
            let result = super::larger_than(5)(6);
            assert!(result.is_valid());
        }

        #[test]
        fn test_invalid() {
            let result = super::larger_than(5)(5);
            assert!(!result.is_valid());
        }
    }

    mod larger_or_equal_than {
        #[test]
        fn test_valid() {
            let result = super::larger_or_equal_than(5)(5);
            assert!(result.is_valid());
        }

        #[test]
        fn test_invalid() {
            let result = super::larger_or_equal_than(5)(4);
            assert!(!result.is_valid());
        }
    }
}
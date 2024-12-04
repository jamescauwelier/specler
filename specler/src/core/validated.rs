use std::marker::PhantomData;
use crate::core::spec::contains_spec::ContainsSpec;
use crate::core::spec_error::SpecError;

/// # Validated
///
/// Enum documenting the result of running the provided spec
/// against a struct with a single value.
pub enum Validated<T,S>
where
    T: Clone,
    S: ContainsSpec<T>
{
    /// Marks a validation output as valid based on the spec
    Valid {
        /// The valid value that passed the spec validation
        value: T,
        /// The spec that was used to validate the input
        _spec: PhantomData<S>
    },
    /// Marks a validation output as invalid based on the spec
    Invalid {
        /// The error describing why the input data was invalid
        error: SpecError,
        /// The spec that was used to invalidate the input
        _spec: PhantomData<S>
    }
}

impl <T, U> From<T> for Validated<T, U>
where
    T: Clone,
    U: ContainsSpec<T>
{
    fn from(value: T) -> Validated<T, U> {
        let spec = U::spec();
        match spec.validate(value) {
            Ok(v) => Validated::Valid {
                value: v,
                _spec: PhantomData
            },
            Err(e) => Validated::Invalid {
                error: e,
                _spec: PhantomData
            }
        }
    }
}
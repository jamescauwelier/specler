/// Specification helper module
pub mod spec_validation_result;

/// Module: defines the trait to provide specs in structs
pub mod contains_spec;

use spec_validation_result::SpecValidationResult;
use crate::core::spec_error::SpecError;
use crate::core::validator::Validator;
use crate::core::validator::validator_result::ValidatorResult;

/// Encapsulates a collections of validation rules to apply to
/// an input of type `T`
pub struct Spec<T>(Vec<Validator<T>>);

impl <T: Clone> Spec<T> {
    /// Simple specification constructor
    pub fn new() -> Spec<T> {
        Spec(vec![])
    }

    /// Runs the specification againts an input and returns findings as a `SpecValidationResult`
    pub fn validate<U: Into<T>>(&self, input: U) -> Result<T, SpecError> {
        let input = input.into();
        self.0.iter().fold(
            SpecValidationResult::valid(input),
            | result, validator | {
                let validation_result = validator(result.value());
                match (result, validation_result) {
                    (current, ValidatorResult::Valid) => current,
                    (current, ValidatorResult::Invalid(reason)) => current.fail(reason)
                }
            }
        ).into()
    }

    /// Adds an additional validator to a specification
    pub fn be(mut self, validator: Validator<T>) -> Spec<T>
    {
        self.0.push(validator);

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_spec_error_msg};
    use crate::core::require::Require;
    use crate::specs::string::not_empty;

    #[test]
    fn validate_string_with_empty_spec() {
        let spec = Require::<String>::to();
        let result = spec.validate("");
        assert_eq!(result.unwrap().as_str(), "");
    }

    #[test]
    fn validate_string_to_not_be_empty() {
        let spec = Require::<String>::to()
            .be(not_empty());
        let result = spec.validate("");
        assert_spec_error_msg!(result, "cannot be empty");
    }
}
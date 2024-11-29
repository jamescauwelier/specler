mod spec_validator_result;

use spec_validator_result::SpecValidatorResult;
use crate::core::validator::Validator;
use crate::core::validator::validator_result::ValidatorResult;

pub struct Spec<T>(Vec<Validator<T>>);

impl <T> Spec<T> {
    pub fn new() -> Spec<T> {
        Spec(vec![])
    }

    pub fn validate<U: Into<T>>(&self, input: U) -> SpecValidatorResult<T> {
        let input = input.into();
        self.0.iter().fold(
            SpecValidatorResult::valid(input),
            | result, validator | {
                let validation_result = validator(result.value());
                match (result, validation_result) {
                    (current, ValidatorResult::Valid) => current,
                    (current, ValidatorResult::Invalid(reason)) => current.fail(reason)
                }
            }
        )
    }

    pub fn be(mut self, validator: Validator<T>) -> Spec<T>
    {
        self.0.push(validator);

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_spec_valid, assert_spec_validation_error};
    use crate::core::require::Require;
    use crate::core::spec::spec_validator_result::SpecValidatorResult;
    use crate::specs::string::not_empty;

    #[test]
    fn validate_string_with_empty_spec() {
        let spec = Require::<String>::to();
        let result = spec.validate("");
        assert_spec_valid!(result);
    }

    #[test]
    fn validate_string_to_not_be_empty() {
        let spec = Require::<String>::to()
            .be(not_empty);
        let result = spec.validate("");
        assert_spec_validation_error!(result, "String cannot be empty");
    }
}
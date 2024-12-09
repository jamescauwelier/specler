use crate::core::spec_error::SpecError;

/// Represents the result of a spec validation, which can either be valid or invalid.
/// This enum is separate from a single validation error because it allows for multiple reasons.
pub(crate) enum SpecValidationResult<T: Clone> {
    /// Signals that the spec validation was successful
    Valid(T),
    /// Signals that the spec validation failed with a list of reasons
    Invalid(T, Vec<String>)
}

impl <T: Clone> From<SpecValidationResult<T>> for Result<T, SpecError> {
    fn from(result: SpecValidationResult<T>) -> Result<T, SpecError> {
        match result {
            SpecValidationResult::Valid(value) => Ok(value),
            SpecValidationResult::Invalid(_, reasons) => Err(SpecError::unnamed(reasons))
        }
    }
}

impl <T: Clone> SpecValidationResult<T> {
    pub(crate) fn valid(t: T) -> SpecValidationResult<T> {
        SpecValidationResult::Valid(t)
    }

    pub(crate) fn invalid(t: T, reasons: Vec<String>) -> SpecValidationResult<T> {
        SpecValidationResult::Invalid(t, reasons)
    }

    pub(crate) fn fail(self, reason: String) -> SpecValidationResult<T> {
        match self {
            SpecValidationResult::Valid(v) => {
                SpecValidationResult::invalid(v, vec![reason.to_string()])
            },
            SpecValidationResult::Invalid(v, mut reasons) => {
                reasons.push(reason.to_string());
                SpecValidationResult::invalid(v, reasons)
            }
        }
    }

    pub(crate) fn value(&self) -> T {
        match self {
            SpecValidationResult::Valid(value) => value.clone(),
            SpecValidationResult::Invalid(value, _) => value.clone()
        }
    }


}
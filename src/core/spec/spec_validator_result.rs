use crate::core::spec_error::SpecError;

pub enum SpecValidatorResult<T> {
    Valid(T),
    Invalid(T, Vec<String>)
}

impl <T> From<SpecValidatorResult<T>> for Result<T, SpecError> {
    fn from(result: SpecValidatorResult<T>) -> Result<T, SpecError> {
        match result {
            SpecValidatorResult::Valid(value) => Ok(value),
            SpecValidatorResult::Invalid(_, reasons) => Err(SpecError::new(reasons))
        }
    }
}

impl <T> SpecValidatorResult<T> {
    pub(crate) fn valid(t: T) -> SpecValidatorResult<T> {
        SpecValidatorResult::Valid(t)
    }

    pub(crate) fn invalid(t: T, reasons: Vec<String>) -> SpecValidatorResult<T> {
        SpecValidatorResult::Invalid(t, reasons)
    }

    pub(crate) fn fail(self, reason: String) -> SpecValidatorResult<T> {
        match self {
            SpecValidatorResult::Valid(v) => {
                SpecValidatorResult::invalid(v,vec![reason.to_string()])
            },
            SpecValidatorResult::Invalid(v, mut reasons) => {
                reasons.push(reason.to_string());
                SpecValidatorResult::invalid(v, reasons)
            }
        }
    }

    pub(crate) fn value(&self) -> &T {
        match self {
            SpecValidatorResult::Valid(value) => value,
            SpecValidatorResult::Invalid(value, _) => value
        }
    }
}
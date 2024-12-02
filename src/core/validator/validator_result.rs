#[derive(Debug, PartialEq)]
pub enum ValidatorResult {
    Valid,
    Invalid(String)
}

impl ValidatorResult {
    pub fn invalid(message: &str) -> ValidatorResult {
        ValidatorResult::Invalid(message.to_string())
    }

    pub fn valid() -> ValidatorResult {
        ValidatorResult::Valid
    }

    #[cfg(test)]
    pub(crate) fn error_message(&self) -> &str {
        match self {
            ValidatorResult::Valid => panic!("Cannot get message from a valid result"),
            ValidatorResult::Invalid(message) => message
        }
    }
}
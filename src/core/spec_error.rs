use std::fmt;
use std::fmt::Formatter;
use thiserror::Error;

/// Collects the errors occurring during the validation of a specification
#[derive(Debug, Error)]
pub struct SpecError(Vec<String>);

impl SpecError {
    pub fn new(errors: Vec<String>) -> SpecError {
        SpecError(errors)
    }

    #[cfg(test)]
    pub fn contains(&self, error: &str) -> bool {
        self.0.contains(&error.to_string())
    }
}

impl fmt::Display for SpecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "SpecError: {:?}", self.0.join(", "))
    }
}

use std::fmt;
use std::fmt::Formatter;
use thiserror::Error;

/// Collects the errors occurring during the validation of a specification
#[derive(Debug, Error)]
pub struct SpecError(Vec<String>);

impl SpecError {
    /// Creates an empty new instance of `SpecError`
    pub fn new(errors: Vec<String>) -> SpecError {
        SpecError(errors)
    }

    /// Returns the list of errors that occurred during the validation of a specification
    pub fn errors(&self) -> &Vec<String> {
        &self.0
    }

    /// Enable easy testing of a set of error messages
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

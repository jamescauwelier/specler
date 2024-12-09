use std::fmt;
use std::fmt::Formatter;
use thiserror::Error;

/// Collects the errors occurring during the validation of a specification
#[derive(Debug, Error)]
pub enum SpecError {
    Unnamed {
        spec_violations: Vec<String>,
    },
    Named {
        name: String,
        spec_violations: Vec<String>,
    }
}

impl SpecError {
    /// Creates an empty new instance of `SpecError`
    pub fn unnamed(errors: Vec<String>) -> SpecError {
        SpecError::Unnamed { spec_violations: errors }
    }

    /// Returns the list of errors that occurred during the validation of a specification
    pub fn errors(&self) -> &Vec<String> {
        match self {
            SpecError::Unnamed { spec_violations } => spec_violations,
            SpecError::Named { spec_violations, .. } => spec_violations
        }
    }

    /// Enable easy testing of a set of error messages
    #[cfg(test)]
    pub fn contains(&self, error: &str) -> bool {
        self.errors().contains(&error.to_string())
    }
}

impl fmt::Display for SpecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "SpecError: {:?}", self.errors().join(", "))
    }
}

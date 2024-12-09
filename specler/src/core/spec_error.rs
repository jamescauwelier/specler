use std::fmt;
use std::fmt::Formatter;
use thiserror::Error;

/// Collects the errors occurring during the validation of a specification
#[derive(Debug, Error)]
pub enum SpecError {
    /// Specifies spec errors without knowing for what thing they occurred
    Unnamed {
        /// The list of errors that occurred during the validation of a specification
        spec_violations: Vec<String>,
    },
    /// Specifies spec errors with a name of what thing failed to pass a spec
    Named {
        /// The list of errors that occurred during the validation of a specification
        spec_violations: Vec<String>,
        /// The name of the spec that failed
        name: String,
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

    /// Returns whether the spec errors have been named,
    /// meaning whether we know not just how something
    /// failed, but also what it was that failed, e.g. a first name.
    pub fn is_named(&self) -> bool {
        match self {
            SpecError::Named { .. } => true,
            SpecError::Unnamed { .. } => false
        }
    }

    /// Names the spec error, meaning attaching a name of
    /// what thing failed to pass a spec
    pub fn named(self, spec_name: &str) -> SpecError {
        match self {
            SpecError::Unnamed { spec_violations } => {
                SpecError::Named { name: spec_name.to_string(), spec_violations }
            },
            SpecError::Named { spec_violations, .. } => {
                SpecError::Named { name: spec_name.to_string(), spec_violations }
            }
        }
    }
}

impl fmt::Display for SpecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SpecError::Unnamed { spec_violations } => {
                write!(f, "Spec failed: {}", spec_violations.join(", "))
            },
            SpecError::Named { name, spec_violations } => {
                write!(f, "'{}' spec failed: {}", name, spec_violations.join(", "))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_an_unnamed_spec_error() {
        let error = SpecError::unnamed(vec!["some error".into()]);
        assert!(!error.is_named());
        assert_eq!("Spec failed: some error", error.to_string());
    }

    #[test]
    fn create_a_named_spec_error() {
        let error = SpecError::unnamed(vec!["some error".into()]).named("something");
        assert!(error.is_named());
        assert_eq!("'something' spec failed: some error", error.to_string());
    }
}
pub(crate) mod validator;

/// A module with Spec helper code
pub mod spec;

/// Module for defining a fluent API for specifying validation rules
pub mod require;

/// Asserts to help in verifying validation results
pub mod asserts;
mod spec_error;
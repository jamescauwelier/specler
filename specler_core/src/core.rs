pub(crate) mod validator;

/// A module with Spec helper code
pub mod spec;

/// Module for defining a fluent API for specifying validation rules
pub mod require;

/// Asserts to help in verifying validation results
pub mod asserts;

/// Module for defining errors that can occur during validation.
/// This error is exposed and is the one to be used in consuming
/// code. Other errors are strictly for internal use and should
/// not be exposed.
pub mod spec_error;

/// Module to mark types as validated
pub mod validated;

/// Module to provide traits to define spec dependent factories
pub mod create_with_specification;
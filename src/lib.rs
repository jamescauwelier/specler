#![warn(missing_docs)]

//! # Specler
//!
//! A simple library for defining and validating specifications on your types.
//! This solves an issue with other validation approaches where
//! it is possible to first create a type and then verify its validity.
//! In disagreeing with this approach, a solution was needed to
//! verify specifications in factory methods.
//!
//! ## Concepts
//!
//! A specification defines a list of requirements that a type must meet.
//! These requirements are expressed as a list of validators, which are
//! simple functions returning a `ValidatorResult`. These then get
//! compiled into a single `SpecValidationResult`.
//!
//! ## Examples
//!
//! ### Validating using an empty spec
//!
//! ```
//! use specler::assert_spec_valid;
//! use crate::specler::core::require::Require;
//! use crate::specler::core::spec_error::SpecError;
//!
//! let spec = Require::<String>::to();
//! let result = spec.validate("");
//!
//! assert!(result.is_ok());
//! ```
//!
//! ### Validating a string to not be empty
//!
//! ```
//! use specler::{assert_spec_error_msg, assert_spec_invalid};
//! use specler::core::require::Require;
//! use specler::specs::string::not_empty;
//! use crate::specler::core::spec_error::SpecError;
//!
//! let spec = Require::<String>::to().be(not_empty);
//! let result = spec.validate("");
//!
//! assert!(result.is_err());
//! assert_spec_error_msg!(result, "cannot be empty");
//! ```

/// Core module containing core functionality, excluding concrete type specifications
pub mod core;

/// Module containing core specifications for standard types, e.g. String
pub mod specs;

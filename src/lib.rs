#![warn(missing_docs)]

//! Specler
//!
//! A simple library for defining and validating specifications on your types.
//! This solves an issue with other validation approaches where
//! it is possible to first create a type and then verify its validity.
//! In disagreeing with this approach, a solution was needed to
//! verify specifications in factory methods.

/// Core module containing core functionality, excluding concrete type specifications
pub mod core;

/// Module containing core specifications for standard types, e.g. String
pub mod specs;

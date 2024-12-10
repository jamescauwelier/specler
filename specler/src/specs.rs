/// Validator functions for `String` types
pub mod string;

/// Validator functions for types marked with the `Integer` trait
pub mod integer;

/// # Uuid module
///
/// Provides validator functions to validate both v4 and v7 UUIDs.
/// Other UUID versions are not supported as they are less
/// relevant to identifiers in modern web applications.
#[cfg(feature = "uuid")]
pub mod uuid;
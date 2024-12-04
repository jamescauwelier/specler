use crate::core::spec::Spec;

/// # ContainsSpec
///
/// Trait to provide a specification on a type `T`.
pub trait ContainsSpec<T> {
    /// Provides the spec
    fn spec() -> Spec<T>;
}